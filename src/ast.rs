// @todo will need `transition` types for parsing migration files

pub struct Root {
    types: Vec<ModelTypeDef>,
    literal_reference: Vec<String> // possibly just put this on the parser but might be handy? Could also possibly use Box as a pointer to an tree node? 
}

pub struct ModelTypeDef {
    name: String,
    fields: Vec<FieldDef>
}

pub struct FieldDef {
    name: String,
    type: String // @todo could possibly use types from interpetors? Or leave as string and let the interpretor handle?
}