extern crate proc_macro;
use proc_macro2::{TokenStream};
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::spanned::Spanned;
use quote::{quote, quote_spanned};




/// Generates a convert::From<PDU> -> Vec<u8>
#[proc_macro_derive(PduSerialize)]
pub fn write_pdu(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let stream = generate_pushes(&input.data);

    let expanded = quote! {
        impl std::convert::From<#name> for Vec<u8> {
            fn from(p: #name) -> Vec<u8> {
                let mut output_vector = Vec::new();
                #stream
                output_vector
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn generate_pushes(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! { f.span() => {
                                output_vector.extend_from_slice(&pdu_io::ToBytes::to_bytes(p.#name));
                            }
                        }
                    });

                    quote! {
                        #(#recurse)*
                    }
                },
                Fields::Unnamed(_) => {
                    unimplemented!()
                },
                Fields::Unit => {
                    "".parse().unwrap()
                },
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

/// Generates pdu_io::ParsePdu for a PDU/Struct
#[proc_macro_derive(PduDeserialize)]
pub fn read_pdu(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let pdu_size = size_of_fields(&input.data);
    let read_code = proccess_read(&input.data);
    let expanded = quote! {
        impl pdu_io::ParsePdu for #name {
            fn try_parse(buffer: &[u8]) -> Option<(#name, usize)> {
                let size = #pdu_size;
                if buffer.len() < size {
                    return None
                }
                let mut buffer = &buffer[..];
                Some((#name {
                    #read_code 
                }, size))
                 
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn size_of_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ty;
                        quote_spanned! { f.span() => {
                                <#name>::size_of()
                            }
                        }
                    });

                    quote! {
                        0 #(+ #recurse)*
                    }
                },
                Fields::Unnamed(_) => {
                    unimplemented!()
                },
                Fields::Unit => {
                    "".parse().unwrap()
                },
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn proccess_read(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! {
                            #name: pdu_io::FromBytes::from_bytes(&mut buffer)
                        }
                        
                    });

                    quote! {
                        #(#recurse),*
                    }
                },
                Fields::Unnamed(_) => {
                    unimplemented!()
                },
                Fields::Unit => {
                    unimplemented!()
                },
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
