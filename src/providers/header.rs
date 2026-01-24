use std::collections::HashMap;
use syn::{parse_str, FnArg::Typed, ForeignItem, Item, PatType, Type};

use super::SignatureProvider;
use crate::{providers::BindgenResult, FunctionSignature};

pub fn syn_type_to_rust(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            // Get the last segment for the type name
            if let Some(last) = type_path.path.segments.last() {
                last.ident.to_string()
            } else {
                "c_void".to_string()
            }
        }
        Type::Ptr(type_ptr) => {
            let mutability = if type_ptr.mutability.is_some() {
                "mut"
            } else {
                "const"
            };
            let elem = syn_type_to_rust(&type_ptr.elem);
            format!("*{} {}", mutability, elem)
        }
        Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                "()".to_string()
            } else {
                format!(
                    "({})",
                    type_tuple
                        .elems
                        .iter()
                        .map(|t| syn_type_to_rust(t))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        _ => "c_void".to_string(), // fallback
    }
}

pub struct BindgenProvider {
    header_path: String,
}

impl BindgenProvider {
    pub fn new(header_path: String) -> Self {
        BindgenProvider { header_path }
    }
}

impl SignatureProvider for BindgenProvider {
    fn get_signatures(&self, _so_path: &str) -> Result<BindgenResult, Box<dyn std::error::Error>> {
        self.get_signatures_from_bindgen()
    }
}

impl BindgenProvider {
    pub fn get_signatures_from_bindgen(&self) -> Result<BindgenResult, Box<dyn std::error::Error>> {
        let mut signatures = HashMap::new();

        // Use bindgen to generate bindings from the header
        let bindings = bindgen::Builder::default()
            .header(&self.header_path)
            .generate_comments(false)
            .layout_tests(false)
            .default_enum_style(bindgen::EnumVariation::Consts)
            .prepend_enum_name(false)
            .array_pointers_in_arguments(true)
            .sort_semantically(true)
            .generate()
            .map_err(|e| format!("Bindgen failed: {}", e))?;

        let generated_code = bindings.to_string();

        // Parse the generated code with syn
        let syntax_tree = parse_str::<syn::File>(&generated_code)?;

        let mut bindings_parts = Vec::new();

        for item in syntax_tree.items {
            match item {
                // TODO: i dont really love how this is done.
                Item::ForeignMod(ref foreign_mod) => {
                    // Check if it's extern "C"
                    let is_extern_c = foreign_mod
                        .abi
                        .name
                        .as_ref()
                        .map(|n| n.value() == "C")
                        .unwrap_or(false);
                    if is_extern_c {
                        // Extract function signatures from extern "C" blocks
                        for foreign_item in &foreign_mod.items {
                            if let ForeignItem::Fn(func) = foreign_item {
                                let name = func.sig.ident.to_string();
                                let return_type = match &func.sig.output {
                                    syn::ReturnType::Default => "()".to_string(),
                                    syn::ReturnType::Type(_, ty) => syn_type_to_rust(ty),
                                };
                                let params = func
                                    .sig
                                    .inputs
                                    .iter()
                                    .filter_map(|arg| match arg {
                                        Typed(PatType { ty, .. }) => Some(syn_type_to_rust(ty)),
                                        _ => None,
                                    })
                                    .collect();

                                signatures.insert(
                                    name.clone(),
                                    FunctionSignature {
                                        name,
                                        params,
                                        return_type,
                                    },
                                );
                            }
                        }
                    } else {
                        // Non-C extern blocks go into bindings
                        bindings_parts.push(format!("{}", quote::quote!(#item)));
                    }
                }
                _ => {
                    // Other items like type definitions, consts, etc.
                    bindings_parts.push(format!("{}", quote::quote!(#item)));
                }
            }
        }

        Ok(BindgenResult {
            signatures,
            bindings: bindings_parts.join("\n"),
        })
    }
}
