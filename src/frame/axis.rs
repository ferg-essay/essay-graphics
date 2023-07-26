use essay_plot_api::{TextStyle, Canvas, driver::Renderer, Affine2d, Clip, PathOpt, VertAlign, Point, Path, HorizAlign, Bounds, PathCode};

use crate::{artist::{PathStyle, Text, Artist, patch::CanvasPatch, paths}, graph::Config, frame_option_struct, path_style_options};

use super::{
    data_box::DataBox,
    tick_locator::{MaxNLocator, TickLocator}, FrameArtist, tick_formatter::{TickFormatter, Formatter},
};

pub struct Axis {
    show_grid: ShowGrid,

    major: AxisTicks,
    minor: AxisTicks,

    locator: Box<dyn TickLocator>,
    formatter: Box<dyn TickFormatter>,

    is_visible: bool,
}

impl Axis {
    pub fn new(cfg: &Config, prefix: &str) -> Self {
        Self {
            //locator: Box::new(LinearLocator::new(None)),
            show_grid: ShowGrid::None,
            major: AxisTicks::new(cfg, &cfg.join(prefix, "major")),
            minor: AxisTicks::new(cfg, &cfg.join(prefix, "minor")),
            locator: Box::new(MaxNLocator::new(None)),
            formatter: Box::new(Formatter::Plain),
            is_visible: true,
        }
    }

    //pub fn ticks(&self, data: &DataBox) -> Tensor<f32> {
    //}

    pub(crate) fn major(&self) -> &AxisTicks {
        &self.major
    }

    pub(crate) fn major_mut(&mut self) -> &mut AxisTicks {
        &mut self.major
    }

    pub(crate) fn minor(&self) -> &AxisTicks {
        &self.minor
    }

    pub(crate) fn minor_mut(&mut self) -> &mut AxisTicks {
        &mut self.minor
    }

    pub fn x_ticks(&self, data: &DataBox) -> Vec<(f32, f32)> {
        let c_width = data.get_pos().width();

        let view = data.get_view_bounds();
        let v_width = view.width();

        if view.is_none() {
            Vec::new()
        } else {
            let (vmin, vmax) = (view.xmin(), view.xmax());
            let (min, max) = self.locator.view_limits(vmin, vmax);

            // self.locator.tick_values(min, max)

            let mut x_vec = Vec::<(f32, f32)>::new();

            for x in self.locator.tick_values(min, max).iter() {
                x_vec.push((*x, ((x - vmin) * c_width / v_width).round()));
            }

            x_vec
        }
    }

    pub fn y_ticks(&self, data: &DataBox) -> Vec<(f32, f32)> {
        let v_height = data.get_view_bounds().height();
        let c_height = data.get_pos().height();

        let view = data.get_view_bounds();

        if view.is_none() {
            Vec::new()
        } else {
            let (vmin, vmax) = (view.ymin(), view.ymax());
            let (min, max) = self.locator.view_limits(vmin, vmax);

            // self.locator.tick_values(min, max)

            let mut y_vec = Vec::<(f32, f32)>::new();

            for y in self.locator.tick_values(min, max).iter() {
                y_vec.push((*y, ((y - vmin) * c_height / v_height).round()));
            }

            y_vec
        }
    }

    pub fn value_delta(xvalues: &Vec<f32>) -> f32 {
        let len = xvalues.len();

        if len <= 1 {
            return 1.;
        }

        let mut delta = (xvalues[len - 1] - xvalues[0]).abs();
        for i in 0..len - 1 {
            delta = (xvalues[i + 1] - xvalues[i]).abs().min(delta);
        }

        delta
    }

    pub(crate) fn get_show_grid(&self) -> &ShowGrid {
        &self.show_grid
    }

    pub(crate) fn visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    pub(crate) fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn format(&self, value: f32, delta: f32) -> String {
        self.formatter.format(value, delta)
    }

    pub(crate) fn update(&mut self, canvas: &Canvas) {
        self.major.update(canvas);
        self.minor.update(canvas);
    }
}

//
// XAxis
//

pub struct XAxis {
    spine: Option<CanvasPatch>,

    axis: Axis,
    major_ticks: Vec<f32>,
    major_labels: Vec<String>,

}

impl XAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let mut x_axis = Self {
            spine: Some(CanvasPatch::new(paths::line([0., 0.], [1., 0.]))),
            axis: Axis::new(cfg, prefix),

