use std::collections::HashMap;
use syn::{parse_str, FnArg, ForeignItem, Item, PatType, Type};

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
            .sort_semantically(true)
            .generate()
            .map_err(|e| format!("Bindgen failed: {}", e))?;

        let generated_code = bindings.to_string();

        // Parse the generated code with syn
        let syntax_tree = parse_str::<syn::File>(&generated_code)?;

        let mut bindings_parts = Vec::new();

        for item in syntax_tree.items {
            match item {
                Item::ForeignMod(foreign_mod) => {
                    // Check if it's extern "C"
                    let is_extern_c = foreign_mod
                        .abi
                        .name
                        .as_ref()
                        .map(|n| n.value() == "C")
                        .unwrap_or(false);
                    if is_extern_c {
                        for foreign_item in &foreign_mod.items {
                            if let ForeignItem::Fn(func) = foreign_item {
                                let func_name = func.sig.ident.to_string();
                                let return_type = match &func.sig.output {
                                    syn::ReturnType::Default => "()".to_string(),
                                    syn::ReturnType::Type(_, ty) => syn_type_to_rust(ty),
                                };
                                let mut params = Vec::new();

                                for arg in &func.sig.inputs {
                                    if let FnArg::Typed(PatType { ty, .. }) = arg {
                                        params.push(syn_type_to_rust(ty));
                                    }
                                }

                                signatures.insert(
                                    func_name.clone(),
                                    FunctionSignature {
                                        name: func_name,
                                        params,
                                        return_type,
                                    },
                                );
                            }
                        }
                    }
                    // Don't include extern "C" blocks in bindings
                }
                _ => {
                    // Other items like type definitions, consts, etc.
                    bindings_parts.push(quote::quote!(#item).to_string());
                }
            }
        }

        let bindings_code = bindings_parts.join("\n");

        Ok(BindgenResult {
            signatures,
            bindings: bindings_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_bindgen_provider_simple_function() {
        let header_content = r#"
int add(int a, int b);
void print_hello(void);
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, header_content).unwrap();

        let provider = BindgenProvider::new(temp_file.path().to_str().unwrap().to_string());
        let bindgen_result = provider.get_signatures_from_bindgen().unwrap();

        assert_eq!(bindgen_result.signatures.len(), 2);

        let add_sig = bindgen_result.signatures.get("add").unwrap();
        assert_eq!(add_sig.name, "add");
        assert_eq!(add_sig.params, vec!["c_int", "c_int"]);
        assert_eq!(add_sig.return_type, "c_int");

        let hello_sig = bindgen_result.signatures.get("print_hello").unwrap();
        assert_eq!(hello_sig.name, "print_hello");
        assert_eq!(hello_sig.params.len(), 0);
        assert_eq!(hello_sig.return_type, "()");
    }

    #[test]
    fn test_bindgen_provider_complex_types() {
        let header_content = r#"
unsigned int get_count(void);
float calculate(double value, char* name);
long process_data(short id, unsigned long flags);
int get_version(int* version);
const char* get_name(void);
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, header_content).unwrap();

        let provider = BindgenProvider::new(temp_file.path().to_str().unwrap().to_string());
        let bindgen_result = provider.get_signatures_from_bindgen().unwrap();

        assert_eq!(bindgen_result.signatures.len(), 5);

        let get_count_sig = bindgen_result.signatures.get("get_count").unwrap();
        assert_eq!(get_count_sig.return_type, "c_uint");
        assert_eq!(get_count_sig.params.len(), 0);

        let calc_sig = bindgen_result.signatures.get("calculate").unwrap();
        assert_eq!(calc_sig.return_type, "f32");
        assert_eq!(calc_sig.params, vec!["f64", "*mut c_char"]);

        let process_sig = bindgen_result.signatures.get("process_data").unwrap();
        assert_eq!(process_sig.return_type, "c_long");
        assert_eq!(process_sig.params, vec!["c_short", "c_ulong"]);

        let get_version_sig = bindgen_result.signatures.get("get_version").unwrap();
        assert_eq!(get_version_sig.return_type, "c_int");
        assert_eq!(get_version_sig.params, vec!["*mut c_int"]);

        let get_name_sig = bindgen_result.signatures.get("get_name").unwrap();
        assert_eq!(get_name_sig.return_type, "*const c_char");
        assert_eq!(get_name_sig.params.len(), 0);
    }

    #[test]
    fn test_bindgen_provider_with_enums() {
        let header_content = r#"
typedef enum nvmlReturn_enum {
    NVML_SUCCESS = 0,
    NVML_ERROR_UNKNOWN = 999
} nvmlReturn_t;

nvmlReturn_t nvmlInit_v2(void);
nvmlReturn_t nvmlShutdown(void);
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, header_content).unwrap();

        let provider = BindgenProvider::new(temp_file.path().to_str().unwrap().to_string());
        let bindgen_result = provider.get_signatures_from_bindgen().unwrap();

        assert_eq!(bindgen_result.signatures.len(), 2);

        let init_sig = bindgen_result.signatures.get("nvmlInit_v2").unwrap();
        assert_eq!(init_sig.name, "nvmlInit_v2");
        assert_eq!(init_sig.params.len(), 0);

        assert!(init_sig.return_type == "nvmlReturn_t");

        let shutdown_sig = bindgen_result.signatures.get("nvmlShutdown").unwrap();
        assert_eq!(shutdown_sig.name, "nvmlShutdown");
        assert_eq!(shutdown_sig.params.len(), 0);
        assert!(shutdown_sig.return_type == "nvmlReturn_t");
    }
}
