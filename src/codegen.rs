pub mod js;

use crate::module_data::ModuleData;

pub trait CodeGenerator {
    fn generate(&self, module: ModuleData) -> Result<String, Vec<String>>;
}

pub struct StubCodeGenerator;

impl CodeGenerator for StubCodeGenerator {
    fn generate(&self, module: ModuleData) -> Result<String, Vec<String>> {
        Ok(format!("[STUB] module: {:?}", module))
    }
}