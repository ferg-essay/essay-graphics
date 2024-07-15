use essay_graphics::{prelude::*, plot::grid_color, artist::{Shading, patch::{Patch, self}}, frame::{AspectMode, Data}};
use essay_tensor::init::{linspace, meshgrid};

fn main() { 
    let mut figure = Figure::new();
    let mut graph = figure.new_graph(());

    let path = Path::<Data>::new(vec![
        PathCode::MoveTo(Point(1., 1.)),
        PathCode::LineTo(Point(11., 11.)),
        PathCode::LineTo(Point(6., 11.)),
        PathCode::LineTo(Point(7.7, 10.)),
        PathCode::ClosePoly(Point(0., 2.)),
    ]);
    graph.artist(Patch::new(path)).color("teal").edge_color("black");
    //let patch = patch::arrow((4., 1.), (0.707, 0.707));
    //graph.artist(patch).color("teal").edge_color("black");

    // TODO: triangulation bug
    let patch = patch::arrow((6., 1.), (-0.707, 0.707));
    //graph.artist(patch).color("black");

    graph.aspect(1.);
    graph.xlim(0., 20.);
    graph.ylim(0., 20.);
    //graph.aspect_mode(AspectMode::View);

    figure.show();
}
