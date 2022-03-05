use rsmt2::{
    parse::{IdentParser, ModelParser},
    print::Expr2Smt,
    Solver,
};

use crate::symbolic_expr::SymbolicExpr;

#[derive(Copy, Clone)]
struct Parser;

impl<'a> IdentParser<String, String, &'a str> for Parser {
    fn parse_ident(self, input: &'a str) -> rsmt2::SmtRes<String> {
        Ok(input.into())
    }

    fn parse_type(self, input: &'a str) -> rsmt2::SmtRes<String> {
        Ok(input.into())
    }
}

impl<'a> ModelParser<String, String, String, &'a str> for Parser {
    fn parse_value(
        self,
        input: &'a str,
        _id: &String,
        _args: &[(String, String)],
        _out: &String,
    ) -> rsmt2::SmtRes<String> {
        Ok(input.into())
    }
}

impl Expr2Smt<()> for SymbolicExpr {
    fn expr_to_smt2<Writer>(&self, w: &mut Writer, _: ()) -> rsmt2::SmtRes<()>
    where
        Writer: std::io::Write,
    {
        todo!()
    }
}

impl SymbolicExpr {
    pub fn solve(&self) -> Result<(), rsmt2::errors::Error> {
        let mut solver = Solver::default_z3(Parser)?;

        solver.define_fun(
            "sq",
            &[("n", "Int")],
            "Int",
            "(* n n)", // fn sq        (n:   Int)  ->  Int   { n * n }
        )?;
        solver.declare_const("n", "Int")?;
        solver.declare_const("m", "Int")?;

        solver.assert("(= (+ (sq n) (sq m)) 29)")?;
        solver.assert("(and (< n 5) (> n 0) (> m 0))")?;

        if solver.check_sat()? {
            let mut model = solver.get_model()?;
            model.sort();
            println!("{:#?}", model);
        }
        Ok(())
    }
}
