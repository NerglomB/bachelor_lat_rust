use crate::types::ast::Ast;

/// Returns pi as value.
pub fn pi_const<N>() -> Ast<N>
where
    N: From<f64>,
{
    Ast::Num(N::from(std::f64::consts::PI))
}

/// Returns infinity as value.
pub fn infinity_const<N>() -> Ast<N> {
    Ast::Const("∞".to_owned())
}
