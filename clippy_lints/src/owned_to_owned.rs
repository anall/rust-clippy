use crate::utils::{span_lint_and_sugg, is_copy, match_def_path, match_type, paths, sugg};
use rustc_errors::Applicability;
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_hir::{Expr,ExprKind};
use rustc_middle::ty::TyS;
use rustc_span::symbol::Symbol;

use if_chain::if_chain;

declare_clippy_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// let a = vec![1, 2, 3];
    /// let b = a.to_vec();
    /// let c = a.to_owned();
    /// ```
    /// Use instead:
    /// ```rust
    /// let a = vec![1, 2, 3];
    /// let b = a.clone();
    /// let c = a.clone();
    /// ```
    pub OWNED_TO_OWNED,
    style,
    "using to_owned on already owned type"
}

declare_lint_pass!(OwnedToOwned => [OWNED_TO_OWNED]);

const EQUIVALENT_TO_CLONE : [(&[&str],&str); 2] = [
    (&["alloc","vec","Vec"],"to_vec"),
    (&["std","path","PathBuf"],"to_path_buf"),
];

impl LateLintPass<'_> for OwnedToOwned {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let ExprKind::MethodCall(method_path, _, args, _) = &expr.kind;
            if let Some(meth_did) = cx.typeck_results().type_dependent_def_id(expr.hir_id);
            let arg0 = &args[0]; // FIXME: Is this safe? Should I do if let Some(arg0) = ... instead?
            if let Some(snippet) = sugg::Sugg::hir_opt(cx, arg0);
            let return_type = cx.typeck_results().expr_ty( &expr );
            let input_type = cx.typeck_results().expr_ty( arg0 );
            if TyS::same_type( return_type, input_type );
            if match_def_path(cx, meth_did, &paths::TO_OWNED_METHOD) || EQUIVALENT_TO_CLONE.iter().any(|(type_path,sym)| {
                match_type(cx,input_type,type_path) && method_path.ident.name == Symbol::intern(sym)
            });
            then {
                if is_copy(cx,return_type) {
                    // if return_type is copy, have the suggestion be to replace the call with the variable itself
                    //  this prevents fixing this lint, only to have clone_on_copy complain next.
                    span_lint_and_sugg(
                        cx,OWNED_TO_OWNED,expr.span,
                        &format!("using `{}` on an already-owned type",method_path.ident.name),
                        "replace this with",
                        snippet.to_string(),
                        Applicability::MaybeIncorrect
                    );
                } else {
                    span_lint_and_sugg(
                        cx,OWNED_TO_OWNED,method_path.ident.span,
                        &format!("using `{}` on an already-owned type",method_path.ident.name),
                        "replace this with",
                        "clone".to_string(),
                        Applicability::MaybeIncorrect
                    );
                }
            }
        }
    }
}

