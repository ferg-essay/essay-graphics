use std::collections::HashMap;

use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, 
    input::Input, path_style::MeshStyle, 
    renderer::{Canvas, RenderErr, Renderer, Result}, 
    Affine2d, BezierMesh2d, Bounds, CapStyle, Clip, Color, FontStyle, FontTypeId, 
    Hatch, HorizAlign, JoinStyle, LineStyle, Mesh2d, Mesh2dColor, 
    Path, PathCode, PathOpt, Point, Size, TextStyle, TextureId, VertAlign
};
use essay_tensor::tensor::Tensor;
use wgpu::util::StagingBelt;

use crate::render::render::{render_draw_inner, RenderWgpu};
use super::{
    bezier_mesh::BezierMeshRender, form3d::Form3dRender, hatch::init_hatch, lines::lines, 
    mesh2d::Mesh2dRender, mesh2d_color::Mesh2dColorRender, 
    text::TextRender, text_cache::FontId, 
    texture_store::TextureCache, triangulate3::fill_shape
};

pub struct PlotCanvas {
    bounds: Bounds<Canvas>,
    scale_factor: f32,
    input: Input,

    mesh2d_render: Mesh2dRender,
    bezier_mesh_render: BezierMeshRender,

    mesh2d_color_render: Mesh2dColorRender,

    text_render: TextRender,

    form3d_render: Form3dRender,

    texture_store: TextureCache,
    hatch_map: HashMap<Hatch, TextureId>,

    staging: Option<StagingBelt>,

    font_id_default: FontId,

    to_gpu: Affine2d,

    cache_size: Size,
    is_request_redraw: bool,
}

