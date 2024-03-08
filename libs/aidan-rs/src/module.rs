use binaryen_capi_sys as bapi;

mod feature;

pub struct Module {
    ref_: bapi::BinaryenModuleRef,
}

impl Default for Module {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { bapi::BinaryenModuleDispose(self.ref_) }
    }
}

impl Module {
    pub fn new() -> Module {
        Module {
            ref_: unsafe { bapi::BinaryenModuleCreate() },
        }
    }

    pub fn print(&self) {
        unsafe { bapi::BinaryenModulePrint(self.ref_) }
    }

    pub fn set_features(&self, feature: &[feature::Feature]) {
        let mut features: bapi::BinaryenFeatures = 0;
        for f in feature {
            features |= f.return_feature();
        }
        unsafe { bapi::BinaryenModuleSetFeatures(self.ref_, features) }
    }
}
