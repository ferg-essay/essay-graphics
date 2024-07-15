use core::fmt;

use essay_graphics_api::{
    driver::Renderer, PathOpt,
    Bounds, Affine2d, Point, Canvas, Coord, CanvasEvent, Clip,
};

use crate::{artist::{Artist, PathStyle, PlotArtist, ToCanvas}, graph::Config};

use super::{plot_container::PlotContainer, ArtistId, FrameId, LegendHandler};

pub struct DataBox {
    pos_canvas: Bounds<Canvas>,

    data_bounds: Bounds<Data>,
    view_bounds: Bounds<Data>,
    pan_zoom_bounds: Option<Bounds<Data>>,

    x_lim: Option<(f32, f32)>,
    y_lim: Option<(f32, f32)>,

    x_margin: Option<f32>,
    y_margin: Option<f32>,

    aspect: Option<f32>,
    aspect_mode: AspectMode,
    is_flip_y: bool,

    artists: PlotContainer<Data>,

    to_canvas: Affine2d,
    style: PathStyle,

    is_stale: bool,
}

impl DataBox {
    pub fn new(frame_id: FrameId, cfg: &Config) -> Self {
        Self {
            pos_canvas: Bounds::none(),

            data_bounds: Bounds::<Data>::unit(),
            view_bounds: Bounds::<Data>::unit(),
            pan_zoom_bounds: None,

            x_lim: None,
            y_lim: None,
            x_margin: cfg.get_as_type("frame", "x_margin"),
            y_margin: cfg.get_as_type("frame", "y_margin"),
            aspect: None,
            aspect_mode: AspectMode::BoundingBox,
            is_flip_y: false,

            artists: PlotContainer::new(frame_id, cfg),

            style: PathStyle::default(),

            to_canvas: Affine2d::eye(),
            is_stale: true,
        }
    }

    pub fn aspect(&mut self, aspect: f32) -> &mut Self {
        self.aspect = Some(aspect);

        self
    }

    pub fn aspect_mode(&mut self, mode: AspectMode) -> &mut Self {
        self.aspect_mode = mode;

        self
    }

    pub fn flip_y(&mut self, is_flip_y: bool) -> &mut Self {
        self.is_flip_y = is_flip_y;

        self
    }

    pub fn xlim(&mut self, x_min: f32, x_max: f32) -> &mut Self {
        assert!(x_min < x_max);

        self.x_lim = Some((x_min, x_max));

        self
    }

    pub fn ylim(&mut self, y_min: f32, y_max: f32) -> &mut Self {
        assert!(y_min < y_max);

        self.y_lim = Some((y_min, y_max));

        self
    }

    pub fn add_artist(&mut self, artist: impl PlotArtist<Data> + 'static) -> ArtistId {
        //let mut artist = artist;

        //let bounds = artist.get_extent();

        //self.add_data_bounds(&bounds);
        //self.add_view_bounds(&bounds);

        let id = self.artists.add_artist(artist);

        id
    }

    ///
    /// Sets the canvas bounds
    /// 
    pub(crate) fn set_pos(&mut self, pos: &Bounds<Canvas>) -> &mut Self {
        self.update_view();

        self.pos_canvas = pos.clone();

        self.update_aspect();
        
        self.to_canvas = self.get_view_bounds().affine_to(&self.pos_canvas);

        if self.is_flip_y {
            self.to_canvas = self.to_canvas
                .translate(0., - self.pos_canvas.ymin())
                .scale(1., -1.)
                .translate(0., self.pos_canvas.ymax());
        }

        self
    }

    fn update_view(&mut self) {
        let data = &self.data_bounds;

        let (height, width) = (data.height(), data.width());

        let (mut xmin, mut xmax) = (data.xmin(), data.xmax());
        let (mut ymin, mut ymax) = (data.ymin(), data.ymax());

        if self.aspect.is_none() {
            if let Some(x_margin) = self.x_margin {
                xmin -= x_margin * width;
                xmax += x_margin * width;
            }

            if let Some(y_margin) = self.y_margin {
                ymin -= y_margin * height;
                ymax += y_margin * height;
            }
        }

        // single point
        if xmin == xmax {
            xmin = xmin - 1.;
            xmax = xmax + 1.;
        }

        if ymin == ymax {
            ymin = ymin - 1.;
            ymax = ymax + 1.;
        }

        if let Some(xlim) = self.x_lim {
            xmin = xlim.0;
            xmax = xlim.1;
        }

        if let Some(ylim) = self.y_lim {
            ymin = ylim.0;
            ymax = ylim.1;
        }

        self.view_bounds = Bounds::new(Point(xmin, ymin), Point(xmax, ymax));
        // pos.clone()
    }

    fn update_aspect(&mut self) {
        match self.aspect_mode {
            AspectMode::BoundingBox => self.update_aspect_pos(),
            AspectMode::View => self.update_aspect_view(),
        }
    }

    fn update_aspect_view(&mut self) {
        if let Some(_aspect) = self.aspect {
            let mut bounds = self.view_bounds.clone();

            if bounds.height() < bounds.width() {
                let ymid = bounds.ymid();
                let h2 = bounds.width() * 0.5;

                bounds = Bounds::new(
                    Point(bounds.xmin(), ymid - h2),
                    Point(bounds.xmax(), ymid + h2),
                );
            } else {
                let w2 = bounds.height() * 0.5;

                bounds = Bounds::new(
                    Point(bounds.xmid() - w2, bounds.ymin()),
                    Point(bounds.xmid() + w2, bounds.ymax()),
                );
            }

            self.view_bounds = bounds;
        }
    }

