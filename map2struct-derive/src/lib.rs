use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Map2Struct)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    if let syn::Data::Struct(data) = data {
        if let syn::Fields::Named(fields) = data.fields {

            // Step 1: build extractors
            let extractors = fields.named.iter().map(|field| {
                let name = &field.ident;
                let ty = &field.ty;
                quote! {
                    let #name = map
                        .remove(stringify!(#name))
                        .ok_or_else(|| ::map2struct::Error::MissingField(stringify!(#name).to_string()))?
                        .parse::<#ty>()
                        .map_err(|e| ::map2struct::Error::FieldConversion(stringify!(#name).to_string(), e.into()))?;
                }
            });

            // Step 2: Build builder
            let inserters = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote!{#name}
            });

            // Step 3: Build implementation
            let output = quote! {
                impl ::map2struct::Map2Struct for #ident {
                    fn from_map(mut map: ::std::collections::HashMap<String, String>) -> ::map2struct::Result<Self> {
                        // Try to extract every field; raise error on option
                        // Check that map is empty
                        // Build and return result
                        #(#extractors)*
                        if !map.is_empty() {
                            return Err(::map2struct::Error::ExtraFields(map.keys().cloned().collect()));
                        }
                        let t = Self {
                            #(#inserters,)*
                        };
                        Ok(t)
                    } 
                }
            };

            return output.into()
        }
    }
    TokenStream::from(
        syn::Error::new(
            ident.span(),
            "Only structs with named fields can derive `Map2Struct`"
        ).to_compile_error()
    )
}