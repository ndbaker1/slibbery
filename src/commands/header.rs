use tracing::{debug, info};

use crate::cli::{Cli, HeaderArgs};
use crate::elf;
use crate::generator;
use crate::providers::SignatureProvider;
use crate::providers::header::BindgenProvider;
use crate::template;

pub fn run(cli: &Cli, args: &HeaderArgs) -> Result<(), Box<dyn std::error::Error>> {
    debug!("processing library: {}", cli.lib_path.display());

    let functions = elf::extract_function_symbols(&cli.lib_path)?;

    info!("detected {} function symbols", functions.len());

    info!("using header file: {}", args.header_file.display());

    let provider = BindgenProvider::new(args.header_file.to_str().unwrap().to_string());
    let bindgen_result = provider
        .get_signatures(cli.lib_path.to_str().unwrap())
        .unwrap_or_default();

    info!(
        "parsed {} function signatures",
        bindgen_result.signatures.len()
    );

    let (known_stubs, unknown_stubs) =
        generator::generate_function_stubs(&functions, &bindgen_result.signatures)?;

    template::generate_project(
        &cli.output_dir,
        &cli.lib_path,
        &known_stubs,
        &unknown_stubs,
        &bindgen_result.bindings,
    )?;

    Ok(())
}