    fn update_aspect_pos(&mut self) {
        if let Some(_aspect) = self.aspect {
            let view_ratio = self.view_bounds.width() / self.view_bounds.height().max(f32::EPSILON);
            let pos = &self.pos_canvas;
            let pos_ratio = pos.width() / pos.height().max(f32::EPSILON);

            let pos = if pos_ratio < view_ratio {
                let h2 = pos.width() * 0.5 / view_ratio;

                Bounds::new(
                    Point(pos.xmin(), pos.ymid() - h2),
                    Point(pos.xmax(), pos.ymid() + h2),
                )
            } else {
                let w2 = pos.height() * 0.5 * view_ratio;

                Bounds::new(
                    Point(pos.xmid() - w2, pos.ymin()),
                    Point(pos.xmid() + w2, pos.ymax()),
                )
            };

            self.pos_canvas = pos;
        }
    }

    pub(crate) fn get_pos(&self) -> &Bounds<Canvas> {
        &self.pos_canvas
    }

    pub(crate) fn get_view_bounds(&self) -> &Bounds<Data> {
        if let Some(bounds) = &self.pan_zoom_bounds {
            bounds
        } else {
            &self.view_bounds
        }
    }

    pub(crate) fn get_canvas_transform(&self) -> &Affine2d {
        &self.to_canvas
    }

    // true if request redraw
    pub fn event(&mut self, _renderer: &mut dyn Renderer, event: &CanvasEvent) -> bool {
        match event {
            CanvasEvent::ResetView(_) => {
                //self.update_view();
                self.is_stale = true;
                self.pan_zoom_bounds = None;
                true
            }
            CanvasEvent::Pan(_p_start, p_last, p_now) => {
                let to_data = self.pos_canvas.affine_to(&self.get_view_bounds());
                let p0 = to_data.transform_point(*p_last);
                let p1 = to_data.transform_point(*p_now);

                let dx = p0.x() - p1.x();
                let dy = p0.y() - p1.y();

                let view = &self.get_view_bounds();
                self.pan_zoom_bounds = Some(Bounds::new(
                    Point(
                        view.x0() + dx,
                        view.y0() + dy,
                    ),
                    Point(
                        view.x1() + dx,
                        view.y1() + dy,
                    )
                ));

                true
            },
            CanvasEvent::ZoomBounds(p_start, p_now) => {
                if self.pos_canvas.contains(*p_now) {
                    let to_data = self.pos_canvas.affine_to(&self.get_view_bounds());
                    let p0 = to_data.transform_point(*p_start);
                    let p1 = to_data.transform_point(*p_now);

                    // let view = &self.view_bounds;
                    // TODO: check min size?
                    self.pan_zoom_bounds = Some(Bounds::new(p0, p1));
                }

                true
            },
            _ => { false }
        }
    }

    //pub(crate) fn style_mut(&mut self, id: ArtistId) -> &mut PathStyle {
    //    self.artists.style_mut(id)
    //}

    pub(crate) fn _artist<A>(&self, id: ArtistId) -> &A
    where
        A: Artist<Data> + 'static
    {
        self.artists._artist(id)
    }

    pub(crate) fn artist_mut<A>(&mut self, id: ArtistId) -> &mut A
    where
        A: Artist<Data> + 'static
    {
        self.artists.artist_mut(id)
    }

    pub(crate) fn get_handlers(&self) -> Vec<LegendHandler> {
        self.artists.get_handlers()
    }
}

impl Artist<Canvas> for DataBox {
    fn update(&mut self, canvas: &Canvas) {
        self.artists.update(canvas);

        self.is_stale = true;
        if self.is_stale {
            self.is_stale = false;

            self.data_bounds = self.artists.get_extent();
    
            self.update_view();
        }
    }

    fn get_extent(&mut self) -> Bounds<Canvas> {
        self.pos_canvas.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas,
        _clip: &Clip,
        style: &dyn PathOpt,
    ) {
        renderer.flush(&Clip::None);
        //let to_canvas = to_canvas.matmul(&self.to_canvas);
        // let to_canvas = &self.to_canvas;
        let style = self.style.push(style);
        let clip = Clip::Bounds(self.pos_canvas.p0(), self.pos_canvas.p1());

        self.artists.draw(renderer, to_canvas, &clip, &style);

        //renderer.flush(&clip);
        renderer.flush(&Clip::None);

        // TODO: intersect clip
        //for artist in &mut self.artists {
        //    artist.draw(renderer, &to_canvas, &self.pos_canvas, &style);
        //}
    }
}


impl fmt::Debug for DataBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DataBox({},{},{}x{})",
            self.view_bounds.xmin(),
            self.view_bounds.ymin(),
            self.view_bounds.width(),
            self.view_bounds.height())
    }
}

pub enum AspectMode {
    BoundingBox,
    View
}

///
/// Data coordinates
///
#[derive(Clone, Copy, Debug)]
pub struct Data;

impl Coord for Data {
}
