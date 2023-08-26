pub mod codegen_js;

use crate::module_data::ModuleData;

pub trait CodeGenerator {
    fn generate(&self, module: ModuleData) -> Result<String, Vec<String>>;
    fn get_stdlib_symbol(&self, name: &str) -> String;
}

pub struct StubCodeGenerator;

impl CodeGenerator for StubCodeGenerator {
    fn generate(&self, module: ModuleData) -> Result<String, Vec<String>> {
        Ok(format!("[STUB] module: {:?}", module))
    }
    
    fn get_stdlib_symbol(&self, name: &str) -> String {
        format!("[STUB] stdlib: {}", name)
    }
}