            major_ticks: Vec::new(),
            major_labels: Vec::new(),
        };

        x_axis.axis.major_mut().label_style_mut().valign(VertAlign::Top);
        x_axis.axis.minor_mut().label_style_mut().valign(VertAlign::Top);

        x_axis
    }

    pub fn update_axis(&mut self, data: &DataBox) {
        self.major_ticks = Vec::new();
        self.major_labels = Vec::new();

        let xmin = data.get_view_bounds().xmin();
        let xmax = data.get_view_bounds().xmax();

        let xvalues : Vec<f32> = self.axis.x_ticks(data).iter().map(|x| x.0).collect();

        let delta = Axis::value_delta(&xvalues);

        for xv in xvalues {
            if xmin <= xv && xv <= xmax {
                self.major_ticks.push(xv);
                self.major_labels.push(
                    self.axis.major().format(&self.axis, xv, delta)
                );
            };
        }
    }

    pub(crate) fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataBox,
        to_canvas: &Affine2d,
        clip: &Clip,
        style: &dyn PathOpt,
    ) -> f32 {
        self.draw_bottom(renderer, data, to_canvas, clip, style)
    }

    fn draw_bottom(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataBox,
        to_canvas: &Affine2d,
        clip: &Clip,
        style: &dyn PathOpt,
    ) -> f32 {
        let pos = data.get_pos();

        if let Some(patch) = &mut self.spine {
            patch.set_pos([
                (pos.xmin(), pos.ymin() - 1.),
                (pos.xmax(), pos.ymin()),
            ]);

            patch.draw(renderer, to_canvas, clip, style);
        }

        let mut y = data.get_pos().ymin();

        if self.axis.is_visible() {
            self.draw_ticks(renderer, &data, clip, style);

            y -= renderer.to_px(self.axis.major().get_size());
            y -= renderer.to_px(self.axis.major().get_pad());
            y -= self.axis.major().get_label_height();
        }
        y
    }

    fn draw_ticks(
        &mut self, 
        renderer: &mut dyn Renderer, 
        data: &DataBox,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let pos = &data.get_pos();

        let yv = data.get_view_bounds().ymin();
        let to_canvas = data.get_canvas_transform();

        for (xv, label) in self.major_ticks.iter().zip(self.major_labels.iter()) {
            let point = to_canvas.transform_point(Point(*xv, yv));

            let x = point.x();
            let mut y = pos.ymin();
            let major = self.axis.major();

            // Grid
            if self.axis.get_show_grid().is_show_major() {
                let style = major.grid_style().push(style);
                // grid
                let grid = Path::<Canvas>::move_to(x, pos.ymin())
                    .line_to(x, pos.ymax())
                    .to_path();

                renderer.draw_path(&grid, &style, clip).unwrap();
            }

            // Tick
            {
                let style = major.tick_style().push(style);
                let tick_length = renderer.to_px(major.get_size());

                let tick = Path::<Canvas>::move_to(x, y)
                    .line_to(x, y - tick_length).to_path();

                renderer.draw_path(&tick, &style, clip).unwrap();

                y -= tick_length;
                y -= renderer.to_px(major.get_pad());
            }

            // Label
            renderer.draw_text(Point(x, y), label, 0., style, major.label_style(), clip).unwrap();
        }
    }

    pub(crate) fn update(&mut self, canvas: &Canvas) {
        self.axis.update(canvas);
    }

    pub(crate) fn axis_mut(&mut self) -> &mut Axis {
        &mut self.axis
    }
}

//
// YAxis
//

pub struct YAxis {
    spine: Option<CanvasPatch>,

    axis: Axis,
    major_ticks: Vec<f32>,
    major_labels: Vec<String>,

}

impl YAxis {
    pub(crate) fn new(cfg: &Config, prefix: &str) -> Self {
        let mut y_axis = Self {
            spine: Some(CanvasPatch::new(paths::line([0., 0.], [0., 1.]))),
            axis: Axis::new(cfg, prefix),

            major_ticks: Vec::new(),
            major_labels: Vec::new(),
        };

        y_axis.axis.major_mut().label_style_mut().valign(VertAlign::Center);
        y_axis.axis.major_mut().label_style_mut().halign(HorizAlign::Right);

        y_axis
    }

