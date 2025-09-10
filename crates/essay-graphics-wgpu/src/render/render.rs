use std::{mem, num::NonZero};

use essay_graphics_api::{
    form::{Form, FormId, Matrix4}, input::Input, path_style::MeshStyle, renderer::{Canvas, GraphicsContext, Mesh2dBuffer, Pos, RenderErr, Renderer, Result}, Affine2d, BezierMesh2d, Bounds, CapStyle, Color, FontStyle, FontTypeId, HorizAlign, JoinStyle, LineStyle, Mesh2d, Mesh2dColor, Path, PathCode, PathOpt, Point, Shapes, Size, TextStyle, TextureId, VertAlign
};
use essay_tensor::tensor::Tensor;
use wgpu::util::StagingBelt;

use crate::render::{lines::lines, text_atlas::FontId, triangulate3::fill_shape, RenderCanvas};

pub struct PlotRenderer<'a, 'b> {
    wgpu: &'b mut RenderWgpu<'a>,
    canvas: &'b mut RenderCanvas,
    input: &'b Input,
    pos: Pos,
}

impl<'a, 'b> PlotRenderer<'a, 'b> {
    pub(crate) fn render<R>(
        wgpu: &'b mut RenderWgpu<'a>,
        canvas: &'b mut RenderCanvas,
        input: &'b Input,
        draw: impl FnOnce(&mut dyn Renderer) -> Result<R>,
    ) -> Result<R> {
        let mut renderer = Self {
            wgpu,
            canvas,
            input,
            pos: Pos::from(input.size),
        };

        let result = (draw)(&mut renderer)?;

        renderer.flush_inner()?;

        Ok(result)
    }

    fn flush_inner(&mut self) -> Result<()> {
        self.canvas.flush(self.wgpu);
        let result = self.canvas.pipeline.flush(self.wgpu);
        self.wgpu.flush();

        if result {
            Ok(())
        } else {
            Err(RenderErr::RedrawRequired)
        }
    }

    fn get_scissor(&self) -> Option<(u32, u32, u32, u32)> {
        let pos = &self.pos;

        Some((
            pos.xmin() as u32, 
            pos.ymin() as u32,
            pos.width() as u32, 
            pos.height() as u32
        ))
    }

    fn update_scissor(&mut self) {
        let scissor = self.get_scissor();

        self.wgpu.scissor = scissor;
    }

    fn draw_lines2(
        &mut self, 
        path: &Path<Canvas>, 
        style: &dyn PathOpt, 
        styles: &[MeshStyle],
    ) -> Result<()> {
        let linewidth  = style.get_line_width().unwrap_or(0.5);

        if linewidth <= 0. {
            return Ok(());
        }

        let joinstyle  = style.get_join_style()
            .unwrap_or(JoinStyle::Bevel);

        let capstyle  = style.get_cap_style()
            .unwrap_or(CapStyle::Butt);
        
        let linewidth = self.to_px(linewidth); // / self.canvas.width();
        let texture = TextureId::default();

        if let Some((mesh, bezier)) = lines(path, joinstyle, capstyle, linewidth) {
            self.draw_mesh2d(&mesh, texture, styles)?;
            self.draw_bezier_mesh(&bezier, texture, styles)
        } else {
            Ok(())
        }
    }

    ///
    /// draw a text item
    /// 
    fn draw_text_impl(
        &mut self, 
        text: &str, 
        font_id: FontId, 
        size: f32,
        pos: Point, 
        color: Color,
        angle: f32,
        halign: HorizAlign,
        valign: VertAlign,
    ) {
        let x0 = pos.x;
        let y0 = pos.y;

        let mut mesh = Mesh2d::new();

        let text_size = size.round() as u16;

        let s = self.canvas.text_cache.glyph(font_id, text_size, ' ');
        let w_space = s.advance_width;
        
        let mut x = x0; // x0.floor();
        let y = y0.floor(); // (y0 + s.ascent).floor(); // y.floor
        let mut is_first = true;

        for ch in text.chars() {
            let r = self.canvas.text_cache.glyph(font_id, text_size, ch);
            
            //x = x.round();

            if r.is_none() || ch == ' ' {
                x += w_space;
                continue;
            }

            let y_ch = (y + r.dy).floor(); // (y - r.dy).floor();// - r.h as f32;
            let x_ch = if is_first {
                (x).floor()
            } else {
                (x + r.lsb).floor()
            };

            is_first = false;

            let w = r.w; // .ceil();
            let h = r.h; // .ceil();

            mesh.rect_uv(
                ([x_ch, y_ch], [r.tx_min, r.ty_max]),
                ([x_ch + w, y_ch + h], [r.tx_max, r.ty_min]),
            );

            x += r.advance_width;
        }

        let dx = match halign {
            HorizAlign::Left => 0.,
            HorizAlign::Center => - 0.5 * (x - x0),
            HorizAlign::Right => - (x - x0),
        };

        let descent = 0.;

        let dy = match valign {
            VertAlign::Top => - size - descent,
            VertAlign::Center => - 0.5 * (size + descent),
            VertAlign::BaselineBottom => 0.,
            VertAlign::Bottom => - descent,
        };

        let mut affine = Affine2d::eye();
        if angle != 0. {
            affine = affine.rotate_around(0.5 * (x0 + x), y0, angle)
        }

        affine = affine.translate(dx, dy);

        let style = MeshStyle {
            color,
            affine,
        };

        let texture_id = self.canvas.font_texture_id(font_id, size);

        self.draw_mesh2d(&mesh, texture_id, &[style]).unwrap();
    }
}

