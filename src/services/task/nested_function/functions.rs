#![allow(non_snake_case)]

///
/// Entair list of all supported functions
#[derive(Debug)]
pub enum Functions {
    Input,
    Count,
    Add
}
///
/// 
impl Functions {
    pub fn name(&self) -> &str {
        match self {
            Functions::Input => "input",
            Functions::Count => "count",
            Functions::Add => "add",
        }
    }
}