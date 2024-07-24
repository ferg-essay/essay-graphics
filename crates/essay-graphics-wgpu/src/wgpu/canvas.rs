use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, renderer::{Canvas, Drawable, RenderErr, Renderer}, Affine2d, Bounds, CapStyle, Clip, Color, FontStyle, FontTypeId, HorizAlign, ImageId, JoinStyle, LineStyle, Path, PathCode, PathOpt, Point, TextStyle, TextureId, VertAlign
};
use essay_tensor::Tensor;

use crate::PlotRenderer;

use super::{
    bezier::BezierRender, form3d::Form3dRender, image::ImageRender, shape2d::Shape2dRender, shape2d_texture::Shape2dTextureRender, text::TextRender, text_cache::FontId, triangle2d::GridMesh2dRender, triangulate::triangulate2
};


pub struct PlotCanvas {
    bounds: Bounds<Canvas>,
    scale_factor: f32,

    pub(crate) image_render: ImageRender,
    pub(crate) triangle_render: GridMesh2dRender,
    pub(crate) form3d_render: Form3dRender,
    pub(crate) shape2d_render: Shape2dRender,
    pub(crate) shape2d_texture_render: Shape2dTextureRender,
    pub(crate) bezier_render: BezierRender,
    pub(crate) text_render: TextRender,

    // texture_store: TextureCache,

    font_id_default: FontId,

    to_gpu: Affine2d,

    is_request_redraw: bool,
}

impl PlotCanvas {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
    
        let image_render = ImageRender::new(device, format);
        let triangle_render = GridMesh2dRender::new(device, format);
        let triangle3d_render = Form3dRender::new(device, format, width, height);
        let shape2d_render = Shape2dRender::new(device, format);
        let shape2d_texture_render = Shape2dTextureRender::new(device, queue, format);
        let bezier_render = BezierRender::new(device, format);
        let mut text_render = TextRender::new(device, format, 512, 512);

        let font_id_default = text_render.font("default");

        // let texture_store = TextureCache::new();
        
