pub mod level0; // what we get after parsing
pub mod level1;
pub mod level2;

pub trait Lower<'a> {
    fn lower_all_the_way(self) -> level2::Value<'a>;
}

impl<'a> Lower<'a> for level0::Expr<'a> {
    fn lower_all_the_way(self) -> level2::Value<'a> {
        let mut state1 = level1::State::default();
        let level1 = self.map(&mut state1);
        let mut state2 = level2::State::default();
        level1.map(&mut state2)
    }
}
