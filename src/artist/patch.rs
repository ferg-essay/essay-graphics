use core::fmt;

use essay_graphics_api::{
    PathOpt, Point, Affine2d, Bounds, Canvas, Path, Angle,
    driver::Renderer, Coord, Clip
};

use crate::{frame::{Data, LegendHandler}, graph::ConfigArc, data_artist_option_struct, path_style_options, transform_options};

use super::{Artist, paths, PathStyle, PlotArtist, PlotId, IntoArtist, ToCanvas};

pub trait PatchTrait<M: Coord> : Send {
    fn get_path(&mut self) -> &Path<M>;
}

pub struct Patch {
    path: Path<Data>,
    xform_path: Path<Data>,
    label: Option<String>,

    transform: Affine2d,

    style: PathStyle,
}

impl Patch {
    pub fn new(path: impl Into<Path<Data>>) -> Self {
        let path: Path<Data> = path.into();

        Self {
            xform_path: path.clone(),
            path,
            label: None,
            transform: Affine2d::eye(),
            style: PathStyle::new(),
        }
    }

    pub fn rect(p0: impl Into<Point>, p1: impl Into<Point>) -> Self {
        Self::new(paths::rect(p0, p1))
    }

    pub fn stale(&mut self) {
        self.xform_path = self.path.transform(&self.transform);
        //self.bounds = self.xform_path.get_bounds();
    }
}

impl Artist<Data> for Patch {
    fn update(&mut self, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        Bounds::none()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let to_canvas = to_canvas.matmul(&self.transform);
        let path = self.path.transform(&to_canvas);
        let style = self.style.push(style);

        renderer.draw_path(
            &path,
            &style, 
            clip
        ).unwrap();
    }
}

impl PlotArtist<Data> for Patch {
    type Opt = PatchOpt;

