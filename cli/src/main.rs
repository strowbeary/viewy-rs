use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use syn::{
    Expr, ExprBlock, ExprMethodCall, ExprPath, Ident, ItemFn, Local, Pat, PatIdent, Stmt,
    token::Eq,
    visit_mut::{self, VisitMut},
};

struct ClosureTransformer;

impl VisitMut for ClosureTransformer {
    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        visit_mut::visit_expr_method_call_mut(self, node);

        if let Some(last_arg) = node.args.last_mut() {
            if let Expr::Block(ExprBlock { block, .. }) = last_arg {
                if block.stmts.len() == 1 {
                    if let Stmt::Expr(expr, _) = &block.stmts[0] {
                        *last_arg = expr.clone();
                    }
                }
            }
        }
    }

    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) {
        let block = node.block.as_mut();

        if block.stmts.len() == 1 {
            if let Stmt::Expr(Expr::MethodCall(method_call), _) = &block.stmts[0] {
                let var_ident = Ident::new("output", proc_macro2::Span::call_site());

                // Construire let mut output = <expr>;
                let let_stmt = Stmt::Local(Local {
                    attrs: vec![],
                    let_token: Default::default(),
                    pat: Pat::Ident(PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: Some(Default::default()),
                        ident: var_ident.clone(),
                        subpat: None,
                    }),
                    init: Some((
                        Eq {
                            spans: [proc_macro2::Span::call_site()],
                        },
                        Box::new(Expr::MethodCall(method_call.clone())),
                    )),
                    semi_token: Default::default(),
                });

                // Remplacer les statements par let ...; output
                block.stmts = vec![
                    let_stmt,
                    Stmt::Expr(
                        Expr::Path(ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: var_ident.into(),
                        }),
                        None,
                    ),
                ];
            }
        }
    }
}

fn process_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    let mut syntax = syn::parse_file(&content)?;

    let mut transformer = ClosureTransformer;
    transformer.visit_file_mut(&mut syntax);

    let formatted = prettyplease::unparse(&syntax);

    fs::write(path, formatted)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new("./src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        process_file(entry.path())?;
    }
    Ok(())
}
