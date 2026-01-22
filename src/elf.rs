use goblin::elf::Elf;
use std::fs;

pub fn extract_function_symbols(
    so_path: &std::path::Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let buffer = fs::read(so_path)?;
    let elf = Elf::parse(&buffer)?;

    let mut functions = Vec::new();

    for sym in &elf.dynsyms {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if sym.is_function() && !name.is_empty() && sym.st_value != 0 {
                functions.push(name.to_string());
            }
        }
    }

    Ok(functions)
}
