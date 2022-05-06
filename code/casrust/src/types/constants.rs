use crate::types::ast::Ast;

pub fn pi_const<N>() -> Ast<N>
where
    N: From<f64>,
{
    Ast::Num(N::from(std::f64::consts::PI))
}
