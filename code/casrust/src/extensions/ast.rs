use crate::types::ast::Ast;
use crate::types::NumberType;

impl<N> Ast<N>
where
    N: NumberType,
{
    pub fn expand(&self) -> Ast<N> {
        match self {
            Ast::Mul(vec) => {
                let mut result: Vec<Ast<N>> = vec![];
                let mut last_adders: Vec<Ast<N>> = vec![];
                // let mut none_adds = vec![];
                // let mut adds = vec![];
                // for node in vec.clone() {
                //     match node {
                //         Ast::Add(v_add) => {
                //             adds.push(v_add);
                //         }
                //         _ => {
                //             none_adds.push(node);
                //         }
                //     };
                // }

                // let none_adds = Ast::Mul(none_adds);

                // for add in adds {
                //     for el in add {
                //         // clone weil schleife mehrmals durchlaufen wird und ownserhip abgegeben wird
                //         result.push((el.clone() * none_adds.clone()).sort().clone());
                //     }
                // }

                for node in vec.clone() {
                    match node {
                        Ast::Add(v_add) => {
                            if result.is_empty() {
                                result.append(&mut v_add.clone());
                                last_adders.append(&mut v_add.clone());
                            } else {
                                println!("res: {:?}", result);
                                for el in v_add {
                                    println!("el to mut: {:?}", el);
                                    result = result.into_iter().map(|e| e * el.clone()).collect();
                                    println!("after: {:?}", result);
                                }
                            }
                        }
                        _ => {
                            if result.is_empty() {
                                result.push(node);
                            } else {
                                result = result.into_iter().map(|e| e * node.clone()).collect();
                            }
                        }
                    };
                }

                if result.len() == 1 {
                    result.pop().unwrap().sort().clone()
                } else {
                    Ast::Add(result).sort().clone()
                }
            }
            _ => self.clone(),
        }
    }
}