        Self {
            bounds: Bounds::from([width as f32, height as f32]),
            scale_factor: 1.,

            image_render,
            shape2d_render,
            shape2d_texture_render,
            text_render,
            triangle_render,
            form3d_render: triangle3d_render,
            bezier_render,

            font_id_default,
            // texture_store,

            to_gpu: Affine2d::eye(),

            is_request_redraw: false,
        }
    }

    pub(crate) fn is_request_redraw(&self) -> bool {
        self.is_request_redraw
    }

    pub fn request_redraw(&mut self, is_redraw: bool) {
        self.is_request_redraw = is_redraw;
    }

    pub fn clear(&mut self) {
        self.bezier_render.clear();
        self.text_render.clear();
        self.shape2d_render.clear();
        self.shape2d_texture_render.clear();
        self.triangle_render.clear();
        self.image_render.clear();

        self.form3d_render.clear();
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.request_redraw(true);
        self.bounds = Bounds::from([width as f32, height as f32]);

        let pos_gpu = Bounds::<Canvas>::new(
            Point(-1., -1.),
            Point(1., 1.)
        );

        self.to_gpu = self.bounds.affine_to(&pos_gpu);

        self.form3d_render.resize(device, width, height);
    }

    pub fn to_scissor(&self, clip: &Clip) -> Option<(u32, u32, u32, u32)> {
        match clip {
            Clip::None => None,
            Clip::Bounds(p0, p1) => {
                Some((
                    p0.0 as u32, 
                    (self.bounds.height() - p1.1) as u32, 
                    (p1.0 - p0.0) as u32, 
                    (p1.1 - p0.1) as u32
                ))
            }
        }
    }

    ///
    /// Returns the boundary of the canvas in pixels
    ///
    pub fn bounds(&self) -> &Bounds<Canvas> {
        &self.bounds
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        // traditional pt to px
        let pt_to_px = 4. / 3.;

        self.scale_factor = scale_factor * pt_to_px;
    }

    #[inline]
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    #[inline]
    pub fn to_px(&self, size: f32) -> f32 {
        self.scale_factor * size
    }

    fn fill_path(
        &mut self, 
        path: &Path<Canvas>, 
        _clip: &Clip,
    ) {
        self.shape2d_render.start_shape(None);
        self.bezier_render.start_shape();

        // let mut points = Vec::<Point>::new();
        let mut last = Point(0., 0.);
        for code in path.codes() {
            if let PathCode::Bezier2(p1, p2) = code {
                self.bezier_render.draw_bezier_fill(&last, p1, p2);

                //points.push(p1.clone());
            }

            last = code.tail();

            //points.push(last);
        }

        let triangles = triangulate2(path);

        for triangle in &triangles {
            self.shape2d_render.draw_triangle(&triangle[0], &triangle[1], &triangle[2]);
        }
    }

    pub(crate) fn fill_texture_path(
        &mut self, 
        path: &Path<Canvas>, 
        texture: TextureId,
        _clip: &Clip,
    ) {
        self.shape2d_texture_render.start_shape(texture, None);
        // TODO: bezier

        let triangles = triangulate2(path);

        for triangle in &triangles {
            self.shape2d_texture_render.draw_triangle(&triangle[0], &triangle[1], &triangle[2]);
        }
    }

    pub(crate) fn draw_lines(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
        _clip: &Clip,
    ) {
        let linewidth  = match style.get_line_width() {
            Some(linewidth) => *linewidth,
            None => 0.5,
        };

        if linewidth <= 0. {
            return;
        }

        let joinstyle  = match style.get_join_style() {
            Some(joinstyle) => joinstyle.clone(),
            None => JoinStyle::Bevel,
        };

        let capstyle  = match style.get_cap_style() {
            Some(capstyle) => capstyle.clone(),
            None => CapStyle::Butt,
        };
        
        let lw2 = self.to_px(0.5 * linewidth); // / self.canvas.width();
        let lw2 = lw2.max(0.5);
        
        self.shape2d_render.start_shape(None);
        self.bezier_render.start_shape();

        let mut p0 = Point(0.0f32, 0.0f32);
        let mut p_move = p0;
        let mut p_first = p0;
        let mut p_last = p0;

        for code in path.codes() {
            let p_next = match code {
                PathCode::MoveTo(p) => {
                    self.cap_line(p_last, p0, lw2, &capstyle);
                    
                    p0 = *p;
                    p_move = p0;
                    p_first = p0;
                    p_last = p0;
                    p0
                }
                PathCode::LineTo(p1) => {
                    self.shape2d_render.draw_line(&p0, p1, lw2);
                    // TODO: clip
                    *p1
                }
                PathCode::Bezier2(p1, p2) => {
                    self.bezier_render.draw_bezier_line(&p0, p1, p2, lw2);

                    *p2
                }
                PathCode::Bezier3(_, _, _) => {
                    panic!("Bezier3 should already be split into Bezier2");
                }
                PathCode::ClosePoly(p1) => {
                    //self.draw_line(p0.x(), p0.y(), p1.x(), p1.y(), lw_x, lw_y, rgba);
                    self.shape2d_render.draw_line(&p0, p1, lw2);
                    self.shape2d_render.draw_line(p1, &p_move, lw2);

                    self.join_lines(p0, *p1, p_move, lw2, &joinstyle);
                    self.join_lines(*p1, p_move, p_first, lw2, &joinstyle);

                    *p1
                }
            };

            self.join_lines(p_last, p0, p_next, lw2, &joinstyle);

            if p_first == p_move {
                p_first = p_next;
                self.cap_line(p_next, p_move, lw2, &capstyle);
            }
            p_last = p0;
            p0 = p_next;
        }
        self.cap_line(p_last, p0, lw2, &capstyle);
    }

    fn join_lines(
        &mut self, 
        b0: Point, 
        b1: Point, 
        b2: Point, 
        lw2: f32, 
        join_style: &JoinStyle
    ) {
        let min_join = 1.;

        if b0 == b1 || b1 == b2 || lw2 < min_join {
            // small lines can ignore joining.
            return;
        }

        self.join_lines_sign(b0, b1, b2, lw2, join_style, 1.);
        self.join_lines_sign(b0, b1, b2, lw2, join_style, -1.);
    }


    fn join_lines_sign(
        &mut self, 
        b0: Point, 
        b1: Point, 
        b2: Point,
        lw2: f32, 
        join_style: &JoinStyle,
        sign: f32,
    ) {
        let (nx, ny) = line_normal(b0, b1, lw2);
        let (nx, ny) = (sign * nx, sign * ny);

        // outside edge
        let p0 = Point(b0.x() + nx, b0.y() - ny);
        let p1 = Point(b1.x() + nx, b1.y() - ny);

        let (nx, ny) = line_normal(b1, b2, lw2);
        let (nx, ny) = (sign * nx, sign * ny);

        // outside edge
        let q1 = Point(b1.x() + nx, b1.y() - ny);
        let q2 = Point(b2.x() + nx, b2.y() - ny);

        // add bevel triangle
        self.shape2d_render.draw_triangle(&p1, &q1, &b1);

        match join_style {
            JoinStyle::Bevel => {},
            JoinStyle::Miter => {
                // TODO: clamp intersections of too-long length
                let mp = line_intersection(p0, p1, q1, q2);

                if mp != p0 { // non-parallel
                    self.shape2d_render.draw_triangle(&p1, &mp, &q1);
                }
            },
            JoinStyle::Round => {
                let mp = line_intersection(p0, p1, q1, q2);

                if mp != p0 && p0.dist(&p1) > 1. && q1.dist(&q2) > 1. { // non-parallel
                    self.bezier_render.draw_bezier_fill(&p1, &mp, &q1);
                }
            }
        }
    }

    fn cap_line(
        &mut self, 
        b0: Point, 
        b1: Point,
        lw2: f32, 
        cap_style: &CapStyle
    ) {
        if b0 == b1 || cap_style == &CapStyle::Butt {
            // small lines can ignore joining.
            return;
        }

        let (nx, ny) = line_normal(b0, b1, lw2);
        let (dx, dy) = (ny, nx);

        // outside edge
        let p0 = Point(b1.x() + nx, b1.y() - ny);
        // extended edge
        let p1 = Point(b1.x() + nx + dx, b1.y() - ny + dy);

        // inside edge
        let q0 = Point(b1.x() - nx, b1.y() + ny);
        // extended edge
        let q1 = Point(b1.x() - nx + dx, b1.y() + ny + dy);

        let mp = Point(b1.x() + dx, b1.y() + dy);

        match cap_style {
            CapStyle::Round => {
                self.shape2d_render.draw_triangle(&p0, &mp, &q0);
                self.bezier_render.draw_bezier_fill(&p0, &p1, &mp);
                self.bezier_render.draw_bezier_fill(&mp, &q1, &q0);
            }
            CapStyle::Projecting => {
                self.shape2d_render.draw_triangle(&p0, &p1, &q1);
                self.shape2d_render.draw_triangle(&q1, &q0, &p0);
            },
            CapStyle::Butt => {
                panic!(); // Butt has early exit
            }
        }
    }

    pub fn draw_path(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        // let to_unit = self.to_gpu.matmul(to_device);

        let face_color = match style.get_face_color() {
            Some(color) => *color,
            None => Color(0x000000ff)
        };

        let edge_color = match style.get_edge_color() {
            Some(color) => *color,
            None => face_color
        };

        let path = match style.get_line_style() {
            Some(LineStyle::Solid) | None => {
                transform_solid_path(path)
            }
            Some(line_style) => {
                let lw = match style.get_line_width() {
                    Some(lw) => self.to_px(*lw),
                    None => self.to_px(2.),
                };
                
                let pattern = line_style.to_pattern(lw);

                transform_dashed_path(path, pattern)
            },
        };

        if path.is_closed_path() && ! face_color.is_none() {
            let mut is_texture = true;

            if let Some(hatch) = style.get_hatch() {
                let texture = self.shape2d_texture_render.hatch_texture(*hatch);

                self.fill_texture_path(&path, texture, clip);

                self.shape2d_texture_render.draw_style(face_color, &self.to_gpu);
                //self.bezier_render.draw_style(face_color, &self.to_gpu);

                is_texture = true;
            } else if let Some(texture) = style.get_texture() {
                self.fill_texture_path(&path, *texture, clip);
    
                self.shape2d_texture_render.draw_style(face_color, &self.to_gpu);
                self.bezier_render.draw_style(face_color, &self.to_gpu);

                is_texture = true;
            } else {
                self.fill_path(&path, clip);

                self.shape2d_render.draw_style(face_color, &self.to_gpu);
                self.bezier_render.draw_style(face_color, &self.to_gpu);
            }

            if face_color != edge_color || is_texture {
                self.draw_lines(&path, style, clip);

                self.shape2d_render.draw_style(edge_color, &self.to_gpu);
                self.bezier_render.draw_style(edge_color, &self.to_gpu);
            }
        } else {
            self.draw_lines(&path, style, clip);

            self.shape2d_render.draw_style(edge_color, &self.to_gpu);
            self.bezier_render.draw_style(edge_color, &self.to_gpu);
        }


        return Ok(());
    }

    pub fn draw_markers(
        &mut self, 
        path: &Path<Canvas>, 
        xy: &Tensor,
        scale: &Tensor,
        color: &Tensor<u32>,
        style: &dyn PathOpt, 
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        let path = transform_solid_path(path);

        let face_color = match style.get_face_color() {
            Some(color) => *color,
            None => Color(0x000000ff)
        };

        let edge_color = match style.get_edge_color() {
            Some(color) => *color,
            None => face_color
        };

        if path.is_closed_path() && ! face_color.is_none() {
            self.fill_path(&path, clip);

            for (i, xy) in xy.iter_row().enumerate() {
                let affine = marker_affine(xy[0], xy[1], i, scale);
                let color = marker_color(i, color, face_color);

                self.shape2d_render.draw_style(color, &self.to_gpu.matmul(&affine));
                self.bezier_render.draw_style(color, &self.to_gpu.matmul(&affine));
            }

            if face_color != edge_color && ! edge_color.is_none() {
                self.draw_lines(&path, style, clip);

                for (i, xy) in xy.iter_row().enumerate() {
                    let affine = marker_affine(xy[0], xy[1], i, scale);
    
                    self.shape2d_render.draw_style(edge_color, &self.to_gpu.matmul(&affine));
                    self.bezier_render.draw_style(edge_color, &self.to_gpu.matmul(&affine));
                }
            }
        } else if ! edge_color.is_none() {
            self.draw_lines(&path, style, clip);

            for (i, xy) in xy.iter_row().enumerate() {
                let affine = marker_affine(xy[0], xy[1], i, scale);
                let color = marker_color(i, color, edge_color);

                self.shape2d_render.draw_style(color, &self.to_gpu.matmul(&affine));
                self.bezier_render.draw_style(color, &self.to_gpu.matmul(&affine));
            }
        }

        Ok(())
    }

    pub fn font(
        &mut self,
        style: &FontStyle,
    ) -> Result<FontTypeId, RenderErr> {
        if let Some(family) = style.get_family() {
            let font_id = self.text_render.font(family);

            Ok(FontTypeId(font_id.0)) // i()))
        } else {
            Err(RenderErr::NotImplemented)            
        }
    }

    pub fn draw_text(
        &mut self,
        xy: Point, // location in Canvas coordinates
        text: &str,
        angle: f32,
        style: &dyn PathOpt, 
        text_style: &TextStyle,
        _clip: &Clip,
    ) -> Result<(), RenderErr> {

        let color = match style.get_face_color() {
            Some(color) => *color,
            None => Color(0x000000ff),
        };

        let size = match &text_style.get_size() {
            Some(size) => *size,
            None => 10.,
        };

        let size = self.to_px(size);

        let halign = match text_style.get_width_align() {
            Some(align) => align.clone(),
            None => HorizAlign::Center,
        };

        let valign = match text_style.get_height_align() {
            Some(align) => align.clone(),
            None => VertAlign::Bottom,
        };

        let font_id = match text_style.get_font() {
            Some(type_id) => FontId(type_id.0),
            None => self.font_id_default,
        };
        // let font_id = self.text_render.font("sans-serif");

        self.text_render.draw(
            text,
            font_id,
            size,
            xy, 
            Point(self.bounds.width(), self.bounds.height()),
            color,
            angle,
            halign,
            valign,
        );
 
        Ok(())
    }

    pub fn draw_triangles(
        &mut self,
        vertices: Tensor<f32>,  // Nx2 x,y in canvas coordinates
        rgba: Tensor<u32>,    // N in rgba
        triangles: Tensor<u32>, // Mx3 vertex indices
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        assert!(vertices.rank() == 2, 
            "vertices must be 2d (rank2) shape={:?}",
            vertices.shape().as_slice());
        assert!(vertices.cols() == 2, 
            "vertices must be rows of 2 columns (x, y) shape={:?}",
            vertices.shape().as_slice());
        assert!(rgba.rank() == 1,
            "colors must be a 1D vector shape={:?}",
            rgba.shape().as_slice());
        assert!(vertices.rows() == rgba.cols(), 
            "number of vertices and colors must match. vertices={:?} colors={:?}",
            vertices.shape().as_slice(), rgba.shape().as_slice());
        assert!(triangles.cols() == 3, 
            "triangle indices must have 3 vertices (3 columns) shape={:?}",
            triangles.shape().as_slice());

        self.triangle_render.start_triangles();

        for (xy, color) in vertices.iter_row().zip(rgba.iter()) {
            self.triangle_render.draw_vertex(xy[0], xy[1], *color);
        }

        for tri in triangles.iter_row() {
            self.triangle_render.draw_triangle(tri[0], tri[1], tri[2]);
        }

        self.triangle_render.draw_style(&self.to_gpu);

        Ok(())
    }

    pub fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId {
        self.form3d_render.create_form(form)
    }

    pub fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
        clip: &Clip,
    ) -> Result<(), RenderErr> {
        let clip = match clip {
            Clip::None => Clip::None,
            Clip::Bounds(p0, p1) => {
                Clip::Bounds(
                    //Point(p0.0, p0.1),
                    //Point(p1.0, p1.1) 
                    Point(p0.0, self.bounds().ymax() - p1.1),
                    Point(p1.0, self.bounds().ymax() - p0.1) 
                )
            }
        };

        self.form3d_render.camera(camera);
        self.form3d_render.draw_form(form, &clip);
        
        Ok(())
    }

    fn _draw_image(
        &mut self,
        device: &wgpu::Device,
        bounds: &Bounds<Canvas>,  // Nx2 x,y in canvas coordinates
        colors: &Tensor<u8>,    // N in rgba
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        assert!(colors.rank() == 3, "colors rank must be 3 shape={:?}", colors.shape().as_slice());
        assert!(colors.cols() == 4, "colors must have 4-width columns shape={:?}", colors.shape().as_slice());

        self.image_render._draw(device, bounds, colors, &self.to_gpu);

        Ok(())
    }

    pub fn create_image(&mut self, device: &wgpu::Device, colors: &Tensor<u8>) -> ImageId {
        assert!(colors.rank() == 3, "colors rank must be 3 shape={:?}", colors.shape().as_slice());
        assert!(colors.cols() == 4, "colors must have 4-width columns shape={:?}", colors.shape().as_slice());

        self.image_render.create_image(device, colors)
    }

    pub fn create_texture(&mut self, image: &Tensor<u8>) -> TextureId {
        assert!(image.rank() == 2, "colors rank must be 2 shape={:?}", image.shape().as_slice());

        //self.shape2d_render.add_texture(image.rows(), image.cols(), image.as_slice())
        todo!();
    }

    pub fn create_texture_rgba8(
        &mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        image: &Tensor<u8>
    ) -> TextureId {
        assert!(image.rank() == 3, "texture rank must be 3 shape={:?}", image.shape().as_slice());
        assert!(image.cols() == 4, "texture cols 4 shape={:?}", image.shape().as_slice());

        self.form3d_render.create_texture_rgba8(
            device, 
            queue, 
            image
        )
    }

    pub fn draw_image_ref(
        &mut self,
        device: &wgpu::Device,
        bounds: &Bounds<Canvas>,  // Nx2 x,y in canvas coordinates
        image: ImageId,    // N in rgba
        _clip: &Clip,
    ) -> Result<(), RenderErr> {
        self.image_render.draw_image(device, bounds, &image, &self.to_gpu);

        Ok(())
    }

    pub(crate) fn draw(
        &mut self,
        figure: &mut Box<dyn Drawable>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        // encoder: &mut wgpu::CommandEncoder,
    ) {
        //let (width, height) = bounds;

        //self.clear();

        //self.resize(device, width, height);
        //let pt_to_px_factor = 4. / 3.;
        //self.set_scale_factor(scale_factor * pt_to_px_factor);
        //let draw_bounds = self.canvas.bounds().clone();

        let pos = self.bounds().clone();
        // let mut renderer = self.renderer(device, queue, view);
        // figure.update(&mut renderer, &pos);
        // renderer.flush_inner(&Clip::None);

        /*
        let mut renderer = DrawRenderer::new(
            self, 
            device, 
            Some(queue), 
            Some(view)
        );
        */
        let mut renderer = self.renderer(device, queue, view);

        //figure.draw(&mut renderer, &draw_bounds);
        figure.draw(&mut renderer);

        // renderer.flush_inner(&Clip::None);
    }

    pub fn renderer<'a>(
        &'a mut self, 
        device: &'a wgpu::Device, 
        queue: &'a wgpu::Queue, 
        view: &'a wgpu::TextureView
    ) -> PlotRenderer<'a> {
        self.clear();

        PlotRenderer::new(self, device, Some(queue), Some(view))
    }
}

