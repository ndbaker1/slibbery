pub mod providers;

pub mod cli;
pub mod commands;
pub mod elf;
pub mod generator;
pub mod template;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: String,
}
