use essay_graphics::{artist::patch::Patch, frame::Data, layout::Figure, prelude::*};

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
    graph.artist(Patch::new(path));

    //graph.artist(Patch::new(path)).color("teal").edge_color("black");

    //graph.aspect(1.);
    //graph.xlim(0., 20.);
    //graph.ylim(0., 20.);

    figure.show();
}
