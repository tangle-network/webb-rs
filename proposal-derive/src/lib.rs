//! This crate provides a derive macro to implement the `Proposal` trait.

use ethers_core::abi::HumanReadableParser;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_macro_input, DeriveInput, Fields, Generics, Ident, Lit, LitStr, Token,
};

/// Args for the proposal derive macro
struct Args {
    function_sig: LitStr,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let _: Token!(=) = input.parse()?;
        let function_sig: Lit = input.parse()?;
        if ident != "function_sig" {
            return Err(syn::Error::new(
                ident.span(),
                format!("expected `function_sig` but got {ident} instead!"),
            ));
        }
        let function_sig = match function_sig {
            Lit::Str(v) => v,
            unknown => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("expected a string literal but got `{unknown:?}` instead!"),
                ));
            }
        };
        let args = Self { function_sig };
        Ok(args)
    }
}

fn derive_proposal(input: DeriveInput) -> syn::Result<TokenStream> {
    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let generics = input.generics;
    let struct_fields = match input.data {
        syn::Data::Struct(s) => s.fields,
        _ => {
            return Err(syn::Error::new(
                name.span(),
                "expected the proposal to be a struct but got something else instead!",
            ))
        }
    };
    make_sure_it_has_header_field(&struct_fields)?;
    let common = impl_common_methods(&name, &generics, &struct_fields)?;
    let attr = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("proposal"))
        .ok_or_else( ||
            syn::Error::new(name.span(), r#"missing function signature! please add #[proposal(function_sig = "...")] on the proposal struct"#),
        )?;
    let args: Args = attr.parse_args()?;
    let trait_impl = derive_proposal_trait(name, generics, &args)?;
    let mut expanded = common;
    expanded.extend(trait_impl);
    Ok(expanded)
}

fn derive_proposal_trait(
    name: Ident,
    generics: Generics,
    args: &Args,
) -> syn::Result<TokenStream> {
    let function_sig = &args.function_sig;
    let sig = match HumanReadableParser::parse_function(&function_sig.value()) {
        Ok(f) => f.short_signature(),
        Err(e) => return Err(syn::Error::new(function_sig.span(), e)),
    };
    // Had to do this hack.
    let computed_function_sig = {
        let (s0, s1, s2, s3) = (sig[0], sig[1], sig[2], sig[3]);
        quote! { [#s0, #s1, #s2, #s3] }
    };
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #impl_generics crate::ProposalTrait for #name #ty_generics #where_clause {
            fn header(&self) -> crate::ProposalHeader {
                self.header
            }

            fn function_sig() -> crate::FunctionSignature {
                crate::FunctionSignature(#computed_function_sig)
            }

            fn to_vec(&self) -> Vec<u8> {
                crate::to_vec(self).expect("never fails to serialize")
            }
        }
    };

    Ok(expanded.into())
}

/// Makes sure of the following:
/// 1. that the proposal has the header field.
/// 2. the header field has the correct type.
/// 3. the header field is the first field in the struct.
fn make_sure_it_has_header_field(fields: &Fields) -> syn::Result<()> {
    let header_field = fields.iter().next().ok_or_else(|| {
        syn::Error::new_spanned(fields, "expected at least one field")
    })?;
    let header_field_name = header_field.ident.as_ref().ok_or_else(|| {
        syn::Error::new_spanned(
            header_field,
            "expected the first field to have a name",
        )
    })?;
    if header_field_name != "header" {
        return Err(syn::Error::new_spanned(
            header_field_name,
            "expected the first field to be named `header`",
        ));
    }
    let header_field_type = &header_field.ty;
    if !matches!(header_field_type, syn::Type::Path(p) if p.path.is_ident("ProposalHeader"))
    {
        return Err(syn::Error::new_spanned(
            header_field_type,
            "expected the first field to be of type `ProposalHeader`",
        ));
    }

    Ok(())
}

/// Implements the common methods for any proposal.
/// The common methods are:
/// - `LENGTH` const
/// - `new` function
/// - `header` getter
/// - `[field_name]` getter
fn impl_common_methods(
    name: &Ident,
    generics: &Generics,
    fields: &Fields,
) -> syn::Result<TokenStream> {
    // LENGTH const
    // 40 is the length of the proposal header.
    // + ANY additional field using `core::mem::size_of::<T>()`
    // so the final length is 40 + sum of all fields.
    let length = fields
        .iter()
        .skip(1)
        .map(|f| {
            let ty = &f.ty;
            quote! { core::mem::size_of::<#ty>() }
        })
        .fold(quote! { 40 }, |acc, f| quote! { #acc + #f });
    let length = quote! {
        /// The length of the proposal in bytes.
        pub const LENGTH: usize = #length;
    };

    // new function
    // The new function takes the header and all the fields.
    // It returns Self.
    let new = {
        let header_field = fields.iter().next().unwrap();
        let header_field_name = header_field.ident.as_ref().unwrap();
        let header_field_type = &header_field.ty;
        let fields_names = fields.iter().skip(1).map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            quote! { #field_name }
        });
        let fields_with_types = fields.iter().skip(1).map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let field_type = &f.ty;
            quote! { #field_name: #field_type }
        });
        quote! {
            /// Creates a new proposal.
            #[must_use]
            pub const fn new(#header_field_name: #header_field_type, #(#fields_with_types),*) -> Self {
                Self { #header_field_name, #(#fields_names),* }
            }
        }
    };

    // header getter
    // returns ProposalHeader.
    let header_getter = {
        let header_field = fields.iter().next().unwrap();
        let header_field_name = header_field.ident.as_ref().unwrap();
        quote! {
            /// Returns the header of the proposal.
            #[must_use]
            pub const fn header(&self) -> ProposalHeader {
                self.#header_field_name
            }
        }
    };

    // field getters
    // returns a ref to the field type.
    let field_getters = fields.iter().skip(1).map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;
        quote! {
            /// Returns a reference to that field of the proposal.
            #[must_use]
            pub const fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #length
            #new
            #header_getter
            #(#field_getters)*
        }
    };
    Ok(expanded.into())
}

#[proc_macro_derive(Proposal, attributes(proposal))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    derive_proposal(input).unwrap_or_else(|err| err.to_compile_error().into())
}
