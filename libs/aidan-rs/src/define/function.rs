use std::ptr::null_mut;

use crate::{
    expression::Expression,
    types::Type,
    utils::{to_cstring, FuncSigneture},
};

pub struct Function {
    pub(crate) ref_: binaryen_capi_sys::BinaryenFunctionRef,
}

impl crate::module::Module {
    pub fn add_func(
        &mut self,
        name: String,
        signeture: FuncSigneture,
        vars: Vec<Type>,
        body: Expression,
    ) -> Function {
        let name = to_cstring(name);
        if vars.is_empty() {
            unsafe {
                return Function {
                    ref_: binaryen_capi_sys::BinaryenAddFunction(
                        self.ref_,
                        name.as_ptr(),
                        signeture.param.type_ref,
                        signeture.result.type_ref,
                        null_mut(),
                        0,
                        body.ref_,
                    ),
                };
            }
        }

        let mut vars_c = vars.iter().map(|v| v.type_ref).collect::<Vec<_>>();
        unsafe {
            Function {
                ref_: binaryen_capi_sys::BinaryenAddFunction(
                    self.ref_,
                    name.as_ptr(),
                    signeture.param.type_ref,
                    signeture.result.type_ref,
                    vars_c.as_mut_ptr(),
                    vars_c.len() as u32,
                    body.ref_,
                ),
            }
        }
    }
}
