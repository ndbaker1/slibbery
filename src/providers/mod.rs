pub mod header;

pub use header::HeaderProvider;

pub trait SignatureProvider {
    fn get_signatures(
        &self,
        elf: &goblin::elf::Elf,
        buffer: &[u8],
    ) -> Result<std::collections::HashMap<String, crate::FunctionSignature>, Box<dyn std::error::Error>>;
}