pub(crate) fn line_normal(
    p0: Point, 
    p1: Point, 
    lw2: f32, 
) -> (f32, f32) {
    let dx = p1.x() - p0.x();
    let dy = p1.y() - p0.y();

    let len = dx.hypot(dy).max(f32::EPSILON);

    let dx = dx / len;
    let dy = dy / len;

    // normal to the line
    let nx = dy * lw2;
    let ny = dx * lw2;

    (nx, ny)
}

pub(crate) fn line_intersection(
    p0: Point, 
    p1: Point, 
    q0: Point, 
    q1: Point
) -> Point {
    let mut det = (p0.x() - p1.x()) * (q0.y() - q1.y())
        - (p0.y() - p1.y()) * (q0.x() - q1.x());

    if det.abs() <= f32::EPSILON {
        return p0; // p0 is marker for coincident or parallel lines
    } else if det.abs() < 0.2 {
        // clamp long extensions for miter
        det = 0.2 * det.signum();
    }


    let p_xy = p0.x() * p1.y() - p0.y() * p1.x();
    let q_xy = q0.x() * q1.y() - q0.y() * q1.x();

    let x = (p_xy * (q0.x() - q1.x()) - (p0.x() - p1.x()) * q_xy) / det;
    let y = (p_xy * (q0.y() - q1.y()) - (p0.y() - p1.y()) * q_xy) / det;

    Point(x, y)
}

