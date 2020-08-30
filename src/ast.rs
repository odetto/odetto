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

#[derive(Clone, Debug)]
pub struct ModelTypeDef {
    pub name: String,
    pub fields: Vec<FieldDef>
}

#[derive(Clone, Debug)]
pub struct FieldDef {
    pub name: String,
    pub field_type: String // @todo could possibly use types from interpetors? Or leave as string and let the interpretor handle?
}