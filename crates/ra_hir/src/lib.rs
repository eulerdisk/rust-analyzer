//! HIR (previously known as descriptors) provides a high-level object oriented
//! access to Rust code.
//!
//! The principal difference between HIR and syntax trees is that HIR is bound
//! to a particular crate instance. That is, it has cfg flags and features
//! applied. So, the relation between syntax and HIR is many-to-one.

macro_rules! impl_froms {
    ($e:ident: $($v:ident), *) => {
        $(
            impl From<$v> for $e {
                fn from(it: $v) -> $e {
                    $e::$v(it)
                }
            }
        )*
    }
}

pub mod db;
#[macro_use]
pub mod mock;
mod path;
pub mod source_binder;

mod ids;
mod macros;
mod name;
mod module_tree;
mod nameres;
mod adt;
mod type_ref;
mod ty;
mod impl_block;
mod expr;
mod generics;
mod docs;
mod resolve;
mod import;

mod code_model_api;
mod code_model_impl;

#[cfg(test)]
mod marks;

use crate::{
    db::{HirDatabase, PersistentHirDatabase},
    name::{AsName, KnownName},
    ids::{SourceItemId, SourceFileItems},
};

pub use self::{
    path::{Path, PathKind},
    name::Name,
    ids::{HirFileId, MacroCallId, MacroCallLoc, HirInterner},
    macros::{MacroDef, MacroInput, MacroExpansion},
    nameres::{ItemMap, PerNs, Namespace},
    ty::Ty,
    impl_block::{ImplBlock, ImplItem},
    docs::{Docs, Documentation},
    adt::AdtDef,
    expr::{ExprScopes, ScopesWithSyntaxMapping},
    resolve::{Resolver, Resolution},
    import::ImportResolver,
};

pub use self::code_model_api::{
    Crate, CrateDependency,
    Def,
    Module, ModuleDef, ModuleSource, Problem,
    Struct, Enum, EnumVariant,
    Function, FnSignature, ScopeEntryWithSyntax,
    StructField, FieldSource,
    Static, Const,
    Trait, Type,
};