// transform and normalize path
fn transform_solid_path(path: &Path<Canvas>) -> Path<Canvas> {
    let mut codes = Vec::<PathCode>::new();

    let mut p0 = Point(0.0f32, 0.0f32);

    // TODO: clip and compress
    for code in path.codes() {
        p0 = match code {
            PathCode::MoveTo(p0) => {
                codes.push(PathCode::MoveTo(*p0));

                *p0
            }
            PathCode::LineTo(p1) => {
                codes.push(PathCode::LineTo(*p1));

                *p1
            }
            PathCode::Bezier2(p1, p2) => {
                codes.push(PathCode::Bezier2(*p1, *p2));

                *p2
            }
            PathCode::Bezier3(p1, p2, p3) => {
                let p1 = *p1;
                let p2 = *p2;
                let p3 = *p3;

                // Truong, et. al. 2020
                // Quadratic Approximation of Cubic Curves
                // 
                // Note: if more accuracy is needed, the cubic can also be 
                // split into two cubics before converting to quadratics

                // q0_1 = b0 + 0.75 (b1 - b0)
                let q0_1 = Point(
                    p0.x() + 0.75 * (p1.x() - p0.x()),
                    p0.y() + 0.75 * (p1.y() - p0.y()),
                );

                // q1_1 = b3 + 0.75 (b2 - b3)
                let q1_1 = Point(
                    p3.x() + 0.75 * (p2.x() - p3.x()),
                    p3.y() + 0.75 * (p2.y() - p3.y()),
                );

                // q0_2 = q1_0 = 0.5 * (q0_1 + q1_1)
                let q0_2 = Point(
                    0.5 * (q0_1.x() + q1_1.x()),
                    0.5 * (q0_1.y() + q1_1.y()),
                );

                codes.push(PathCode::Bezier2(q0_1, q0_2));
                codes.push(PathCode::Bezier2(q1_1, p3));

                p3
            }
            PathCode::ClosePoly(p1) => {
                codes.push(PathCode::ClosePoly(*p1));

                *p1
            }
        }
    }

    Path::<Canvas>::new(codes)
}

