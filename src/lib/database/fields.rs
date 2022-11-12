#[derive(Clone)]
pub enum DataType {
    Null,
    Integer(i32),
    Real(f32),
    Text(String),
}

pub trait SerialiseField {
    fn serialise(&self) -> String;
}

pub trait DeserialiseField {
    fn deserialise(database_val: DataType) -> Self;
}

pub struct DefaultTypeField {
    value: DataType,
}

impl SerialiseField for DefaultTypeField {
    fn serialise(&self) -> String {
        match &self.value {
            DataType::Null => "null".to_string(),
            DataType::Integer(i) => i.to_string(),
            DataType::Real(r) => r.to_string(),
            DataType::Text(t) => t.clone(),
        }
    }
}

impl DeserialiseField for DefaultTypeField {
    fn deserialise(database_val: DataType) -> Self {
        Self { value: database_val }
    }
}
