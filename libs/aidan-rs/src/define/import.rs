use crate::utils::{to_cstring, FuncSigneture};

impl crate::module::Module {
    pub fn add_func_import(
        &mut self,
        internal_name: String,
        external_module: String,
        external_name: String,
        signeture: FuncSigneture,
    ) {
        let internal_name = to_cstring(internal_name);
        let external_module = to_cstring(external_module);
        let external_name = to_cstring(external_name);
        unsafe {
            binaryen_capi_sys::BinaryenAddFunctionImport(
                self.ref_,
                internal_name.as_ptr(),
                external_module.as_ptr(),
                external_name.as_ptr(),
                signeture.param.type_ref,
                signeture.result.type_ref,
            )
        }
    }
}