impl PlotCanvas {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
        scale_factor: f32,
    ) -> Self {
        let mesh2d_render = Mesh2dRender::new(device, format);
        let bezier_mesh_render = BezierMeshRender::new(device, format);

        let mesh2d_color_render = Mesh2dColorRender::new(device, format);

        let mut text_render = TextRender::new(device, format, 512, 512);

        let font_id_default = text_render.font("default");

        let form3d_render = Form3dRender::new(device, format, width, height);

        let staging = StagingBelt::new(2048 * 128);

        let mut texture_store = TextureCache::new(device, queue);

        let hatch_map = init_hatch(device, queue, &mut texture_store);

        let mut canvas = Self {
            bounds: Bounds::from([width as f32, height as f32]),

            cache_size: Size::default(),
            input: Input::default(),
            scale_factor: 4. / 3. * scale_factor,

            mesh2d_render,
            bezier_mesh_render,
            mesh2d_color_render,
            form3d_render,
            text_render,

            font_id_default,
            texture_store,
            hatch_map,

            staging: Some(staging),
            to_gpu: Affine2d::eye(),

            is_request_redraw: false,
        };

        canvas.input.size = Size(width as f32, height as f32);
        canvas.resize(&device);

        canvas
    }

    pub fn request_redraw(&mut self, is_redraw: bool) {
        self.is_request_redraw = is_redraw;
    }

    pub fn draw<'a, R>(
        &'a mut self,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        view: Option<&'a wgpu::TextureView>,
        is_flush: bool,
        draw: impl FnOnce(&mut dyn Renderer) -> Result<R> + 'a
    ) -> Result<R> {
        self.clear();

        if self.cache_size != self.input.size {
            self.resize(device);
        }        

        let staging = self.take_staging();

        let (result, staging) = render_draw_inner(self, device, queue, view, staging, is_flush, draw);

        self.replace_staging(staging);

        result
    }
    
    pub fn clear(&mut self) {
    }

    pub fn resize(&mut self, device: &wgpu::Device) -> bool {
        if self.cache_size == self.input.size || self.input.size.width() == 0. {
            return false;
        }

        self.cache_size = self.input.size;
        self.request_redraw(true);
        self.bounds = Bounds::from(self.input.size);

        let pos_gpu = Bounds::<Canvas>::new(
            Point(-1., -1.),
            Point(1., 1.)
        );
        
        self.to_gpu = self.bounds.affine_to(&pos_gpu);

        if self.input.size.width() > 0. {
            self.form3d_render.resize(device, self.cache_size.width() as u32, self.cache_size.height() as u32);
        }

        true
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
    pub fn bounds(&self) -> Bounds<Canvas> {
        self.bounds
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        // traditional pt to px
        let pt_to_px = 4. / 3.;

        self.scale_factor = scale_factor * pt_to_px;
        //self.scale_factor = 1.; // TK
    }

    #[inline]
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    #[inline]
    pub fn to_px(&self, size: f32) -> f32 {
        self.scale_factor * size
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn set_input(&mut self, input: &Input) {
        self.input = input.clone();
    }

    pub(crate) fn draw_bezier_mesh(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &BezierMesh2d, 
        _texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        let style: Vec<MeshStyle> = style.iter().map(|marker| {
            MeshStyle {
                color: marker.color,
                affine: marker.affine.compose(&self.to_gpu),
            }
        }).collect();

        self.bezier_mesh_render.draw(wgpu, mesh, style.as_slice());

        Ok(())
    }

    pub(crate) fn draw_mesh2d(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        let style: Vec<MeshStyle> = style.iter().map(|marker| {
            MeshStyle {
                color: marker.color,
                affine: marker.affine.compose(&self.to_gpu),
            }
        }).collect();

        self.mesh2d_render.draw(
            wgpu, 
            &self.texture_store, 
            mesh, 
            texture,
            style.as_slice(),
        );

        Ok(())
    }

    pub(crate) fn draw_mesh2d_color(
        &mut self, 
        wgpu: &mut RenderWgpu,
        mesh: &Mesh2dColor,
    ) -> Result<(), RenderErr> {
        self.mesh2d_color_render.draw(
            wgpu, 
            mesh, 
            &self.to_gpu,
        );

        Ok(())
    }

    pub(crate) fn draw_path(
        &mut self, 
        wgpu: &mut RenderWgpu,
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
    ) -> Result<(), RenderErr> {
        let mut face_color = style.get_face_color()
            .unwrap_or(Color::black());

        let mut edge_color = style.get_edge_color()
            .unwrap_or(face_color);

        if let Some(alpha) = style.get_alpha() {
            face_color = face_color.with_alpha(alpha * face_color.alpha());
            edge_color = edge_color.with_alpha(alpha * edge_color.alpha());
        }

        let texture = TextureId::default();

        let path = match style.get_line_style() {
            Some(LineStyle::Solid) | None => {
                transform_solid_path(path)
            }
            Some(line_style) => {
                let lw = match style.get_line_width() {
                    Some(lw) => self.to_px(lw),
                    None => self.to_px(2.),
                };
                
                let pattern = line_style.to_pattern(lw);

                transform_dashed_path(path, pattern)
            },
        };

        if path.is_closed_path() && face_color.alpha() > 0. {
            let mut is_texture = false;

            if let Some(hatch) = style.get_hatch() {
                let (mesh, bezier) = fill_shape(&path);

                let texture = self.hatch_map[&hatch];

                let style = vec![(face_color, &self.to_gpu).into()];

                self.mesh2d_render.draw(wgpu, &self.texture_store, &mesh, texture, &style);
                self.bezier_mesh_render.draw(wgpu, &bezier, &style);

                is_texture = true;
            } else if let Some(texture) = style.get_texture() {
                let (mesh, bezier) = fill_shape(&path);
                let style = vec![(face_color, &self.to_gpu).into()];

                self.mesh2d_render.draw(wgpu, &self.texture_store, &mesh, texture, &style);
                self.bezier_mesh_render.draw(wgpu, &bezier, &style);

                is_texture = true;
            } else {
                let (mesh, bezier) = fill_shape(&path);

                let style = vec![(face_color, &self.to_gpu).into()];

                self.mesh2d_render.draw(wgpu, &self.texture_store, &mesh, texture, &style);
                self.bezier_mesh_render.draw(wgpu, &bezier, &style);
            }

            if (face_color != edge_color || is_texture) && edge_color.alpha() > 0. {
                self.draw_lines2(wgpu, &path, style, &vec![(edge_color, &self.to_gpu).into()]);
            }
        } else if edge_color.alpha() > 0. {
            self.draw_lines2(wgpu, &path, style, &vec![(edge_color, &self.to_gpu).into()]);
        }


        return Ok(());
    }

    pub(crate) fn draw_markers(
        &mut self, 
        wgpu: &mut RenderWgpu,
        path: &Path<Canvas>, 
        path_style: &dyn PathOpt, 
        marker_style: &[MeshStyle],
    ) -> Result<(), RenderErr> {
        let path = transform_solid_path(path);

        let face_color = match path_style.get_face_color() {
            Some(color) => color,
            None => Color(0x000000ff)
        };

        let edge_color = match path_style.get_edge_color() {
            Some(color) => color,
            None => face_color
        };

        let texture = TextureId::default();

        let marker_style: Vec<MeshStyle> = marker_style.iter()
            .map(|style| {
                MeshStyle::from((style.color, &self.to_gpu.matmul(&style.affine)))
            }).collect();

        if path.is_closed_path() && ! face_color.is_none() {
            let (mesh, bezier) = fill_shape(&path);

            self.mesh2d_render.draw(wgpu, &self.texture_store, &mesh, texture, &marker_style);
            self.bezier_mesh_render.draw(wgpu, &bezier, &marker_style);

            if face_color != edge_color && ! edge_color.is_none() {
                self.draw_lines2(wgpu, &path, path_style, &marker_style);
            }
        } else if ! edge_color.is_none() {
            self.draw_lines2(wgpu, &path, path_style, &marker_style);
        }

        Ok(())
    }

    fn draw_lines2(
        &mut self, 
        wgpu: &mut RenderWgpu,
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
        styles: &Vec<MeshStyle>,
    ) {
        let linewidth  = style.get_line_width().unwrap_or(0.5);

        if linewidth <= 0. {
            return;
        }

        let joinstyle  = style.get_join_style()
            .unwrap_or(JoinStyle::Bevel);

        let capstyle  = style.get_cap_style()
            .unwrap_or(CapStyle::Butt);
        
        let linewidth = self.to_px(linewidth); // / self.canvas.width();
        let texture = TextureId::default();

        if let Some((mesh, bezier)) = lines(path, joinstyle, capstyle, linewidth) {
            self.mesh2d_render.draw(wgpu, &self.texture_store, &mesh, texture, styles);
            self.bezier_mesh_render.draw(wgpu, &bezier, styles);
        }
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
    ) -> Result<(), RenderErr> {
        if text.len() == 0 { // todo: more sophisticated validation
            return Ok(());
        }

        let color = match style.get_face_color() {
            Some(color) => color,
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

    pub fn text_size(
        &mut self,
        text: &str,
        text_style: &TextStyle,
    ) -> Size {
        let size = text_style.get_size().map_or(10., |s| s);
        let size = self.to_px(size);

        let font_id = text_style.get_font().map_or(
            self.font_id_default,
            |id| FontId(id.0)
        );

        self.text_render.text_size(
            text,
            font_id,
            size
        )
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
    ) -> Result<(), RenderErr> {
        self.form3d_render.camera(camera);
        self.form3d_render.draw_form(form);
        
        Ok(())
    }

    pub fn create_texture(&mut self, image: &Tensor<u8>) -> TextureId {
        assert!(image.rank() == 2, "colors rank must be 2 shape={:?}", image.shape().as_vec());

        //self.shape2d_render.add_texture(image.rows(), image.cols(), image.as_slice())
        todo!();
    }

    pub fn create_texture_rgba8(
        &mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        image: &Tensor<u8>
    ) -> TextureId {
        assert!(image.rank() == 3, "texture requires rank 3 shape={:?}", image.shape().as_vec());
        assert!(image.cols() == 4, "texture requires 4 columns shape={:?}", image.shape().as_vec());
    
        self.texture_store.add_rgba_u8(
            device, 
            queue, 
            image.dim(1) as u32, 
            image.dim(0) as u32, 
            image.as_slice()
        )
    }

    pub(crate) fn flush(&mut self, wgpu: &mut RenderWgpu) {
        self.bezier_mesh_render.flush(wgpu);
        self.mesh2d_render.flush(wgpu, &self.texture_store);
        self.mesh2d_color_render.flush(wgpu);
        self.text_render.flush(wgpu);
        self.form3d_render.flush(wgpu, &self.texture_store);
     }
    
    pub(crate) fn take_staging(&mut self) -> wgpu::util::StagingBelt {
        self.staging.take().unwrap()
    }
    
    pub(crate) fn replace_staging(&mut self, staging: wgpu::util::StagingBelt) {
        assert!(self.staging.is_none());

        self.staging.replace(staging);
    }
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