impl<'a, 'b> Renderer for PlotRenderer<'a, 'b> {
    fn extent(&self) -> Bounds<Canvas> {
        self.canvas.pos()
    }

    fn pos(&self) -> Bounds<Canvas> {
        self.pos
    }

    fn scale_factor(&self) -> f32 {
        //self.input.scale_factor * 4. / 3.
        2. * 4. / 3.
    }

    fn to_px(&self, size: f32) -> f32 {
        self.scale_factor() * size
    }

    fn input(&self) -> &Input {
        self.input
    }

    fn draw_path(
        &mut self, 
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

                let texture = self.canvas.hatch_id(hatch);

                let style = vec![(face_color, &Affine2d::eye()).into()];

                self.draw_mesh2d(&mesh, texture, &style)?;
                self.draw_bezier_mesh(&bezier, texture, &style)?;

                is_texture = true;
            } else if let Some(texture) = style.get_texture() {
                let (mesh, bezier) = fill_shape(&path);
                let style = vec![(face_color, &Affine2d::eye()).into()];

                self.draw_mesh2d(&mesh, texture, &style)?;
                self.draw_bezier_mesh(&bezier, texture, &style)?;

                is_texture = true;
            } else {
                let (mesh, bezier) = fill_shape(&path);

                // self.to_gpu
                let style = vec![(face_color, &Affine2d::eye()).into()];

                self.draw_mesh2d(&mesh, texture, style.as_slice())?;
                self.draw_bezier_mesh(&bezier, texture, style.as_slice())?;
            }

