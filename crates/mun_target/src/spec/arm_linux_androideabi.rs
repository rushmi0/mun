use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "arm-linux-androideabi".into(),
        pointer_width: 32,
        arch: "arm".into(),
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        options: TargetOptions {
            abi: "eabi".into(),
            // https://developer.android.com/ndk/guides/abis.html#armeabi
            features: "+strict-align,+v5te".into(),
            ..super::android_base::opts()
        },
    }
}
