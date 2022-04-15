use terracotta::symbolic_expr::SymbolicExpr;

enum Logic<L> {
    And(Box<Logic<L>>, Box<Logic<L>>),
    Or(Box<Logic<L>>, Box<Logic<L>>),
    Not(Box<Logic<L>>),
    Lit(L),
}

// impl Context for HashMap<String, bool> {
//     type Key = String;
//     type Value = bool;

//     fn lookup(&self, key: &Self::Key) -> Self::Value {
//         self[key]
//     }
// }

impl Logic<bool> {
    fn interpret(&self) -> bool {
        match self {
            Logic::And(a, b) => a.interpret() & b.interpret(),
            Logic::Or(a, b) => a.interpret() | b.interpret(),
            Logic::Not(v) => !v.interpret(),
            Logic::Lit(x) => x.clone(),
        }
    }
}

impl Logic<SymbolicExpr> {
    fn interpret(&self) -> SymbolicExpr {
        match self {
            Logic::And(a, b) => a.interpret() & b.interpret(),
            Logic::Or(a, b) => a.interpret() | b.interpret(),
            Logic::Not(v) => !v.interpret(),
            Logic::Lit(x) => x.clone(),
        }
    }
}

fn main() {
    let expr: Logic<bool> = Logic::And(Box::new(Logic::Lit(true)), Box::new(Logic::Lit(false)));
    println!("{}", expr.interpret());

    let sym_expr: Logic<SymbolicExpr> = Logic::And(
        Box::new(Logic::Lit(SymbolicExpr::Var("a".into()))),
        Box::new(Logic::Lit(SymbolicExpr::Var("b".into()))),
    );
    println!("{}", sym_expr.interpret());

    let a: SymbolicExpr = "a".into();
}
