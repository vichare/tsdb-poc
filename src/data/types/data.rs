use crate::data::types::basic_types::Value;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Data<V: Value> {
    pub value: V,
    pub timestamp: NaiveDateTime,
}
