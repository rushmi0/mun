use std::{cell::RefCell, mem};

use inkwell::types::AnyType;
use mun_abi as abi;

use crate::{
    ir::types as ir,
    value::{IrTypeContext, SizedValueType},
};

#[test]
fn abi_struct_sizes() {
    fn test_type_size<'ink, A: Sized, T: SizedValueType<'ink>>(context: &IrTypeContext<'ink, '_>) {
        let ir_type = T::get_ir_type(context);
        println!("{}", ir_type.print_to_string().to_string());
        let ir_size = context.target_data.get_abi_size(&ir_type);
        assert_eq!(mem::size_of::<A>(), ir_size as usize);
        assert_eq!(mem::align_of::<A>(), T::get_alignment(context));
    }

    // Get target data for the current host
    let target = mun_target::spec::Target::host_target().expect("unable to determine host target");
    let target_data = inkwell::targets::TargetData::create(&target.data_layout);

    // Create an LLVM context and type context to work with.
    let context = inkwell::context::Context::create();
    let type_context = IrTypeContext {
        context: &context,
        target_data: &target_data,
        struct_types: &RefCell::default(),
    };

    test_type_size::<abi::Guid, abi::Guid>(&type_context);
    test_type_size::<abi::Privacy, abi::Privacy>(&type_context);
    test_type_size::<abi::StructMemoryKind, abi::StructMemoryKind>(&type_context);
    test_type_size::<abi::TypeId<'_>, ir::TypeId<'_>>(&type_context);
    test_type_size::<abi::PointerTypeId<'_>, ir::PointerTypeId<'_>>(&type_context);
    test_type_size::<abi::ArrayTypeId<'_>, ir::ArrayTypeId<'_>>(&type_context);
    test_type_size::<abi::TypeDefinitionData<'_>, ir::TypeDefinitionData<'_>>(&type_context);
    test_type_size::<abi::StructDefinition<'_>, ir::StructDefinition<'_>>(&type_context);
    test_type_size::<abi::TypeDefinition<'_>, ir::TypeDefinition<'_>>(&type_context);
    test_type_size::<abi::FunctionSignature<'_>, ir::FunctionSignature<'_>>(&type_context);
    test_type_size::<abi::FunctionPrototype<'_>, ir::FunctionPrototype<'_>>(&type_context);
    test_type_size::<abi::ModuleInfo<'_>, ir::ModuleInfo<'_>>(&type_context);
    test_type_size::<abi::DispatchTable<'_>, ir::DispatchTable<'_>>(&type_context);
    test_type_size::<abi::TypeLut<'_>, ir::TypeLut<'_>>(&type_context);
    test_type_size::<abi::AssemblyInfo<'_>, ir::AssemblyInfo<'_>>(&type_context);
}

/// The ABI types are emitted as packed IR structs, so their layout must be
/// computed from the ABI alignment of their fields. On 32-bit x86 the ABI
/// alignment of 64-bit values and aggregates differs from LLVM's preferred
/// alignment, which previously made the generated layout differ from the one
/// `rustc` uses.
///
/// The expected values are `size_of` and `align_of` of the `mun_abi` types as
/// compiled by `rustc` for `i686-linux-android`.
#[test]
fn abi_struct_layouts_i686() {
    fn test_type_layout<'ink, T: SizedValueType<'ink>>(
        context: &IrTypeContext<'ink, '_>,
        size: u64,
        alignment: usize,
    ) {
        let ir_type = T::get_ir_type(context);
        let name = std::any::type_name::<T>();
        assert_eq!(
            context.target_data.get_abi_size(&ir_type),
            size,
            "size of {name}"
        );
        assert_eq!(T::get_alignment(context), alignment, "alignment of {name}");
    }

    for triple in ["i686-linux-android", "i686-unknown-linux-gnu"] {
        let target = mun_target::spec::Target::search(triple).unwrap();
        let target_data = inkwell::targets::TargetData::create(&target.data_layout);
        let context = inkwell::context::Context::create();
        let type_context = IrTypeContext {
            context: &context,
            target_data: &target_data,
            struct_types: &RefCell::default(),
        };

        test_type_layout::<ir::TypeId<'_>>(&type_context, 20, 4);
        test_type_layout::<ir::PointerTypeId<'_>>(&type_context, 8, 4);
        test_type_layout::<ir::ArrayTypeId<'_>>(&type_context, 4, 4);
        test_type_layout::<ir::TypeDefinitionData<'_>>(&type_context, 36, 4);
        test_type_layout::<ir::StructDefinition<'_>>(&type_context, 32, 4);
        test_type_layout::<ir::TypeDefinition<'_>>(&type_context, 48, 4);
        test_type_layout::<ir::FunctionSignature<'_>>(&type_context, 28, 4);
        test_type_layout::<ir::FunctionPrototype<'_>>(&type_context, 32, 4);
        test_type_layout::<ir::FunctionDefinition<'_>>(&type_context, 36, 4);
        test_type_layout::<ir::ModuleInfo<'_>>(&type_context, 20, 4);
        test_type_layout::<ir::DispatchTable<'_>>(&type_context, 12, 4);
        test_type_layout::<ir::TypeLut<'_>>(&type_context, 16, 4);
        test_type_layout::<ir::AssemblyInfo<'_>>(&type_context, 56, 4);
    }
}
