extern crate proc_macro;
use proc_macro::{TokenStream};
use syn::{DeriveInput, parse_macro_input, parse::Parse, Type, Ident, Token};
use quote::quote;

/// Makes a struct structural
#[proc_macro_derive(Struct)]
pub fn _struct(input: TokenStream) -> TokenStream {
   let input = parse_macro_input!(input as DeriveInput);

    let DeriveInput {  ident, data, .. } = &input;

    let fields = match &data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Can only be used on structs")
    };

    let attrs_quote = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            (stringify!(#name), stringify!(#ty))
        }
    });

    let fields_quote = fields.iter().map(|field| {
        let name_ident = field.ident.as_ref().unwrap();
        let name = format!("{}", name_ident);
        let ty = &field.ty;

        quote! {
            impl ::structural::HasAttr<#name> for #ident {
                type Ty = #ty;

                fn get(&self) -> &#ty {
                    &self.#name_ident
                }

                fn set(&mut self, value: #ty) {
                    self.#name_ident = value
                }

                fn take(self) -> #ty {
                    self.#name_ident
                }
            }
        }
    });

    let attrs_derive = quote! {
        impl ::structural::Attrs for #ident {
            const ATTRS: &'static [(&'static str, &'static str)] = &[#(#attrs_quote),*];
        }
    };

    let expanded = quote! {
        #attrs_derive
        #(#fields_quote)*
    };

   TokenStream::from(expanded)
}

struct Attr(Ident, Type);

impl Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse::<Type>()?;

        Ok(Self(name, ty))
    }
}

struct Attrs(Vec<Attr>);

impl Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();

        while let Ok(attr) = input.parse::<Attr>() {
            attrs.push(attr);

            if input.parse::<Token![,]>().is_err() {
                break;
            }
        };

        Ok(Self(attrs))
    }
}

#[proc_macro]
pub fn has_attrs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Attrs);

    let attrs = input.0.iter().map(|attr| {
        let Attr(k, v) = attr;
        let k = format!("{k}");

        quote! {
            ::structural::HasAttr<#k, Ty=#v>
        }
    }).collect::<Vec<_>>();

    println!("{:?}", attrs);

    let quotted = quote! {
        impl #(#attrs)+*
    };

    TokenStream::from(quotted)
}
