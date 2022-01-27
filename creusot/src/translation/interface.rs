use std::borrow::Cow;

use why3::{
    declaration::{Contract, Decl, Module, TyDeclKind, ValKind},
    Ident,
};

use crate::{clone_map::CloneMap, ctx::*, util};

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{TyCtxt, TyKind};

use super::{function::closure_generic_decls, ty::translate_closure_ty};

pub fn interface_for(
    ctx: &mut TranslationCtx<'_, 'tcx>,
    def_id: DefId,
) -> (Module, CloneMap<'tcx>) {
    let mut names = CloneMap::new(ctx.tcx, def_id, false);
    names.clone_self(def_id);
    let mut sig = util::signature_of(ctx, &mut names, def_id);
    sig.contract.variant = Vec::new();

    let mut decls: Vec<_> = closure_generic_decls(ctx.tcx, def_id).collect();

    if ctx.tcx.is_closure(def_id) {
        if let TyKind::Closure(_, subst) = ctx.tcx.type_of(def_id).kind() {
            let mut tydecl = translate_closure_ty(ctx, def_id, subst);
            tydecl.kind = TyDeclKind::Opaque;
            decls.push(Decl::TyDecl(tydecl))
        }
    }

    decls.extend(names.to_clones(ctx));

    match util::item_type(ctx.tcx, def_id) {
        ItemType::Predicate => {
            sig.retty = None;
            sig.contract = Contract::new();
            decls.push(Decl::ValDecl(ValKind::Predicate { sig }));
        }
        ItemType::Logic => {
            sig.contract = Contract::new();
            decls.push(Decl::ValDecl(ValKind::Function { sig }));
        }
        _ => {
            if !def_id.is_local() && !ctx.externs.verified(def_id) {
                sig.contract.requires.push(why3::mlcfg::Exp::mk_false());
            }

            decls.push(Decl::ValDecl(ValKind::Val { sig }));
        }
    }

    let name = interface_name(ctx.tcx, def_id);

    (Module { name, decls }, names)
}

pub fn interface_name(tcx: TyCtxt, def_id: DefId) -> Ident {
    format!("{}_Interface", Cow::from(&*module_name(tcx, def_id))).into()
}
