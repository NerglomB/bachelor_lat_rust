use crate::types::{ast::Ast, NumberType};

/// Tries to find a perfect root for given base and exponent. Algorithm is based on the Newton procedure.
pub fn perfect_nth_root<N>(base: &Ast<N>, exp: &Ast<N>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    let mut ret_val: Option<Ast<N>> = None;

    if *hard_eval {
        if let (Ast::Num(v1), Ast::Num(v2)) = (base, exp) {
            ret_val = Some(Ast::Num(v1.pow(v2.clone())));
        }
    } else {
        if let (Ast::Num(v1), Ast::Num(v2)) = (base, exp) {
            if v1.is_integer() && v2.is_rational() {
                let (num, den, b): (i128, i128, i128) = (
                    v2.get_numerator().into(),
                    v2.get_denominator().into(),
                    v1.get_numerator().into(),
                );
                if num == 1 {
                    let guess = ((b as f64).powf(1.0 / den as f64) + 0.5) as u32;

                    let mut x = guess as f64;
                    if guess as u64 > u64::pow(2, 50) {
                        loop {
                            let t = x.powi((den - 1) as i32);
                            let xprev = x;
                            x = (((den as i32 - 1) as f64 * x + b as f64) as f64 / t as f64)
                                / den as f64;
                            if (x - xprev).abs() < 2.0 {
                                break;
                            }
                        }
                    }

                    let mut t = x.powi(den as i32);
                    while t < b as f64 {
                        x += 1.0;
                        t = x.powi(den as i32);
                    }
                    while t > b as f64 {
                        x -= 1.0;
                        t = x.powi(den as i32);
                    }

                    let x = x as u32;
                    if t == b as f64 {
                        ret_val = Some(Ast::Num(N::from(x as i128)));
                    }
                }
            }
        }
    }

    ret_val
}

/// Tries to simplify the numerical value of a power operation where the base is a multiplication.
pub fn pow_mul<N>(base: &Ast<N>, exp: &Ast<N>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    let mut ret_val = None;
    if let Ast::Mul(vec) = base {
        let vec0 = &vec[vec.len() - 1];
        match vec0 {
            Ast::Num(_) => {
                if let Some(check_root) = perfect_nth_root(vec0, &exp, hard_eval) {
                    let mut cloned_vec = vec.clone();
                    cloned_vec.remove(vec.len() - 1);
                    ret_val = Some(Ast::Mul(vec![
                        check_root,
                        Ast::Pow(Box::new(Ast::Mul(cloned_vec)), Box::new(exp.clone())),
                    ]));
                }
            }
            _ => {}
        };
    }

    ret_val
}