    fn config(&mut self, cfg: &ConfigArc, id: PlotId) -> Self::Opt {
        self.style = PathStyle::from_config(cfg, "patch");

        unsafe { PatchOpt::new(id) }
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

data_artist_option_struct!(PatchOpt, Patch);

impl PatchOpt {
    path_style_options!(style);
    transform_options!(transform);

    pub fn label(&mut self, label: impl AsRef<str>) -> &mut Self {
        self.write(|artist| {
            let label = label.as_ref();
            if label.len() > 0 {
                artist.label = Some(label.to_string());
            } else {
                artist.label = None;
            }
        });

        self
    }

    pub fn path(&mut self, path: impl Into<Path<Data>>) -> &mut Self {
        self.write(|artist| {
            artist.path = path.into();
            artist.stale();
        });

        self
    }
}

pub fn arrow(xy: impl Into<Point>, dxdy: impl Into<Point>) -> Arrow {
    Arrow::new(xy, dxdy)
}

pub struct Arrow {
    xy: Point,
    dxdy: Point,
    width: f32,
    head_length: f32,
    head_width: f32,
    tail_width: f32,
}

impl Arrow {
    pub fn new(xy: impl Into<Point>, dxdy: impl Into<Point>) -> Self {
        Self {
            xy: xy.into(),
            dxdy: dxdy.into(),
            width: 1.,
            head_width: 0.6,
            head_length: 0.2,
            tail_width: 0.2,
        }
    }

    pub fn tail_width(mut self, width: f32) -> Self {
        assert!(0. <= width && width <= 1.0);

        self.tail_width = width;
        self
    }

    pub fn head_width(mut self, width: f32) -> Self {
        assert!(0. <= width && width <= 1.0);

        self.head_width = width;
        self
    }

    pub fn head_length(mut self, length: f32) -> Self {
        assert!(0. <= length && length <= 1.0);

        self.head_length = length;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn to_path(&self) -> Path<Data> {
        let Point(x, y) = self.xy;
        let Point(dx, dy) = self.dxdy.into();
    
        let s_tail = 0.5 * self.tail_width * self.width;
        let s_head = 0.5 * self.head_width * self.width;
    
        let hypot = dx.hypot(dy);
        let (tx, ty) = (dy / hypot, - dx / hypot);
        
        let x_tail = tx * s_tail;
        let y_tail = ty * s_tail;
    
        let xt_head = tx * s_head;
        let yt_head = ty * s_head;
    
        let tail_length = 1. - self.head_length;
        let dx_head = tail_length * dx;
        let dy_head = tail_length * dy;
    
        Path::move_to(x - x_tail, y - y_tail)
        .line_to(x + x_tail, y + y_tail)
        .line_to(x + x_tail + dx_head, y + y_tail + dy_head)
    
        .line_to(x + xt_head + dx_head, y + yt_head + dy_head)
        .line_to(x + dx, y + dy)
        .line_to(x - xt_head + dx_head, y - yt_head + dy_head)
    
        .close_poly(x - x_tail + dx_head, y - y_tail + dy_head)
        .to_path()
    }
}

impl IntoArtist<Data> for Arrow {
    type Artist = Patch;

    fn into_artist(self) -> Self::Artist {
        Patch::new(self.to_path())
    }
}

pub struct CanvasPatch {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,

    path: Path<Canvas>,
    to_canvas: Affine2d,
    style: PathStyle,
}

impl CanvasPatch {
    pub fn new(path: impl Into<Path<Canvas>>) -> Self {
        Self {
            bounds: Bounds::unit(),
            pos: Bounds::none(),

            path: path.into(),
            to_canvas: Affine2d::eye(),
            style: PathStyle::new(),
        }
    }

    pub fn set_pos(&mut self, pos: impl Into<Bounds<Canvas>>) {
        self.pos = pos.into();
        self.to_canvas = self.bounds.affine_to(&self.pos);
    }
}

impl Artist<Canvas> for CanvasPatch {
    fn update(&mut self, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let to_canvas = to_canvas.matmul(&self.to_canvas);
        let path = self.path.transform(&to_canvas);
        let style = self.style.push(style);

        renderer.draw_path(
            &path,
            &style, 
            clip
        ).unwrap();
    }
}

pub struct PathPatch<M: Coord> {
    path: Path<M>,
}

impl<M: Coord> PathPatch<M> {
    pub fn new(path: Path<M>) -> Self {
        Self {
            path
        }
    }
}

impl Artist<Canvas> for PathPatch<Canvas> {
    fn update(&mut self, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Canvas> {
        todo!()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let path = self.path.transform(&to_canvas);

        renderer.draw_path(
            &path,
            style, 
            clip
        ).unwrap();
    }
}

impl Artist<Data> for PathPatch<Data> {
    fn update(&mut self, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        self.path.get_bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let path = self.path.transform(&to_canvas);

        renderer.draw_path(
            &path,
            style, 
            clip
        ).unwrap();
    }
}

pub struct Line {
    p0: Point,
    p1: Point,

    path: Option<Path<Canvas>>,
}

impl Line {
    pub fn new(
        p0: impl Into<Point>,
        p1: impl Into<Point>,
    ) -> Self {
        Self {
            p0: p0.into(),
            p1: p1.into(),
            path: None,
        }
    }

    //pub fn color(&mut self, color: Color) {
    //    self.color = Some(color);
    //}
}

impl PatchTrait<Canvas> for Line {
    fn get_path(&mut self) -> &Path<Canvas> {
        if self.path.is_none() {
            let path = Path::<Canvas>::from([
                self.p0, self.p1,
            ]);

            self.path = Some(path);
        }
            
        match &self.path {
            Some(path) => path,
            None => todo!(),
        }        
    }
}

impl Artist<Canvas> for Line {
    fn update(&mut self, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.get_path().get_bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        if let Some(path) = &self.path {
            let path = path.transform(&to_canvas);

            renderer.draw_path(&path, style, clip).unwrap();
        }
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line({:?}, {:?})", self.p0, self.p1)
    }
}

pub struct Wedge {
    center: Point,
    radius: f32,
    angle: (Angle, Angle),

    path: Option<Path<Data>>,
}

impl Wedge {
    pub fn new(
        center: Point,
        radius: f32,
        angle: (Angle, Angle),
    ) -> Self {
        Self {
            center,
            radius,
            angle,
            path: None,
        }
    }

    //pub fn color(&mut self, color: Color) {
    //    self.color = Some(color);
    //}
}

impl PatchTrait<Data> for Wedge {
    fn get_path(&mut self) -> &Path<Data> {
        if self.path.is_none() {
            let wedge = paths::wedge(self.angle);
            
            //println!("Wedge {:?}", wedge.codes());
            let transform = Affine2d::eye()
                .scale(self.radius, self.radius)
                .translate(self.center.x(), self.center.y());

            let wedge = wedge.transform::<Data>(&transform);

            self.path = Some(wedge);
        }

        match &self.path {
            Some(path) => path,
            None => todo!(),
        }        
    }
}

impl Artist<Data> for Wedge {
    fn update(&mut self, _canvas: &Canvas) {
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        self.get_path().get_bounds()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        if let Some(path) = &self.path {
            let path = path.transform(to_canvas);
            renderer.draw_path(&path, style, clip).unwrap();
        }
    }
}

impl fmt::Debug for Wedge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wedge(({}, {}), {}, [{}, {}])",
            self.center.x(), self.center.y(),
            self.radius,
            self.angle.0.to_degrees(), self.angle.1.to_degrees())
    }
}
