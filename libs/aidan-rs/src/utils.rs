use crate::types::Type;

pub struct FuncSigneture {
    pub(crate) param: Type,
    pub(crate) result: Type,
}

impl FuncSigneture {
    pub fn new(param: Type, result: Type) -> Self {
        Self {
            param,
            result,
        }
    }
}

pub(crate) fn to_cstring(s: impl Into<Vec<u8>>) -> std::ffi::CString {
    std::ffi::CString::new(s).unwrap()
}