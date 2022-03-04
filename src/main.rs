mod symbolic_expr;

use crate::symbolic_expr::SymbolicExpr;

fn main() {
    let a: SymbolicExpr = "a".into();
    println!("{}", (SymbolicExpr::Int(2) + 2) / a.clone() * a);
}
