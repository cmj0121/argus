// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! the pseudo data field used in the argus.

/// All the known field types defined in the argus.
pub enum FieldType {}

/// The arbritrary data stored in the argus which used to store the
/// information user save into the argus.
pub trait Field {
    /// return the field type.
    fn fieldtype(&self) -> FieldType;

    /// the total bits of the data in the field.
    fn len(&self) -> usize;

    /// generate the bytes from the field
    fn as_bytes(&self) -> &'static [u8];

    /// return the field is zero (not-been-set)
    fn is_zero(&self) -> bool;

    /// show as the human-readable string
    fn as_str(&self) -> &str;
}
