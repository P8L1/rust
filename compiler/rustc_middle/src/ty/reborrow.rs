use rustc_abi::FieldIdx;
use rustc_hir::def::CtorKind;
use rustc_span::{Span, Symbol};

use crate::ty::{self, Ty, TyCtxt};

#[derive(Clone, Copy, Debug)]
pub struct ReborrowField<'tcx> {
    pub index: FieldIdx,
    pub name: Symbol,
    pub ty: Ty<'tcx>,
    pub span: Span,
}

#[derive(Clone, Copy, Debug)]
pub struct CoerceSharedFieldPair<'tcx> {
    pub source: ReborrowField<'tcx>,
    pub target: ReborrowField<'tcx>,
}

#[derive(Clone, Copy, Debug)]
pub enum CoerceSharedFieldPairError<'tcx> {
    FieldStyleMismatch,
    MissingSourceField { target: ReborrowField<'tcx> },
}

pub fn single_lifetime_arg<'tcx>(args: ty::GenericArgsRef<'tcx>) -> Option<ty::Region<'tcx>> {
    let mut lifetimes = args.iter().filter_map(|arg| arg.as_region());
    let lifetime = lifetimes.next()?;
    lifetimes.next().is_none().then_some(lifetime)
}

/// Returns the instantiated non-`PhantomData` fields that participate in generic reborrows.
pub fn reborrow_data_fields<'tcx>(
    tcx: TyCtxt<'tcx>,
    def: ty::AdtDef<'tcx>,
    args: ty::GenericArgsRef<'tcx>,
) -> Vec<ReborrowField<'tcx>> {
    def.non_enum_variant()
        .fields
        .iter_enumerated()
        .filter_map(|(index, field)| {
            let ty = field.ty(tcx, args).skip_norm_wip();
            (!ty.is_phantom_data()).then_some(ReborrowField {
                index,
                name: field.name,
                ty,
                span: tcx.def_span(field.did),
            })
        })
        .collect()
}

/// Canonical CoerceShared source-to-target field correspondence.
///
/// Named structs match fields by name. Tuple structs match fields by position. `PhantomData`
/// fields are ignored on both sides, so a target data field must correspond to a source data field.
/// The field types are instantiated but intentionally not normalized here: validation, borrowck,
/// interpretation, and codegen each need to normalize or relate aliases with their phase-specific
/// inference or monomorphization context before deciding whether to copy or recurse.
pub fn coerce_shared_field_pairs<'tcx>(
    tcx: TyCtxt<'tcx>,
    source_def: ty::AdtDef<'tcx>,
    source_args: ty::GenericArgsRef<'tcx>,
    target_def: ty::AdtDef<'tcx>,
    target_args: ty::GenericArgsRef<'tcx>,
) -> Result<Vec<CoerceSharedFieldPair<'tcx>>, CoerceSharedFieldPairError<'tcx>> {
    let source_variant = source_def.non_enum_variant();
    let target_variant = target_def.non_enum_variant();
    if source_variant.ctor_kind() != target_variant.ctor_kind() {
        return Err(CoerceSharedFieldPairError::FieldStyleMismatch);
    }

    let by_index = matches!(target_variant.ctor_kind(), Some(CtorKind::Fn));
    let source_fields = reborrow_data_fields(tcx, source_def, source_args);
    reborrow_data_fields(tcx, target_def, target_args)
        .into_iter()
        .map(|target| {
            let source = source_fields
                .iter()
                .copied()
                .find(|source| {
                    if by_index { source.index == target.index } else { source.name == target.name }
                })
                .ok_or(CoerceSharedFieldPairError::MissingSourceField { target })?;

            Ok(CoerceSharedFieldPair { source, target })
        })
        .collect()
}
