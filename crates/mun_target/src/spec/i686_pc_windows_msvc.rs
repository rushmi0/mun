use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    let base = super::windows_msvc_base::opts();

    let mut pre_link_args = base.pre_link_args.clone();
    pre_link_args.extend([
        // Mark all dynamic libraries and executables as compatible with the
        // larger 4GiB address space available to x86 Windows binaries on x86_64.
        "/LARGEADDRESSAWARE".into(),
        // Ensure the linker will only produce an image if it can also produce
        // a table of the image's safe exception handlers.
        // https://docs.microsoft.com/en-us/cpp/build/reference/safeseh-image-has-safe-exception-handlers
        "/SAFESEH".into(),
    ]);

    Target {
        llvm_target: "i686-pc-windows-msvc".into(),
        pointer_width: 32,
        arch: "x86".into(),
        data_layout:
            "e-m:x-p:32:32-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32-a:0:32-S32"
                .into(),
        options: TargetOptions {
            cpu: "pentium4".into(),
            pre_link_args,
            ..base
        },
    }
}
