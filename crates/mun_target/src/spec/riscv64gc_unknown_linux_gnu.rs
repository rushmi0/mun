use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "riscv64-unknown-linux-gnu".into(),
        pointer_width: 64,
        arch: "riscv64".into(),
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n64-S128".into(),
        options: TargetOptions {
            cpu: "generic-rv64".into(),
            features: "+m,+a,+f,+d,+c".into(),
            // Pass floats in floating-point registers, like the host code does.
            llvm_abiname: "lp64d".into(),
            ..super::linux_base::opts()
        },
    }
}
