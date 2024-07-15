use essay_graphics::{prelude::*, artist::patch::Patch};
use path_opt::Hatch;

fn main() { 
    let mut figure = Figure::new();
    let mut graph = figure.new_graph(());

    let patch = Patch::new(Path::move_to(0., 0.)
        .line_to(1., 0.)
        .close_poly(1., 1.));
    graph.artist(patch)
        .color("purple")
        .edge_color("black")
        .rotate(Angle::Unit(0.1))
        .scale(0.5)
        .hatch(Hatch::Horizontal);

    let patch2 = Patch::new(Path::move_to(0.3, 0.5)
        .line_to(1., 1.0)
        .close_poly(1., 0.5));

    graph.artist(patch2)
        .color("teal")
        .hatch(Hatch::Horizontal);
    
    figure.show();
}