    pub fn update_axis(&mut self, data: &DataBox) {
        self.major_ticks = Vec::new();
        self.major_labels = Vec::new();

        let ymin = data.get_view_bounds().ymin();
        let ymax = data.get_view_bounds().ymax();

        let yvalues : Vec<f32> = self.axis.y_ticks(data).iter().map(|y| y.0).collect();

        let delta = Axis::value_delta(&yvalues);

        for yv in yvalues {
            if ymin <= yv && yv <= ymax {
                self.major_ticks.push(yv);
                self.major_labels.push(
                    self.axis.major().format(&self.axis, yv, delta)
                );
            };
        }
    }

    pub(crate) fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataBox,
        to_canvas: &Affine2d,
        clip: &Clip,
        style: &dyn PathOpt,
    ) -> f32 {
        self.draw_left(renderer, data, to_canvas, clip, style)
    }

    fn draw_left(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataBox,
        to_canvas: &Affine2d,
        clip: &Clip,
        style: &dyn PathOpt,
    ) -> f32 {
        let pos = data.get_pos();

        if let Some(patch) = &mut self.spine {
            patch.set_pos(Bounds::new(
                Point(pos.xmin() - 1., pos.ymin()),
                Point(pos.xmin(), pos.ymax()),
            ));

            patch.draw(renderer, to_canvas, clip, style);
        }

        let mut x = data.get_pos().xmin();

        if self.axis.is_visible() {
            self.draw_ticks(renderer, &data, clip, style);

            let width = self.major_labels.iter().map(|s| s.len()).max().unwrap();
        
            x -= renderer.to_px(self.axis.major().get_size());
            x -= renderer.to_px(self.axis.major().get_pad());
            x -= 0.5 * width as f32 * self.axis.major().get_label_height();
        }

        x
    }

    fn draw_ticks(
        &mut self, 
        renderer: &mut dyn Renderer, 
        data: &DataBox,
        clip: &Clip,
        style: &dyn PathOpt,
    ) {
        let pos = &data.get_pos();

        let xv = data.get_view_bounds().xmin();
        let to_canvas = data.get_canvas_transform();

        for (yv, label) in self.major_ticks.iter().zip(self.major_labels.iter()) {
            let point = to_canvas.transform_point(Point(xv, *yv));

            let y = point.y();
            let mut x = pos.xmin();
            let major = self.axis.major();

            // Grid
            if self.axis.get_show_grid().is_show_major() {
                let style = major.grid_style().push(style);
                // grid
                let grid = Path::<Canvas>::new(vec![
                    PathCode::MoveTo(Point(pos.xmin(), y)),
                    PathCode::LineTo(Point(pos.xmax(), y)),
                ]);

                renderer.draw_path(&grid, &style, clip).unwrap();
            }

            // Tick
            {
                let style = major.tick_style().push(style);
                let tick_length = renderer.to_px(major.get_size());
                
                let tick = Path::<Canvas>::new(vec![
                    PathCode::MoveTo(Point(x - tick_length, y)),
                    PathCode::LineTo(Point(x, y)),
                ]);

                renderer.draw_path(&tick, &style, clip).unwrap();

                x -= tick_length;
                x -= renderer.to_px(major.get_pad());
            }

            // Label
            renderer.draw_text(Point(x, y), label, 0., style, major.label_style(), clip).unwrap();
        }
    }

    pub(crate) fn update(&mut self, canvas: &Canvas) {
        self.axis.update(canvas);
    }

    pub(crate) fn axis_mut(&mut self) -> &mut Axis {
        &mut self.axis
    }
}

pub struct AxisTicks {
    grid_style: PathStyle,
    ticks_style: PathStyle,
    label_text: Text,
    size: f32,
    pad: f32,
    locator: Option<Box<dyn TickLocator>>,
    formatter: Option<Box<dyn TickFormatter>>,
}

impl AxisTicks {
    fn new(cfg: &Config, prefix: &str) -> Self {
        let mut ticks = Self {
            grid_style: PathStyle::from_config(cfg, &cfg.join(prefix, "grid")),
            ticks_style: PathStyle::from_config(cfg, &cfg.join(prefix, "ticks")),
            size: match cfg.get_as_type(prefix, "size") {
                Some(size) => size,
                None => 0.,
            },
            pad: match cfg.get_as_type(prefix, "pad") {
                Some(size) => size,
                None => 0.,
            },
            label_text: Text::new(),
            locator: None,
            formatter: None,
        };

        match cfg.get_as_type::<f32>(prefix, "width") {
            Some(width) => { ticks.ticks_style.line_width(width); }
            None => {}
        };

        ticks.label_text.label("0.0");
        
        ticks
    }

