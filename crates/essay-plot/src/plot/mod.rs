use essay_tensor::Tensor;

use crate::{graph::{Graph, Data}, artist::Artist};

mod pie;
mod scatter;
mod lineplot;

pub use lineplot::{
    plot, 
};

pub use pie::{
    pie, 
};

pub use scatter::{
    scatter, 
};

impl Graph {
    pub fn plot(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> &mut Artist<Data> {
        lineplot::plot(self, x, y)
    }

    pub fn scatter(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
    ) -> &mut Artist<Data> {
        scatter::scatter(self, x, y)
    }

    pub fn pie(
        graph: &mut Graph, 
        x: impl Into<Tensor>, 
    ) { // -> &mut Artist {
        pie::pie(graph, x)
    }
}