            if (face_color != edge_color || is_texture) && edge_color.alpha() > 0. {
                self.draw_lines2(&path, style, &vec![(edge_color, &Affine2d::eye()).into()])?;
            }
        } else if edge_color.alpha() > 0. {
            self.draw_lines2(&path, style, &vec![(edge_color, &Affine2d::eye()).into()])?;
        }


        return Ok(());
    }

    fn draw_markers(
        &mut self, 
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

        if path.is_closed_path() && ! face_color.is_none() {
            let (mesh, bezier) = fill_shape(&path);

            self.draw_mesh2d(&mesh, texture, &marker_style)?;
            self.draw_bezier_mesh(&bezier, texture, &marker_style)?;

            if face_color != edge_color && ! edge_color.is_none() {
                self.draw_lines2(&path, path_style, marker_style)?;
            }
        } else if ! edge_color.is_none() {
            self.draw_lines2(&path, path_style, marker_style)?;
        }

        Ok(())
    }
    
    fn draw_bezier_mesh(
        &mut self,
        mesh: &BezierMesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()> {
        let style: Vec<MeshStyle> = style.iter().map(|marker| {
            MeshStyle {
                color: marker.color,
                affine: marker.affine.compose(&self.canvas.to_gpu),
            }
        }).collect();

        self.canvas.pipeline.draw_bezier_mesh(self.wgpu, mesh, texture, style.as_slice())
    }
    
    fn draw_mesh2d(
        &mut self,
        mesh: &Mesh2d,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()> {
        let style: Vec<MeshStyle> = style.iter().map(|marker| {
            MeshStyle {
                color: marker.color,
                affine: marker.affine.compose(&self.canvas.to_gpu),
            }
        }).collect();

        self.canvas.pipeline.draw_mesh2d(self.wgpu, mesh, texture, style.as_slice())
    }
    
    fn create_mesh2d_buffer(
        &mut self,
        mesh: &Mesh2d,
    ) -> Result<Mesh2dBuffer> {
        self.canvas.pipeline.create_mesh2d_buffer(self.wgpu, mesh)
    }
    
    fn draw_mesh2d_buffer(
        &mut self,
        mesh: &Mesh2dBuffer,
        texture: TextureId,
        style: &[MeshStyle],
    ) -> Result<()> {
        let style: Vec<MeshStyle> = style.iter().map(|marker| {
            MeshStyle {
                color: marker.color,
                affine: marker.affine.compose(&self.canvas.to_gpu),
            }
        }).collect();

        self.canvas.pipeline.draw_mesh2d_buffer(self.wgpu, mesh, texture, style.as_slice())
    }
    
    fn draw_mesh2d_color(
        &mut self,
        mesh: &Mesh2dColor,
    ) -> Result<()> {
        self.canvas.pipeline.draw_mesh2d_color(self.wgpu, mesh, &self.canvas.to_gpu)
    }
    
    fn draw_shape(
        &mut self, 
        shape: &Shapes,
    ) -> Result<()> {
        self.canvas.pipeline.draw_shape(self.wgpu, shape, TextureId::default())
    }

    fn font(
        &mut self,
        style: &FontStyle
    ) -> Result<FontTypeId, RenderErr> {
        self.canvas.font(style)
    }

    fn draw_text(
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
            None => self.canvas.text_cache.font_id("default"),
        };

        self.draw_text_impl(
            text,
            font_id,
            size,
            xy, 
            color,
            angle,
            halign,
            valign,
        );
 
        Ok(())
    }

    fn text_size(
        &mut self, 
        text: &str,
        text_style: &TextStyle,
    ) -> Size {
        let mut font_set = self.canvas.font_context.default_font_set();

        let size = self.to_px(text_style.get_size().unwrap_or(10.));

        let mut width = 0.;
        let mut height = 0.0f32;
        let mut is_first = true;
        
        for ch in text.chars() {
            let rect = font_set.glyph_size(size, ch);

            height = height.max((rect.ascent + rect.descent) as f32);

            if is_first {
                width += rect.width;
            } else {
                width += rect.width + rect.lsb;
            }

            is_first = false;
        }

        Size::new(width, height)
    }

    fn create_form(
        &mut self,
        form: &Form,
    ) -> FormId {
        self.canvas.pipeline.create_form(form)
    }

    fn draw_form(
        &mut self,
        form: FormId,
        camera: &Matrix4,
    ) -> Result<(), RenderErr> {
        self.canvas.pipeline.draw_form(form, camera)
    }

    fn request_redraw(
        &mut self,
        _bounds: Bounds<Canvas>
    ) {
        self.canvas.request_redraw(true)
    }

    fn create_texture_rgba8(
        &mut self,
        colors: &Tensor<u8>, // [rows, cols, 4]
    ) -> TextureId {
        self.canvas.pipeline.create_texture_rgba8(self.wgpu.device, self.wgpu.queue, colors)
    }

    fn flush(
        &mut self,
    ) {
        // self.flush_inner();
    }

    fn draw_with<'c>(
        &mut self, 
        pos: Bounds<Canvas>, 
        f: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'c>
    ) -> Result<()> {
        let push = Push::new(self, pos);

        (f)(push.ptr)?;

        Ok(())
    }

    fn draw_with_clip<'c>(
        &mut self, 
        pos: Bounds<Canvas>, 
        f: Box<dyn FnOnce(&mut dyn Renderer) -> Result<()> + 'c>
    ) -> Result<()> {
        self.flush();

        let push = Push::new_clip(self, pos);

        (f)(push.ptr)?;

        // push.ptr.flush_inner();

        Ok(())
    }
}

struct Push<'a, 'b, 'c> {
    ptr: &'a mut PlotRenderer<'b, 'c>,

    pos: Bounds<Canvas>,
}

impl<'a, 'b, 'c> Push<'a, 'b, 'c> {
    fn new(renderer: &'a mut PlotRenderer<'b, 'c>, pos: Bounds<Canvas>) -> Self {
        let mut push = Self {
            ptr: renderer,
            pos,
        };

        // renderer._get_scissor();

        mem::swap(&mut push.pos, &mut push.ptr.pos);
        push.ptr.update_scissor();

        push
    }

