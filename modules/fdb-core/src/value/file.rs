use crate::file::FDBFieldData;

use super::{Context, Value, UnknownValueType, ValueType};

#[derive(Debug, Copy, Clone, PartialEq)]
/// The `common::Context` for used to make `file::FDBFieldValue`
pub struct FileContext;

impl Context for FileContext {
    type String = IndirectValue;
    type I64 = IndirectValue;
    type XML = IndirectValue;
}

/// An indirect value in the file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IndirectValue {
    /// The base of the value
    pub addr: u32,
}

/// A database field value repr
pub type FDBFieldValue = Value<FileContext>;

impl TryFrom<FDBFieldData> for FDBFieldValue {
    type Error = UnknownValueType;

    fn try_from(value: FDBFieldData) -> Result<Self, Self::Error> {
        let value_type: ValueType = value.data_type.try_into()?;
        Ok(match value_type {
            ValueType::Nothing => FDBFieldValue::Nothing,
            ValueType::Integer => FDBFieldValue::Integer(i32::from_le_bytes(value.value)),
            ValueType::Float => FDBFieldValue::Float(f32::from_le_bytes(value.value)),
            ValueType::Text => FDBFieldValue::Text(IndirectValue {
                addr: u32::from_le_bytes(value.value),
            }),
            ValueType::Boolean => FDBFieldValue::Boolean(value.value != [0; 4]),
            ValueType::BigInt => FDBFieldValue::BigInt(IndirectValue {
                addr: u32::from_le_bytes(value.value),
            }),
            ValueType::VarChar => FDBFieldValue::VarChar(IndirectValue {
                addr: u32::from_le_bytes(value.value),
            }),
        })
    }
}
