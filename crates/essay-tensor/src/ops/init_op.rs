use core::fmt;
use std::{any::type_name};

use crate::{function::{IntoForward, NodeOp, Tape, Operation, Graph, graph::GradientOp}, Tensor, 
    tensor::{Dtype, TensorUninit, TensorId}, prelude::Shape
};

pub trait InitKernel<D:Dtype> : fmt::Debug + Clone + PartialEq + Sync + Send + 'static
{
    type State : Default;

    fn f(&self, state: &mut Self::State) -> D;
}

#[derive(Clone, PartialEq)]
pub struct InitCpu<Op:InitKernel<f32>>(Op, Shape);

pub fn init_op<Op>(op: Op, shape: impl Into<Shape>) -> Tensor
where
    Op: InitKernel<f32>
{
    let shape = shape.into();

    let uop = InitCpu(op.clone(), shape);

    let id = NodeOp::new(&[], uop.to_op());

    let tensor = uop.f(&[], id);

    Tape::set_tensor(tensor)
}

impl<Op:InitKernel<f32>> Operation for InitCpu<Op> {
    fn name(&self) -> &str {
        type_name::<Op>()
    }
    
    fn f(
        &self,
        _args: &[&Tensor],
        id: TensorId,
    ) -> Tensor {
        let shape = self.shape();
        let len = shape.size();
    
        unsafe {
            let mut out = TensorUninit::<f32>::new(len);
    
            let op = &self.0;
            let o_ptr = out.as_mut_ptr();

            let mut state = Op::State::default();
        
            for i in 0..len {
                *o_ptr.add(i) = op.f(&mut state);
            }
    
            Tensor::from_uninit_with_id(out, shape, id)
        }
    }

    fn df(
        &self,
        _forward: &Graph,
        graph: &mut Graph,
        i: usize,
        args: &[TensorId],
        prev: TensorId,
    ) -> TensorId {
        assert!(i == 0);

        graph.add_grad_op(self.clone(), &[args[0]], prev)
    }
}

impl<Op: InitKernel<f32>> GradientOp for InitCpu<Op> {
    fn name(&self) -> &str {
        type_name::<Op>()
    }

    fn df(
        &self,
        args: &[&Tensor],
        prev: &Tensor,
    ) -> Tensor {
        todo!()
    }
}


impl<Op: InitKernel<f32>> InitCpu<Op> {
    fn shape(&self) -> &Shape {
        &self.1
    }
}
