use super::fields::{SerialiseField};

struct Row {
    fields: Vec<Box<dyn SerialiseField>>
}
