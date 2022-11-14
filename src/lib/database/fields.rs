#[derive(Clone)]
pub enum SQLType {
    Null,
    Integer,
    Real,
    Text,
}

#[derive(Clone)]
pub enum SQLDataType {
    Null,
    Integer(i32),
    Real(f32),
    Text(String),
}

pub trait SerialiseField {
    fn serialise(&self) -> String;
}

pub trait DeserialiseField {
    fn deserialise(database_val: SQLDataType) -> Self;
}

pub struct DefaultTypeField {
    value: SQLDataType,
}

impl SerialiseField for DefaultTypeField {
    fn serialise(&self) -> String {
        match &self.value {
            SQLDataType::Null => "null".to_string(),
            SQLDataType::Integer(i) => i.to_string(),
            SQLDataType::Real(r) => r.to_string(),
            SQLDataType::Text(t) => t.clone(),
        }
    }
}

impl DeserialiseField for DefaultTypeField {
    fn deserialise(database_val: SQLDataType) -> Self {
        Self { value: database_val }
    }
}
