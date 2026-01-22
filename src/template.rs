use std::fs;

const CARGO_TOML_TEMPLATE: &str = include_str!("../project-template/Cargo.toml");
const LIB_RS_TEMPLATE: &str = include_str!("../project-template/src/lib.rs");

pub fn copy_templates_to_output(
    output_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir.join("src"))?;
    fs::write(output_dir.join("Cargo.toml"), CARGO_TOML_TEMPLATE)?;
    fs::write(output_dir.join("src/lib.rs"), LIB_RS_TEMPLATE)?;

    Ok(())
}

pub fn inject_generated_code(
    output_dir: &std::path::Path,
    lib_path: &std::path::Path,
    function_stubs: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let lib_path_str = if lib_path.is_absolute() {
        lib_path.to_str().unwrap().to_string()
    } else {
        // two levels up because the resulting file will be under src/lib.rs
        format!("../../{}", lib_path.display())
    };

    let lib_include = format!(
        "const ORIGINAL_SO: &[u8] = include_bytes!(\"{}\");",
        lib_path_str
    );

    let lib_rs_path = output_dir.join("src/lib.rs");
    let mut content = fs::read_to_string(&lib_rs_path)?;
    content = content.replace("{{FUNCTION_STUBS}}", function_stubs);
    content = content.replace("{{LIB_INCLUDE}}", &lib_include);
    fs::write(&lib_rs_path, content)?;

    Ok(())
}
