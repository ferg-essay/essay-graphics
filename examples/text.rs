use essay_graphics::{prelude::*, artist::TextCoords};
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();
    let mut graph = figure.new_graph([1., 1.]);

    let x = linspace(0., 6.28, 20);
    let y = x.sin();

    graph.title("My Title"); // .color(0x008033).size(18.);
    graph.plot(&x, &y);

    graph.text((0., 1.), "hello").coord(TextCoords::FrameFraction).color("purple");

    figure.show();
}
