#[derive(Clone)]
pub enum DataType {
    Null,
    Integer(i32),
    Real(f32),
    Text(String),
}

pub trait SerialiseField {
    fn serialise(&self) -> DataType;
}

pub trait DeserialiseField {
    fn deserialise(database_val: DataType) -> Self;
}

pub struct DefaultTypeField {
    value: DataType,
}

impl SerialiseField for DefaultTypeField {
    fn serialise(&self) -> DataType {
        self.value.clone()
    }
}

impl DeserialiseField for DefaultTypeField {
    fn deserialise(database_val: DataType) -> Self {
        Self { value: database_val }
    }
}
