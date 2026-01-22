// stub_generator/src/main.rs
use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use dyll::{
    providers::{HeaderProvider, SignatureProvider},
    FunctionSignature,
};

// Embedded templates
const CARGO_TOML_TEMPLATE: &str = include_str!("../project-template/Cargo.toml");
const LIB_RS_TEMPLATE: &str = include_str!("../project-template/src/lib.rs");

fn copy_templates_to_output(
    output_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory structure first
    fs::create_dir_all(output_dir.join("src"))?;

    // Write Cargo.toml template
    fs::write(output_dir.join("Cargo.toml"), CARGO_TOML_TEMPLATE)?;

    // Write src/lib.rs template
    fs::write(output_dir.join("src/lib.rs"), LIB_RS_TEMPLATE)?;

    Ok(())
}

fn generate_function_stubs(
    functions: &[String],
    signatures: &HashMap<String, FunctionSignature>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut code = String::new();

    // Generate stub for each function
    for func in functions {
        let stub_code = if let Some(sig) = signatures.get(func) {
            // Generate type-aware stub
            let params_str = sig.params.join(", ");
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
            format!(
                r#"
// {params_str} -> {return_type}
#[no_mangle]
pub extern "C" fn {func}{args_def} -> {return_type} {{
    unsafe {{
        // Add your mock logic here

        // Forward to original
        let orig: extern "C" fn{args_def} -> {return_type} = std::mem::transmute(get_symbol(b"{func}\0"));
        orig({params_call})
    }}
}}
"#,
                func = func,
                args_def = args_def,
                return_type = sig.return_type,
                params_call = params_call,
                params_str = params_str
            )
        } else {
            // Fallback to generic stub
            format!(
                r#"
// Generic stub - unknown signature
#[no_mangle]
pub extern "C" fn {func}() {{
    unsafe {{
        // Add your mock logic here

        // Forward to original
        let orig: extern "C" fn() = std::mem::transmute(get_symbol(b"{func}\0"));
        orig()
    }}
}}
"#,
                func = func
            )
        };
        code.push_str(&stub_code);
    }

    Ok(code)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 || args.len() > 6 {
        eprintln!("Usage: {} <input.so> <output_dir> <header.h>", args[0]);
        std::process::exit(1);
    }

    let so_path = &args[1];
    let output_dir = PathBuf::from(&args[2]);
    let header_path = args[3].clone();

    let buffer = fs::read(so_path)?;
    let elf = Elf::parse(&buffer)?;

    // Extract function symbols
    let mut functions = Vec::new();

    for sym in &elf.dynsyms {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if sym.is_function() && !name.is_empty() && sym.st_value != 0 {
                functions.push(name.to_string());
            }
        }
    }

    // Get function signatures from header provider
    println!("Using header file: {}", header_path);
    let provider: Box<dyn SignatureProvider> = Box::new(HeaderProvider::new(header_path));
    let signatures = provider.get_signatures(so_path).unwrap_or_default();
    println!("Parsed {} function signatures", signatures.len());

    // Copy templates to output directory
    copy_templates_to_output(&output_dir)?;

    // Generate function stubs
    let function_stubs = generate_function_stubs(&functions, &signatures)?;

    // Determine the library path to embed
    let lib_path = if std::path::Path::new(so_path).is_absolute() {
        so_path.to_string()
    } else {
        // For relative paths, assume from output_dir/src/, go up and then to so_path
        format!("../../{}", so_path)
    };

    let lib_include = format!(
        "const ORIGINAL_SO: &[u8] = include_bytes!(\"{}\");",
        lib_path
    );

    // Replace placeholders in lib.rs
    let lib_rs_path = output_dir.join("src/lib.rs");
    let mut content = fs::read_to_string(&lib_rs_path)?;
    content = content.replace("{{FUNCTION_STUBS}}", &function_stubs);
    content = content.replace("{{LIB_INCLUDE}}", &lib_include);
    fs::write(&lib_rs_path, content)?;

    println!("Generated stub library in {}", output_dir.display());
    println!(
        "Build with: cd {} && cargo build --release",
        output_dir.display()
    );

    Ok(())
}
