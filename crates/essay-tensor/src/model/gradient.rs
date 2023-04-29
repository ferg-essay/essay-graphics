use crate::{Tensor};

use super::Var;

pub struct Tape {
}

impl Tape {
    pub fn new() -> Tape {
        Self {

        }
    }

    pub fn var(&mut self, var: &Var) -> Tensor {
        println!("Var: {:?}", var);
        var.tensor()
    }

    fn gradient(
        &self, 
        loss: &Tensor, 
        var: &Var
    ) -> Tensor {
        println!("var {:?}", var);
        println!("loss {:?}", loss);
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::model::Var;
    use crate::model::gradient::Tape;
    use crate::{Tensor, random::uniform};
    use crate::prelude::{*};

    #[test]
    fn test() {
        let w = Var::new("w", tensor!(0.5));
        let b = Var::new("b", tensor!(0.5));

        let mut tape = Tape::new();
        let w_t = tape.var(&w);
        let b_t = tape.var(&b);

        let x = tensor!(0.0);

        let z = x.clone() * w_t.clone() + b_t;

        let y : Tensor = tensor!(2.0) * x + 1.0.into();
        let loss: Tensor = z.mean_square_error(&y);

        println!("w_t {:#?}", &w_t);
        println!("{:#?} loss {:#?}", &z, &loss);

        let dw = tape.gradient(&loss, &w);

        println!("w_t {:#?}", &w_t);
        println!("{:#?} loss {:#?}", &z, &loss);
        println!("dw {:#?}", &dw);
    }
}