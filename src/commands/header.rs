use crate::cli::{Cli, HeaderArgs};
use crate::elf;
use crate::generator;
use crate::providers::{HeaderProvider, SignatureProvider};
use crate::template;

pub fn run(cli: &Cli, args: &HeaderArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing library: {}", cli.lib_path.display());

    let functions = elf::extract_function_symbols(&cli.lib_path)?;
    println!("Found {} function symbols", functions.len());

    println!("Using header file: {}", args.header_file.display());
    let provider: Box<dyn SignatureProvider> = Box::new(HeaderProvider::new(
        args.header_file.to_str().unwrap().to_string(),
    ));
    let signatures = provider
        .get_signatures(cli.lib_path.to_str().unwrap())
        .unwrap_or_default();
    println!("Parsed {} function signatures", signatures.len());

    template::copy_templates_to_output(&cli.output_dir)?;

    let function_stubs = generator::generate_function_stubs(&functions, &signatures)?;

    template::inject_generated_code(&cli.output_dir, &cli.lib_path, &function_stubs)?;

    println!("Generated stub library in {}", cli.output_dir.display());
    println!(
        "Build with: cd {} && cargo build --release",
        cli.output_dir.display()
    );

    Ok(())
}
