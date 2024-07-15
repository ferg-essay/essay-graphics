use essay_graphics::{prelude::*, artist::{patch::PathPatch, paths}, frame::Data, graph::{Graph, PlotOpt}, layout::Figure};
use essay_graphics_api::{Point, PathCode, Path, Angle};

fn main() {
    //let mut gui = WgpuBackend::new();

    let mut figure = Figure::new();
    let mut graph = figure.new_graph([1., 1.]);

    let test = Tests::WEDGE;

    graph.add_simple_artist(PathPatch::new(test.path()));
    
    figure.show();
}
enum Tests {
    A, A_P,
    CIRCLE,
    WEDGE,
    SEMICIRCLE, SEMICIRCLE_T,
    CUT_BOX, CUT_BOX_B,
    HOLLOW_BOX, HOLLOW_BOX_BZ,
}

impl Tests {
    fn path(&self) -> Path<Data> {
        match self {
            Tests::A => todo!(),
            Tests::A_P => todo!(),
            Tests::CIRCLE => {
                paths::circle().transform(&Affine2d::eye())
            },
            Tests::WEDGE => {
                paths::wedge((Angle::Unit(0.0), Angle::Unit(0.25))).transform(&Affine2d::eye())
            },
            Tests::SEMICIRCLE => {
                Path::move_to(0.0, 0.)
                    .bezier2_to([0.5, 1.0], [1., 0.0])
                    .line_to(0.75, 0.)
                    .bezier2_to([0.5, 0.5], [0.25, 0.0])
                    .close_poly(0., 0.)
                    .to_path()
            },
            Tests::SEMICIRCLE_T => {
                Path::move_to(0.0, 0.)
                    .line_to(0.5, 1.0)
                    .line_to(1., 0.0)
                    .line_to(0.75, 0.)
                    .line_to(0.5, 0.5)
                    .line_to(0.25, 0.0)
                    .close_poly(0., 0.)
                    .to_path()
            },
            Tests::CUT_BOX => {
                Path::move_to(0.0, 0.)
                .line_to(0.25, 0.)
                .bezier2_to([0.5, 0.5], [0.75, 0.0])
                .line_to(1., 0.)
                .line_to(1., 1.)
                .close_poly(0., 1.)
                .to_path()
            },
            Tests::CUT_BOX_B => {
                Path::move_to(0.0, 0.)
                    .line_to(0., 1.)
                    .line_to(1., 1.)
                    .line_to(1., 0.)
                    .line_to(0.75, 0.)
                    .line_to(0.5, 0.5)
                    .line_to(0.25, 0.0)
                    .close_poly(0., 0.)
                    .to_path()
            },
            Tests::HOLLOW_BOX => {
                Path::move_to(0.0, 0.)
                .line_to(0., 1.)
                .line_to(1., 1.)
                .line_to(1., 0.)
                .close_poly(0., 0.)
                .move_to(0.25, 0.25)
                .line_to(0.25, 0.75)
                .line_to(0.75, 0.75)
                .line_to(0.75, 0.25)
                .close_poly(0.25, 0.25)
                .to_path()
            },
            Tests::HOLLOW_BOX_BZ => {
                Path::move_to(0.0, 0.)
                .line_to(0., 1.)
                .line_to(1., 1.)
                .line_to(1., 0.)
                .close_poly(0., 0.)
                .move_to(0.25, 0.25)
                .line_to(0.25, 0.75)
                .bezier2_to([0.5, 1.], [0.75, 0.75])
                .line_to(0.75, 0.25)
                .close_poly(0.25, 0.25)
                .to_path()
            },
        }
    }
}

pub fn bezier2(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::Bezier2(p1.into(), p2.into()),
    ])))
}

pub fn bezier2_poly(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
) -> PlotOpt {
    let p0 = p0.into();

    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0),
        PathCode::Bezier2(p1.into(), p2.into()),
        PathCode::ClosePoly(p0),
    ])))
}

pub fn plot_quad(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::LineTo(p1.into()),
        PathCode::LineTo(p2.into()),
        PathCode::ClosePoly(p3.into()),
    ])))
}

pub fn plot_line(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>,
    p4: impl Into<Point>,
    p5: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::LineTo(p1.into()),
        PathCode::MoveTo(p2.into()),
        PathCode::LineTo(p3.into()),
        PathCode::MoveTo(p4.into()),
        PathCode::LineTo(p5.into()),
    ])))
}
