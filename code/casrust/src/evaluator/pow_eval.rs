use crate::types::{ast::Ast, NumberType};

pub fn perfect_nth_root<N>(base: &Ast<N>, exp: &Ast<N>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    // newtons method for nthroot reicht für meine zwecke von sympy übernommen in
    // # Get initial estimate for Newton's method. Care must be taken to
    // # avoid overflow
    // try:
    //     guess = int(y**(1./n) + 0.5)
    // except OverflowError:
    //     exp = _log(y, 2)/n
    //     if exp > 53:
    //         shift = int(exp - 53)
    //         guess = int(2.0**(exp - shift) + 1) << shift
    //     else:
    //         guess = int(2.0**exp)
    // if guess > 2**50:
    //     # Newton iteration
    //     xprev, x = -1, guess
    //     while 1:
    //         t = x**(n - 1)
    //         xprev, x = x, ((n - 1)*x + y//t)//n
    //         if abs(x - xprev) < 2:
    //             break
    // else:
    //     x = guess
    // # Compensate
    // t = x**n
    // while t < y:
    //     x += 1
    //     t = x**n
    // while t > y:
    //     x -= 1
    //     t = x**n
    // return int(x), t == y <- Wert und ist exakt?, siehe power.py
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

pub fn pow_mul<N>(base: &Ast<N>, exp: &Ast<N>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    let mut ret_val = None;
    if let Ast::Mul(vec) = base {
        // We can assume that a number in a mul is on the last index
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
