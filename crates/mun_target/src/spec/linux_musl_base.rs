use crate::spec::TargetOptions;

pub fn opts() -> TargetOptions {
    TargetOptions {
        env: "musl".to_string(),
        ..super::linux_base::opts()
    }
}
