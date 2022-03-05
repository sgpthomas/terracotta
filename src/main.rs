mod smt;
mod symbolic_expr;

use crate::symbolic_expr::SymbolicExpr;

fn if_expr_test(a: SymbolicExpr, b: SymbolicExpr) -> SymbolicExpr {
    SymbolicExpr::ite(a.lt(10.into()), || b.clone() + 1, || b.clone() - 1)
}

fn main() {
    let v = if_expr_test("a".into(), "b".into());
    v.solve().unwrap();
    println!("{}", v);
}
