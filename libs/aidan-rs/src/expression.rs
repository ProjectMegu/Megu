pub mod call;

pub struct Expression {
    pub(crate) ref_: binaryen_capi_sys::BinaryenExpressionRef,
}