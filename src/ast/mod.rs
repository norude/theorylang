mod eval;
mod level0;
mod level1;
mod parser;

pub use parser::parser;

pub struct InitialLevel<'a>(Vec<level0::Top<'a>>);

impl std::fmt::Display for InitialLevel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for top in &self.0 {
            writeln!(f, "{top}")?;
        }
        Ok(())
    }
}

impl<'a> From<Vec<level0::Top<'a>>> for InitialLevel<'a> {
    fn from(value: Vec<level0::Top<'a>>) -> Self {
        Self(value)
    }
}

impl<'a> InitialLevel<'a> {
    pub fn lower_all_the_way(self) -> FinalLevel<'a> {
        let mut state1 = level1::State::default();
        let level1 = self
            .0
            .into_iter()
            .map(|top| state1.map_top(top))
            .collect::<Vec<_>>();
        FinalLevel(level1)
    }
}

#[derive(Debug)]
pub struct FinalLevel<'a>(Vec<level1::Top<'a>>);

impl FinalLevel<'_> {
    pub fn eval(self) {
        let mut state = eval::State::default();
        for top in self.0 {
            state.eval_top(top);
        }
        let result = state.eval_expr(level1::Expr::ProcCall {
            name: level0::GlobalSymbol(crate::common::Ident("main")),
            args: vec![],
        });
        println!("{result}");
    }
}