fn marker_affine(x: f32, y: f32, i: usize, scale: &Tensor) -> Affine2d {
    let mut affine = Affine2d::eye();

    // optional scaling
    if scale.len() > 1 {
       affine = match scale.cols() {
            1 => affine.scale(scale[i], scale[i]),
            2 => affine.scale(scale[(i, 0)], scale[(i, 1)]),
            _ => panic!("Marker scale must be 1 or 2 dimensional {:?}", scale.shape().as_slice())
        }
    } else if scale.len() == 1 {
        affine = match scale.cols() {
            1 => affine.scale(scale[0], scale[0]),
            2 => affine.scale(scale[(0, 0)], scale[(0, 1)]),
            _ => panic!("Marker scale must be 1 or 2 dimensional {:?}", scale.shape().as_slice())
        }
    }

    affine.translate(x, y)
}

fn marker_color(i: usize, color: &Tensor<u32>, default: Color) -> Color {
    if color.len() == 0 {
        default
    } else if color.len() == 1 {
        Color(color[0])
    } else {
        Color(color[i])
    }
}

fn transform_dashed_path(path: &Path<Canvas>, pattern: Vec<f32>) -> Path<Canvas> {
    let mut codes = Vec::<PathCode>::new();

    let mut p0 = Point(0.0f32, 0.0f32);
    let mut moveto = p0;

    let mut cursor = Cursor::new(pattern);

    for code in path.codes() {
        p0 = match code {
            PathCode::MoveTo(p0) => {
                //codes.push(PathCode::MoveTo(*p0));
                cursor.reset();
                moveto = *p0;

                *p0
            }
            PathCode::LineTo(p1) => {
                // codes.push(PathCode::LineTo(*p1));
                add_dash_line(&mut codes, &mut cursor, p0, *p1)
            }
            PathCode::Bezier2(_, p2) => {
                add_dash_line(&mut codes, &mut cursor, p0, *p2)
            }
            PathCode::Bezier3(_, _, p3) => {
                add_dash_line(&mut codes, &mut cursor, p0, *p3)
            }
            PathCode::ClosePoly(p1) => {
                //codes.push(PathCode::LineTo(*p1));
                add_dash_line(&mut codes, &mut cursor, p0, *p1);
                add_dash_line(&mut codes, &mut cursor, *p1, moveto)
            }
        }
    }

    Path::<Canvas>::new(codes)
}

