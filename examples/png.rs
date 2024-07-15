use essay_graphics::{prelude::*, artist::{patch::PathPatch, Markers, IntoMarker, DrawStyle}, plot::{bar, grid_color, contour, triplot, tricontour, matshow, stem, fill_between, plot}, graph::{Graph, PlotOpt}};
use essay_graphics_api::{Point, Color, PathCode, Path, JoinStyle, CapStyle, LineStyle, Angle};
use essay_tensor::{prelude::*, init::{linspace, meshgrid, meshgrid_ij}, tensor::TensorVec};

fn main() {
    //let mut gui = WgpuBackend::new();

    let mut figure = Figure::new();
    let mut graph = figure.new_graph([1., 1.]);

    //let x = linspace(0., 2. * PI, 30);
    //let y = x.sin();

    let x = linspace(0., 6.28, 20);
    let y = x.sin();

    graph.title("My Title"); // .color(0x008033).size(18.);
    graph.plot(&x, &y);

    // figure.show();
    figure.save("test.png", 200.);
}
