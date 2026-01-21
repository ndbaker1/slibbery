// stub_generator/src/main.rs
use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use slibbery::{
    providers::{HeaderProvider, SignatureProvider},
    FunctionSignature,
};

fn copy_templates_to_output(
    output_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Templates are in the same directory as the binary
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let templates_dir = exe_dir.join("templates");

    // Create output directory structure first
    fs::create_dir_all(output_dir.join("src"))?;

    // Copy Cargo.toml template
    fs::copy(
        templates_dir.join("Cargo.toml"),
        output_dir.join("Cargo.toml"),
    )?;

    // Copy src/lib.rs template
    fs::copy(
        templates_dir.join("src/lib.rs"),
        output_dir.join("src/lib.rs"),
    )?;

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
        eprintln!(
            "Usage: {} <input.so> <output_dir> <header.h> [--lib-path <path>]",
            args[0]
        );
        std::process::exit(1);
    }

    let so_path = &args[1];
    let output_dir = PathBuf::from(&args[2]);
    let header_path = args[3].clone();
    let mut lib_path_override = None;

    let mut i = 4;
    while i < args.len() {
        match args[i].as_str() {
            "--lib-path" => {
                if i + 1 < args.len() {
                    lib_path_override = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --lib-path requires a path argument");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: unexpected argument '{}'", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

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

    println!("Found {} functions", functions.len());

    // Get function signatures from header provider
    println!("Using header file: {}", header_path);
    let provider: Box<dyn SignatureProvider> = Box::new(HeaderProvider::new(header_path));

    let signatures = provider.get_signatures(&elf, &buffer).unwrap_or_default();
    println!("Parsed {} function signatures", signatures.len());

    // Copy templates to output directory
    copy_templates_to_output(&output_dir)?;

    // Generate function stubs
    let function_stubs = generate_function_stubs(&functions, &signatures)?;

    // Determine the library path to embed
    let lib_path = if let Some(override_path) = lib_path_override {
        override_path
    } else if std::path::Path::new(so_path).is_absolute() {
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
