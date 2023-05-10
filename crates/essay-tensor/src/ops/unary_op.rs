use core::fmt;
use std::{sync::Arc, any::type_name};

use crate::{module::{IntoForward, NodeOp, Tape, ForwardOp, Graph, TensorId, graph::BackOp}, Tensor, 
    tensor::{Dtype, TensorUninit, NodeId}
};

pub trait Uop<D:Dtype> : fmt::Debug + Copy + Clone + PartialEq + Sync + Send + 'static
{
    fn f(&self, value: D) -> D;

    fn df_dx(&self, value: D) -> D;
}

#[derive(Clone, PartialEq)]
pub struct UopImpl<Op:Uop<f32>>(Op);

pub fn unary_op<Op>(a: &Tensor, op: Op) -> Tensor
where
    Op:Uop<f32>
{
    let uop = UopImpl(op.clone());

    let node = NodeOp::new(&[a], uop.to_op());

    let tensor = uop.eval(&[&a], node);

    Tape::set_tensor(tensor)
}

impl<Op:Uop<f32>> ForwardOp for UopImpl<Op> {
    fn name(&self) -> &str {
        type_name::<Op>()
    }
    
    fn eval(
        &self,
        args: &[&Tensor],
        node: NodeId,
    ) -> Tensor {
        let a = args[0];
        let len = a.len();
    
        let data = unsafe {
            let mut out = TensorUninit::<f32>::new(len);
    
            let op = &self.0;
            let a_ptr = a.data().as_ptr();
            let o_ptr = out.as_mut_ptr();
        
            for i in 0..len {
                *o_ptr.add(i) = op.f(*a_ptr.add(i));
            }
    
            out.init()
        };
        
        let shape = a.shape().clone();
        Tensor::new_op(Arc::new(data), shape, node)
    }

    fn backprop(
        &self,
        _forward: &Graph,
        graph: &mut Graph,
        i: usize,
        args: &[TensorId],
        prev: TensorId,
    ) -> TensorId {
        assert!(i == 0);

        graph.add_back_op(self.clone(), &[args[0]], prev)
    }
}

impl<Op:Uop<f32>> BackOp for UopImpl<Op> {
    fn name(&self) -> &str {
        type_name::<Op>()
    }

    fn df(
        &self,
        args: &[&Tensor],
        prev: &Tensor,
    ) -> Tensor {
        let tensor = &args[0];
        let buffer = tensor.data();
        let prev = prev.data();
        let len = buffer.len();
        
        let data = unsafe {
            let mut data = TensorUninit::<f32>::new(len);

            let op = &self.0;
        
            for i in 0..len {
                let df_dx = op.df_dx(buffer.get_unchecked(i));
                let prev_df = prev.get_unchecked(i);

                data.set_unchecked(i, df_dx * prev_df);
            }
    
            data.init()
        };
        
        let shape = tensor.shape().clone();
        Tensor::new(data, &shape)
    }
}