use crate::types::ast::Ast;

pub trait ConstType<N>
where
    N: From<i128> + From<f64>,
{
    fn eval(&self, c: &str) -> Ast<N>;
}

#[derive(Debug)]
pub struct PiConst {}
impl<N> ConstType<N> for PiConst
where
    N: From<i128> + From<f64>,
{
    fn eval(&self, c: &str) -> Ast<N> {
        Ast::Num(N::from(std::f64::consts::PI))
    }
}