    pub(crate) fn grid_style(&self) -> &PathStyle {
        &self.grid_style
    }

    pub(crate) fn tick_style(&self) -> &PathStyle {
        &self.ticks_style
    }

    pub(crate) fn label_style(&self) -> &TextStyle {
        self.label_text.text_style()
    }

    pub(crate) fn label_style_mut(&mut self) -> &mut TextStyle {
        self.label_text.text_style_mut()
    }

    pub(crate) fn grid_style_mut(&mut self) -> &mut PathStyle {
        &mut self.grid_style
    }

    pub(crate) fn format(&self, axis: &Axis, value: f32, delta: f32) -> String {
        match &self.formatter {
            Some(formatter) => {
                formatter.format(value, delta)
            }
            None => { 
                axis.format(value, delta) 
            }
        }
    }

    pub(crate) fn get_size(&self) -> f32 {
        self.size
    }

    pub(crate) fn get_pad(&self) -> f32 {
        self.pad
    }

    pub(crate) fn get_label_height(&self) -> f32 {
        self.label_text.height()
    }

    pub(crate) fn update(&mut self, canvas: &Canvas) {
        self.label_text.update(canvas);
    }
}

frame_option_struct!(AxisOpt, Axis, get_axis_mut);

impl AxisOpt {
    pub fn show_grid(&mut self, show: impl Into<ShowGrid>) -> &mut Self {
        self.write(|axis| { axis.show_grid = show.into(); });
        self
    }

    pub fn visible(&mut self, is_visible: bool) -> &mut Self {
        self.write(|axis| { axis.is_visible = is_visible; });
        self
    }

    pub fn locator(&mut self, locator: impl TickLocator + 'static) -> &mut Self {
        self.write(|axis| { 
            axis.locator = Box::new(locator); 
        });
        self
    }

    pub fn formatter(&mut self, formatter: impl TickFormatter + 'static) -> &mut Self {
        self.write(|axis| { 
            axis.formatter = Box::new(formatter); 
        });
        self
    }

    pub fn major(&self) -> AxisTicksOpt {
        let artist = match self.artist {
            FrameArtist::X => FrameArtist::XMajor,
            FrameArtist::Y => FrameArtist::YMajor,
            _ => panic!("invalid major()")
        };

        AxisTicksOpt::new(self.layout.clone(), self.frame_id, artist)
    }

    pub fn major_grid(&self) -> AxisGridOpt {
        let artist = match self.artist {
            FrameArtist::X => FrameArtist::XMajor,
            FrameArtist::Y => FrameArtist::YMajor,
            _ => panic!("invalid major()")
        };

        AxisGridOpt::new(self.layout.clone(), self.frame_id, artist)
    }
}

frame_option_struct!(AxisGridOpt, AxisTicks, get_ticks_mut);

impl AxisGridOpt {
    path_style_options!(grid_style);
}

frame_option_struct!(AxisTicksOpt, AxisTicks, get_ticks_mut);

impl AxisTicksOpt {
    pub fn locator(&mut self, locator: impl TickLocator + 'static) -> &mut Self {
        self.write(|ticks| { 
            ticks.locator = Some(Box::new(locator)); 
        });
        self
    }

    pub fn formatter(&mut self, formatter: impl TickFormatter + 'static) -> &mut Self {
        self.write(|ticks| { 
            ticks.formatter = Some(Box::new(formatter)); 
        });
        self
    }

    path_style_options!(ticks_style);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShowGrid {
    None,
    Major,
    Minor,
    Both,
}

impl ShowGrid {
    pub(crate) fn is_show_major(&self) -> bool {
        match self {
            ShowGrid::None => false,
            ShowGrid::Major => true,
            ShowGrid::Minor => false,
            ShowGrid::Both => true,
        }
    }

    pub(crate) fn is_show_minor(&self) -> bool {
        match self {
            ShowGrid::None => false,
            ShowGrid::Major => false,
            ShowGrid::Minor => true,
            ShowGrid::Both => true,
        }
    }
}

impl From<bool> for ShowGrid {
    fn from(value: bool) -> Self {
        if value {
            Self::Major
        } else {
            Self::None
        }
    }
}
