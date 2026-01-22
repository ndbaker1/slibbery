use crate::FunctionSignature;
use std::collections::HashMap;

pub fn generate_function_stubs(
    functions: &[String],
    signatures: &HashMap<String, FunctionSignature>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut code = String::new();

    for func in functions {
        let stub_code = if let Some(sig) = signatures.get(func) {
            let params_call = if sig.params.is_empty() {
                "".to_string()
            } else {
                (0..sig.params.len())
                    .map(|i| format!("arg{}", i))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let args_def = if sig.params.is_empty() {
                "()".to_string()
            } else {
                format!(
                    "({})",
                    (0..sig.params.len())
                        .map(|i| format!("arg{}: {}", i, sig.params[i]))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };

            // {params_str} -> {return_type}
            format!(
                r#"
#[no_mangle]
pub extern "C" fn {func}{args_def} -> {return_type} {{
    let orig: extern "C" fn{args_def} -> {return_type} = unsafe {{ std::mem::transmute(get_symbol(b"{func}\0")) }};

    // add your mock logic here
    eprintln!("[CALL] {func}");

    // forward to original
    orig({params_call})
}}
"#,
                func = func,
                args_def = args_def,
                return_type = sig.return_type,
                params_call = params_call,
            )
        } else {
            // Generic stub - unknown signature
            format!(
                r#"
#[no_mangle]
pub extern "C" fn {func}() {{
    let orig: extern "C" fn() = unsafe {{ std::mem::transmute(get_symbol(b"{func}\0")) }};

    // add your mock logic here
    eprintln!("[CALL] {func}");

    // forward to original
    orig()
}}
"#,
                func = func
            )
        };
        code.push_str(&stub_code);
    }

    Ok(code)
}
