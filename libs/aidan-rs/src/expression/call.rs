use std::ptr::null_mut;

use crate::{types::Type, utils::to_cstring};

use super::Expression;

impl crate::module::Module {
    pub fn expr_call(&mut self,func_name: String, operands: Vec<Expression>, return_type: Type) -> Expression{
        let func_name = to_cstring(func_name);
        if operands.is_empty() {
            return Expression { ref_: unsafe {
                binaryen_capi_sys::BinaryenCall(self.ref_, func_name.as_ptr(), null_mut(), 0, return_type.type_ref)
            } }
        }        

        let mut exprs = operands.iter().map(|e| e.ref_).collect::<Vec<_>>();
        Expression { ref_: unsafe {
            binaryen_capi_sys::BinaryenCall(self.ref_, func_name.as_ptr(), exprs.as_mut_ptr(), exprs.len() as u32, return_type.type_ref)
        } }
    }
}