fn add_dash_line(
    codes: &mut Vec::<PathCode>, 
    cursor: &mut Cursor,
    p0: Point,
    p1: Point,
) -> Point {
    let dx = p1.x() - p0.x();
    let dy = p1.y() - p0.y();

    let len = dx.hypot(dy);

    if len < 1. {
        return p0;
    }

    if cursor.is_visible() && cursor.is_start() {
        codes.push(PathCode::MoveTo(p0));
    }

    let mut offset = 0.;
    let len_r = len.recip();

    while offset < len {
        let sublen = cursor.sublen();

        let tail = len - offset;
        if tail <= sublen {
            if cursor.is_visible() {
                codes.push(PathCode::LineTo(p1));
            }
            cursor.add(tail);
            return p1;
        } else {
            offset += sublen;

            let p = Point(
                p0.x() + dx * offset * len_r,
                p0.y() + dy * offset * len_r,
            );

            if cursor.is_visible() {
                codes.push(PathCode::LineTo(p));
            } else if offset < len {
                codes.push(PathCode::MoveTo(p));
            }

            cursor.next();
        }
    }

    p1
}   

struct Cursor {
    dashes: Vec<f32>,
    i: usize,
    t: f32,
}

impl Cursor {
    fn new(pattern: Vec<f32>) -> Self {
        Self {
            dashes: pattern,
            i: 0,
            t: 0.,
        }
    }

    #[inline]
    fn reset(&mut self) {
        self.i = 0;
        self.t = 0.;
    }

    #[inline]
    fn is_start(&mut self) -> bool {
        self.t == 0.
    }

    #[inline]
    fn sublen(&self) -> f32 {
        self.dashes[self.i] - self.t
    }

    #[inline]
    fn add(&mut self, len: f32) {
        self.t += len;
    }

    #[inline]
    fn is_visible(&self) -> bool {
        self.i % 2 == 0
    }

    #[inline]
    fn next(&mut self) {
        self.i = (self.i + 1) % self.dashes.len();
        self.t = 0.;
    }
}
