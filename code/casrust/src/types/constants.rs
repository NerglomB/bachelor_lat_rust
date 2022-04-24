use crate::types::ast::Ast;

pub trait ConstType<N>
where
    N: From<i128> + From<f64>,
{
    fn is_const(&self, c: &str) -> bool;

    fn eval(&self, c: &str) -> Option<Ast<N>>;
}

#[derive(Debug)]
pub struct PiConst {}
impl<N> ConstType<N> for PiConst
where
    N: From<i128> + From<f64>,
{
    fn is_const(&self, c: &str) -> bool {
        c == "π"
    }

    fn eval(&self, c: &str) -> Option<Ast<N>> {
        if c == "π" {
            Some(Ast::Num(N::from(std::f64::consts::PI)))
        } else {
            None
        }
    }
}
