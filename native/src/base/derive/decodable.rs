/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
 *
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, GenericParam, parse_macro_input, parse_quote};

pub(crate) fn derive_decodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Add a bound `T: Decodable` to every type parameter T.
    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(crate::socket::Decodable));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encode = gen_encode(&input.data);
    let decode = gen_decode(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics crate::socket::Encodable for #name #ty_generics #where_clause {
            fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
                #encode
                Ok(())
            }
        }
        impl #impl_generics crate::socket::Decodable for #name #ty_generics #where_clause {
            fn decode(r: &mut impl std::io::Read) -> std::io::Result<Self> {
                let val = #decode;
                Ok(val)
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

// Generate an expression to encode each field.
fn gen_encode(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    //     self.x.encode(w)?; self.y.encode(w)?; self.z.encode(w)?;
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! { f.span() =>
                            crate::socket::Encodable::encode(&self.#name, w)?;
                        }
                    });
                    quote! {
                        #(#recurse)*
                    }
                }
                _ => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Generate an expression to decode each field.
fn gen_decode(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    //     Self { x: Decodable::decode(r)?, y: Decodable::decode(r)?, }
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! { f.span() =>
                            #name: crate::socket::Decodable::decode(r)?,
                        }
                    });
                    quote! {
                        Self { #(#recurse)* }
                    }
                }
                _ => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
