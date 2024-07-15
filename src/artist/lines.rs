use core::fmt;

use essay_graphics_api::{
    Bounds, Point, Canvas, Path, PathCode, PathOpt,
    driver::Renderer, Clip
};
use essay_tensor::Tensor;

use crate::{
    artist::PathStyle, 
    frame::{Data, LegendHandler}, 
    data_artist_option_struct, path_style_options, graph::ConfigArc
};

use super::{Artist, PlotArtist, PlotId, markers::{MarkerStyle, IntoMarker}, PathCollection, ToCanvas};

#[derive(Clone, PartialEq, Debug)]
pub enum DrawStyle {
    Default,
    StepsPre,
    StepsMid,
    StepsPost,
}

pub struct Lines2d {
    xy: Tensor, // 2d tensor representing a graph
    path: Path<Data>,

    style: PathStyle,
    label: Option<String>,

    marker: Option<MarkerStyle>,
    collection: Option<PathCollection>,

    draw_style: DrawStyle,

    is_visible: bool,
    z_order: f32,

    extent: Bounds<Data>,
    is_stale: bool,
}

impl Lines2d {
    pub fn from_xy(x: impl Into<Tensor>, y: impl Into<Tensor>) -> Self {
        let x = x.into();
        let y = y.into();

        assert_eq!(x.len(), y.len());

        let lines = x.stack([y], -1);

        let path = build_path(&lines, &DrawStyle::Default);

        Self {
            xy: lines,
            path,
            style: PathStyle::new(),

            label: None,
            marker: None,
            collection: None,

            draw_style: DrawStyle::Default,

            is_visible: true,
            z_order: 0.,

            is_stale: true,

            extent: Bounds::<Data>::none(),
        }
    }

    pub fn set_xy(&mut self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> &mut Self {
        let x = x.into();
        let y = y.into();

        assert_eq!(x.len(), y.len());

        let xy = x.stack([y], -1);

        let path = build_path(&xy, &self.draw_style);

        self.xy = xy;
        self.path = path;

        if let Some(marker) = &self.marker {
            self.collection = Some(PathCollection::new(marker.get_path(), &self.xy));
        }

        self.is_stale = true;

        self
    }

    pub fn marker(&mut self, marker: impl IntoMarker) -> &mut Self {
        let marker = marker.into_marker();

        self.collection = Some(PathCollection::new(marker.get_path(), &self.xy));

        self.marker = Some(marker);
        self.is_stale = true;

        self
    }

    pub fn draw_style(&mut self, draw_style: DrawStyle) -> &mut Self {
        self.draw_style = draw_style;
        self.path = build_path(&self.xy, &self.draw_style);
        self.is_stale = true;

        self
    }
}

fn build_path(line: &Tensor, draw_style: &DrawStyle) -> Path<Data> {
    let mut codes = Vec::<PathCode>::new();
    codes.reserve(line.rows());
    
    let mut is_active = false;
    let (mut prev_x, mut prev_y) = (0.0f32, 0.0f32);

    for xy in line.iter_row() {
        if ! is_active {
            codes.push(PathCode::MoveTo(Point(xy[0], xy[1])));
            is_active = true;
        } else {
            match draw_style {
                DrawStyle::Default => {
                    codes.push(PathCode::LineTo(Point(xy[0], xy[1])));
                }
                DrawStyle::StepsPre => {
                    codes.push(PathCode::LineTo(Point(prev_x, xy[1])));
                    codes.push(PathCode::LineTo(Point(xy[0], xy[1])));
                }
                DrawStyle::StepsMid => {
                    codes.push(PathCode::LineTo(Point((prev_x + xy[0]) * 0.5, prev_y)));
                    codes.push(PathCode::LineTo(Point((prev_x + xy[0]) * 0.5, xy[1])));
                    codes.push(PathCode::LineTo(Point(xy[0], xy[1])));
                }
                DrawStyle::StepsPost => {
                    codes.push(PathCode::LineTo(Point(xy[0], prev_y)));
                    codes.push(PathCode::LineTo(Point(xy[0], xy[1])));
                }
            }
        }

        (prev_x, prev_y) = (xy[0], xy[1]);

        // TODO: build new tensor
    }

    Path::new(codes)
}

impl Artist<Data> for Lines2d {
    fn update(&mut self, canvas: &Canvas) {
        self.extent = Bounds::from(&self.xy);

        if self.is_stale {
            self.is_stale = false;

            if let Some(collection) = &mut self.collection {
                collection.update(canvas);
            }
        }
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        self.extent.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        if ! self.is_visible {
            return;
        }

        let path = self.path.transform(&to_canvas);

        let style = self.style.push(style);

        renderer.draw_path(&path, &style, clip).unwrap();

        if let Some(collection) = &mut self.collection {
            if let Some(marker) = &self.marker {
                let style = marker.get_style().push(&style);

                collection.draw(renderer, to_canvas, clip, &style);
            }
        }
    }
}

impl PlotArtist<Data> for Lines2d {
    type Opt = LinesOpt;

