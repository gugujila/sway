use crate::{language::ty, monomorphize::priv_prelude::*, type_system::*};

pub(crate) fn gather_from_exp(ctx: GatherContext, exp: &ty::TyExpression) {
    gather_from_exp_inner(ctx, &exp.expression, exp.return_type)
}

pub(crate) fn gather_from_exp_inner(
    ctx: GatherContext,
    exp: &ty::TyExpressionVariant,
    return_type: TypeId,
) {
    gather_from_ty(ctx, return_type);
    match exp {
        ty::TyExpressionVariant::FunctionApplication {
            arguments,
            contract_call_params,
            call_path,
            fn_ref,
            ..
        } => {
            arguments
                .iter()
                .for_each(|(_, arg)| gather_from_exp(ctx, arg));
            contract_call_params
                .iter()
                .for_each(|(_, arg)| gather_from_exp(ctx, arg));
            ctx.add_constraint(Constraint::mk_fn_call(call_path, arguments, fn_ref));
        }
        ty::TyExpressionVariant::LazyOperator { lhs, rhs, .. } => {
            gather_from_exp(ctx, lhs);
            gather_from_exp(ctx, rhs);
        }
        ty::TyExpressionVariant::VariableExpression { .. } => {
            // NOTE: may need to do something here later
        }
        ty::TyExpressionVariant::Tuple { fields } => {
            fields.iter().for_each(|field| gather_from_exp(ctx, field));
        }
        ty::TyExpressionVariant::Array {
            contents: _,
            elem_type: _,
        } => {
            todo!();
            // contents
            //     .iter()
            //     .for_each(|elem| gather_from_exp(ctx, elem));
        }
        ty::TyExpressionVariant::ArrayIndex {
            prefix: _,
            index: _,
        } => {
            todo!();
            // gather_from_exp(ctx, prefix);
            // gather_from_exp(ctx, index);
        }
        ty::TyExpressionVariant::StructExpression { .. } => todo!(),
        ty::TyExpressionVariant::CodeBlock(block) => {
            gather_from_code_block(ctx, block);
        }
        ty::TyExpressionVariant::IfExp { .. } => todo!(),
        ty::TyExpressionVariant::MatchExp { .. } => todo!(),
        ty::TyExpressionVariant::AsmExpression { .. } => todo!(),
        ty::TyExpressionVariant::StructFieldAccess { .. } => todo!(),
        ty::TyExpressionVariant::TupleElemAccess { prefix, .. } => {
            gather_from_exp(ctx, prefix);
        }
        ty::TyExpressionVariant::EnumInstantiation { .. } => todo!(),
        ty::TyExpressionVariant::AbiCast { .. } => todo!(),
        ty::TyExpressionVariant::StorageAccess(_) => todo!(),
        ty::TyExpressionVariant::IntrinsicFunction(_) => todo!(),
        ty::TyExpressionVariant::AbiName(_) => todo!(),
        ty::TyExpressionVariant::EnumTag { exp } => {
            gather_from_exp(ctx, exp);
        }
        ty::TyExpressionVariant::UnsafeDowncast { .. } => todo!(),
        ty::TyExpressionVariant::WhileLoop { .. } => todo!(),
        ty::TyExpressionVariant::Reassignment(_) => todo!(),
        ty::TyExpressionVariant::StorageReassignment(_) => todo!(),
        ty::TyExpressionVariant::Return(exp) => {
            gather_from_exp(ctx, exp);
        }
        ty::TyExpressionVariant::Literal(_) => {}
        ty::TyExpressionVariant::Break => {}
        ty::TyExpressionVariant::Continue => {}
        ty::TyExpressionVariant::FunctionParameter => {}
    }
}
