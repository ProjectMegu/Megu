pub enum Feature {
    #[allow(clippy::upper_case_acronyms)]
    MVP,
    Atomics,
    MutableGlobals,
    NontrappingFPToInt,
    SignExt,
    SIMD128,
    BulkMemory,
    ExceptionHandling,
    TailCall,
    ReferenceTypes,
    Multivalue,
    GC,
    Memory64,
    RelaxedSIMD,
    ExtendedConst,
    Strings,
    MultiMemory,
    All
}

impl Feature {
    pub(crate) fn return_feature(&self) -> binaryen_capi_sys::BinaryenFeatures {
        use Feature::*;
        use binaryen_capi_sys::*;
        unsafe {
            match self {
                MVP => BinaryenFeatureMVP(),
                Atomics => BinaryenFeatureAtomics(),
                MutableGlobals => BinaryenFeatureMutableGlobals(),
                NontrappingFPToInt => BinaryenFeatureNontrappingFPToInt(),
                SignExt => BinaryenFeatureSignExt(),
                SIMD128 => BinaryenFeatureSIMD128(),
                BulkMemory => BinaryenFeatureBulkMemory(),
                ExceptionHandling => BinaryenFeatureExceptionHandling(),
                TailCall => BinaryenFeatureTailCall(),
                ReferenceTypes => BinaryenFeatureReferenceTypes(),
                Multivalue => BinaryenFeatureMultivalue(),
                GC => BinaryenFeatureGC(),
                Memory64 => BinaryenFeatureMemory64(),
                RelaxedSIMD => BinaryenFeatureRelaxedSIMD(),
                ExtendedConst => BinaryenFeatureExtendedConst(),
                Strings => BinaryenFeatureStrings(),
                MultiMemory => BinaryenFeatureMultiMemory(),
                All => BinaryenFeatureAll(),
            }
        }
    }
}