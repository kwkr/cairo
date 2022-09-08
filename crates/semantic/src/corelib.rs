use std::path::PathBuf;

use defs::ids::{GenericFunctionId, GenericTypeId, ModuleId};
use filesystem::db::ProjectConfig;
use filesystem::ids::{CrateLongId, FileLongId};
use syntax::token::TokenKind;

use crate::db::SemanticGroup;
use crate::{ConcreteFunction, ConcreteType, FunctionId, FunctionLongId, TypeId, TypeLongId};

pub fn core_config(db: &dyn SemanticGroup) -> ProjectConfig {
    let core_crate = db.intern_crate(CrateLongId("core".into()));
    // TODO(spapini): find the correct path.
    // This is the directory of Cargo.toml of the syntax_codegen crate.
    let dir = env!("CARGO_MANIFEST_DIR");
    // Pop the "/crates/semantic" suffix.
    let mut path = PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned();
    path.push("corelib/mod.cairo");
    let core_root_file = db.intern_file(FileLongId::OnDisk(path));
    ProjectConfig::default().with_crate(core_crate, core_root_file)
}

pub fn core_module(db: &dyn SemanticGroup) -> ModuleId {
    let core_crate = db.intern_crate(CrateLongId("core".into()));
    ModuleId::CrateRoot(core_crate)
}

pub fn core_felt_ty(db: &dyn SemanticGroup) -> TypeId {
    let core_module = db.core_module();
    // This should not fail if the corelib is present.
    let generic_type = db
        .module_item_by_name(core_module, "felt".into())
        .expect("Unexpected diagnostics when looking for corelib")
        .and_then(GenericTypeId::from)
        .unwrap();
    db.intern_type(TypeLongId::Concrete(ConcreteType { generic_type, generic_args: vec![] }))
}

pub fn unit_ty(db: &dyn SemanticGroup) -> TypeId {
    db.intern_type(TypeLongId::Tuple(vec![]))
}

pub fn core_binary_operator(
    db: &dyn SemanticGroup,
    operator_kind: TokenKind,
) -> Option<FunctionId> {
    let core_module = db.core_module();
    let function_name = match operator_kind {
        TokenKind::Plus => "felt_add",
        TokenKind::Minus => "felt_sub",
        TokenKind::Mul => "felt_mul",
        _ => return None,
    };
    let generic_function = db
        .module_item_by_name(core_module, function_name.into())
        .expect("Unexpected diagnostics when looking for corelib.")
        .and_then(GenericFunctionId::from)?;
    Some(db.intern_function(FunctionLongId::Concrete(ConcreteFunction {
        generic_function,
        generic_args: vec![],
        return_type: core_felt_ty(db),
    })))
}
