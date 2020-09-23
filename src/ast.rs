use std::fmt;
// @todo will need `transition` types for parsing migration files

#[derive(Clone, Debug)]
pub struct Root {
    pub types: Vec<ModelTypeDef>,
    // Identifier_reference: Vec<String> // possibly just put this on the parser but might be handy? Could also possibly use Box as a pointer to an tree node? 
}

impl Root {
    pub fn new() -> Root {
        Root {
            types: Vec::new()
        }
    }
}

// @todo remove default once annotations are complete
#[derive(Clone, Debug, Default)]
pub struct ModelTypeDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
    pub annotation: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub enum FieldTypeType {
    basic, // Int, String, Comment etc -> needs better name
    array,
    required_array
}

// @todo remove default once annotations are complete
#[derive(Clone, Debug, Default)]
pub struct FieldDef {
    pub name: String,
    pub field_type: String, // @todo could possibly use types from interpetors? Or leave as string and let the interpretor handle?
    pub type_type: FieldTypeType,
    pub required: bool,
    pub annotation: Option<String>,
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.types;

        write!(f, "[\n")?;
        for (_, m) in vec.iter().enumerate() {
            write!(f, "\tModel: {} [\n", m.name)?;
            for (_, d) in m.fields.iter().enumerate() {
                write!(f, "\t\t{} : {}{}\n", d.name, d.field_type, if d.required == true { "!" } else { "" })?;
            }
            write!(f, "\t]\n")?;
        }
        write!(f, "]\n")
        
    }
}