    fn config(&mut self, cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "lines");

        unsafe { LinesOpt::new(id) }
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        match &self.label {
            Some(label) => {
                let style = self.style.clone();
                Some(LegendHandler::new(label.clone(), 
                move |renderer, top_style, bounds| {
                    let line = Path::<Canvas>::from([
                        [bounds.xmin(), bounds.ymid()],
                        [bounds.xmax(), bounds.ymid()],
                    ]);
                    renderer.draw_path(
                        &line, 
                        &style.push(top_style), 
                        &Clip::None
                    ).unwrap();
                }))
            },
            None => None,
        }
    }
}

data_artist_option_struct!(LinesOpt, Lines2d);

impl LinesOpt {
    path_style_options!(style);

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.write(|artist| {
            if label.len() > 0 {
                artist.label = Some(label.to_string());
            } else {
                artist.label = None;
            }
        });

        self
    }

    pub fn set_xy(&mut self, x: impl Into<Tensor>, y: impl Into<Tensor>) -> &mut Self {
        self.write(|artist| {
            artist.set_xy(x, y);
        });

        self
    }

    pub fn marker(&mut self, marker: impl IntoMarker) -> &mut Self {
        self.write(|artist| {
            artist.marker(marker);
        });

        self
    }

    pub fn draw_style(&mut self, draw_style: impl Into<DrawStyle>) -> &mut Self {
        self.write(|artist| {
            artist.draw_style(draw_style.into());
        });

        self
    }

    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.write(|artist| {
            artist.is_visible = visible;
            artist.is_stale = true;
        });

        self
    }

    pub fn z_order(&mut self, order: f32) -> &mut Self {
        self.write(|artist| {
            artist.z_order = order;
            artist.is_stale = true;
        });

        self
    }
}

//impl PathStyleArtist for Lines2d {
//    fn style_mut(&mut self) -> &mut PathStyle {
//        &mut self.style
//    }
//}

impl fmt::Debug for Lines2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.xy.dim(0) {
            0 => {
                write!(f, "Lines2D[]")
            },
            1 => {
                write!(f, "Lines2D[({}, {})]", self.xy[(0, 0)], self.xy[(0, 1)])
            },
            2 => {
                write!(f, "Lines2D[({}, {}), ({}, {})]", 
                    self.xy[(0, 0)], self.xy[(0, 1)],
                    self.xy[(1, 0)], self.xy[(1, 1)])
            },
            n => {
                write!(f, "Lines2D[({}, {}), ({}, {}), ..., ({}, {})]", 
                    self.xy[(0, 0)], self.xy[(0, 1)],
                    self.xy[(1, 0)], self.xy[(1, 1)],
                    self.xy[(n - 1, 0)], self.xy[(n - 1, 1)])
            }
        }
    }
}

#[cfg(test)]
mod test {
    use essay_tensor::prelude::*;

    use super::Lines2d;

    #[test]
    fn test_lines() {
        let lines = Lines2d::from_xy(
            tf32!([1., 2., 4., 8.]),
            tf32!([10., 20., 40., 80.])
        );
        println!("Lines {:?}", &lines);
    }
}