/// Define `Resolver` trait and implement it on some hashmap and also define `Resolver` tuple
/// composition. Provide also some utility functions related to how to create a `Resolver` and
/// resolving render.
///

use std::borrow::Cow;
use std::collections::HashMap;

use syn::{parse_quote, Stmt};
use proc_macro2::{Ident, Span};

use crate::parse::{CaseArg, Fixture};

pub(crate)  fn fixture_resolver<'a>(fixtures: impl Iterator<Item=&'a Fixture>) -> impl Resolver + 'a {
    fixtures.map(|f|
        ( f.name.to_string(), extract_resolve_expression(f).into() )
    ).collect::<HashMap<_, CaseArg>>()
}

/// A trait that `resolve` the given ident to expression code to assign the value.
pub(crate) trait Resolver {
    fn resolve(&self, ident: &Ident) -> Option<Cow<CaseArg>>;
}

pub(crate) fn arg_2_fixture(ident: &Ident, resolver: &impl Resolver) -> Stmt {
    let fixture = resolver
        .resolve(ident)
        .map(|e| e.clone())
        .unwrap_or_else(|| default_fixture_resolve(ident));
    parse_quote! {
        let #ident = #fixture;
    }
}

impl<'a> Resolver for HashMap<String, &'a CaseArg> {
    fn resolve(&self, ident: &Ident) -> Option<Cow<CaseArg>> {
        let ident = ident.to_string();
        self.get(&ident)
            .map(|&c| Cow::Borrowed(c) )
    }
}

impl<'a> Resolver for HashMap<String, CaseArg> {
    fn resolve(&self, ident: &Ident) -> Option<Cow<CaseArg>> {
        let ident = ident.to_string();
        self.get(&ident)
            .map(|c| Cow::Borrowed(c) )
    }
}

impl<R1: Resolver, R2: Resolver> Resolver for (R1, R2) {
    fn resolve(&self, ident: &Ident) -> Option<Cow<CaseArg>> {
        self.0.resolve(ident).or_else(|| self.1.resolve(ident))
    }
}

fn default_fixture_resolve(ident: &Ident) -> Cow<CaseArg> {
    Cow::Owned(parse_quote! { #ident::default() } )
}

fn extract_resolve_expression(fixture: &Fixture) -> syn::Expr {
    let name = &fixture.name;
    let positional= &fixture.positional;
    let pname = format!("partial_{}", positional.len());
    let partial = Ident::new(&pname, Span::call_site());
    parse_quote! { #name::#partial(#(#positional), *) }
}