    fn new_clip(renderer: &'a mut PlotRenderer<'b, 'c>, pos: Bounds<Canvas>) -> Self {
        let mut push = Self {
            ptr: renderer,
            pos,
        };

        mem::swap(&mut push.pos, &mut push.ptr.pos);
        push.ptr.update_scissor();

        push
    }
} 

impl Drop for Push<'_, '_, '_> {
    fn drop(&mut self) {
        mem::swap(&mut self.pos, &mut self.ptr.pos);
        self.ptr.update_scissor();
    }
}


pub(crate) struct RenderWgpu<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub view: &'a wgpu::TextureView,

    pub encoder: Option<wgpu::CommandEncoder>,
    pub staging: StagingBelt,

    pub bounds: Bounds<Canvas>,
    pub scissor: Option<(u32, u32, u32, u32)>,
    pub state: State,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    PreInit,
    Initialized
}

impl<'a> RenderWgpu<'a> {
    pub fn init(&mut self) {
        if self.state == State::PreInit {
            self.state = State::Initialized;

            self.clear_screen(self.view);
        }
    }

    pub fn init_encoder(&mut self) -> &mut wgpu::CommandEncoder {
        if self.encoder.is_none() {
            self.encoder = Some(
                self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
            );

            self.staging.recall();

            self.init();
        };

        self.encoder.as_mut().unwrap()
    }

    pub fn write_buffer(&mut self, target: &wgpu::Buffer, data: &[u8]) {
        let len = NonZero::new(data.len() as u64).unwrap();

        self.init_encoder();

        if let Some(encoder) = &mut self.encoder {
            self.staging.write_buffer(
                encoder,
                target,
                0,
                len,
                self.device,
            ).copy_from_slice(data)
        }
    }

    pub fn write(&mut self, target: &wgpu::Buffer, data: &[u8], offset: u64, len: NonZero<u64>) {
        // let len = NonZero::new(data.len() as u64).unwrap();

        self.init_encoder();

        if let Some(encoder) = &mut self.encoder {
            self.staging.write_buffer(
                encoder,
                target,
                offset,
                len,
                self.device,
            ).copy_from_slice(data)
        }
    }

    pub fn render_pass<'b>(
        &'b mut self,
        draw: impl FnOnce(&mut wgpu::RenderPass<'b>) + 'b
    ) {
        self.init();
        self.init_encoder();

        if let Some(encoder) = &mut self.encoder {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: self.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
            });

            rpass.set_viewport(
                self.bounds.xmin(),
                self.bounds.ymin(),
                self.bounds.width(),
                self.bounds.height(),
                0., 1.,
            );

            if let Some(scissor) = self.scissor {
                rpass.set_scissor_rect(scissor.0, scissor.1, scissor.2, scissor.3);
            }
    
            (draw)(&mut rpass);
        }
    }

    fn clear_screen(&mut self, view: &wgpu::TextureView) {
        self.get_encoder().begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    //    }
    }

    fn get_encoder(&mut self) -> &mut wgpu::CommandEncoder {
        self.init_encoder();

        self.encoder.as_mut().unwrap()
    }

    fn flush(&mut self) {
        if let Some(encoder) = self.encoder.take() {
            self.staging.finish();
            self.queue.submit(Some(encoder.finish()));
        }
    }
}

/*
impl Drop for PlotRenderer<'_, '_> {
    fn drop(&mut self) {
        self.flush_inner();
    }
}
    */

// transform and normalize path
fn transform_solid_path(path: &Path<Canvas>) -> Path<Canvas> {
    let mut codes = Vec::<PathCode>::new();

    let mut p0 = Point::ZERO;

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
                let q0_1 = Point::new(
                    p0.x + 0.75 * (p1.x - p0.x),
                    p0.y + 0.75 * (p1.y - p0.y),
                );

                // q1_1 = b3 + 0.75 (b2 - b3)
                let q1_1 = Point::new(
                    p3.x + 0.75 * (p2.x - p3.x),
                    p3.y + 0.75 * (p2.y - p3.y),
                );

                // q0_2 = q1_0 = 0.5 * (q0_1 + q1_1)
                let q0_2 = Point::new(
                    0.5 * (q0_1.x + q1_1.x),
                    0.5 * (q0_1.y + q1_1.y),
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

    let mut p0 = Point::ZERO;
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
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

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

            let p = Point::new(
                p0.x + dx * offset * len_r,
                p0.y + dy * offset * len_r,
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
