use essay_graphics_api::{Path, PathCode, Point};
use essay_tensor::Tensor;

use crate::{
    artist::patch::PathPatch, 
    graph::{Graph, PlotOpt}, 
};

pub fn fill_between(
    graph: &mut Graph, 
    x: impl Into<Tensor>, 
    y1: impl Into<Tensor>, 
    y2: impl Into<Tensor>, 
) -> PlotOpt { // BarOpt {
    let x : Tensor = x.into();
    let y1 : Tensor = y1.into();
    let y2 : Tensor = y2.into();

    assert!(x.rank() == 1, "require rank-1 tensor {:?}", x.shape().as_slice());
    assert_eq!(x.shape(), y1.shape(), "require matching sizes x={:?} y1={:?}",
        x.shape().as_slice(), y1.shape().as_slice());
    assert_eq!(x.shape(), y2.shape(), "require matching sizes x={:?} y2={:?}",
        x.shape().as_slice(), y2.shape().as_slice());

    let mut vec = Vec::<PathCode>::new();

    vec.push(PathCode::MoveTo(Point(x[0], y1[0])));
    
    for (x, y) in x.iter().zip(y1.iter()).skip(1) {
        vec.push(PathCode::LineTo(Point(*x, *y)));
    }
    
    for (x, y) in x.iter().zip(y2.iter()).rev() {
        vec.push(PathCode::LineTo(Point(*x, *y)));
    }

    vec.push(PathCode::ClosePoly(Point(x[0], y2[0])));

    let patch = PathPatch::new(Path::new(vec));

    graph.add_simple_artist(patch)
}
