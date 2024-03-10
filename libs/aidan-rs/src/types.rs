#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    I32,
    I64,
    F32,
    F64,
    None,
    Tuple(Vec<TypeKind>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub(crate) kind: TypeKind,
    pub(crate) type_ref: binaryen_capi_sys::BinaryenType,
}

impl Type {
    pub fn new(kind: &TypeKind) -> Self{
        let type_ref = match kind {
            TypeKind::I32 => unsafe {
                binaryen_capi_sys::BinaryenTypeInt32()
            },
            TypeKind::I64 => unsafe {
                binaryen_capi_sys::BinaryenTypeInt64()
            },
            TypeKind::F32 => unsafe {
                binaryen_capi_sys::BinaryenTypeFloat32()
            },
            TypeKind::F64 => unsafe {
                binaryen_capi_sys::BinaryenTypeFloat64()
            },
            TypeKind::None => unsafe {
                binaryen_capi_sys::BinaryenTypeNone()
            },
            TypeKind::Tuple(types) => {
                let mut type_refs = Vec::new();
                for t in types {
                    type_refs.push(Type::new(t).type_ref);
                }
                unsafe {
                    binaryen_capi_sys::BinaryenTypeCreate(type_refs.as_mut_ptr(), types.len() as u32)
                }
            }
        };

        Self {
            kind: kind.clone(),
            type_ref,
        }
    }
}