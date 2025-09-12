use core::hash;
use std::{ops, sync::Arc};

use essay_graphics_api::{
    input::Input, output::Output, 
    renderer::{self, Canvas, Drawable, Renderer}, 
    Bounds, Length, Padding, Rectangle, Size, TextStyle
};

use crate::{
    style::UiTheme, 
    ui::{
        alloc::AllocPair, widget::DrawWidget, AllocSize, AppState, Context, Layer, Painter, RenderPass, Response, UiRender, Update, View
    }, 
    util::Id, widget2::Text, 
};

use super::alloc::{Alloc, AllocDirection};

pub struct Ui<'a> {
    id: Id,
    stable_id: Id,
    next_auto_id_salt: u64,
    
    alloc: Alloc,

    layer: Layer,
    render: &'a mut UiRender,
    style: Arc<UiTheme>,

    stack: Option<&'a UiStack<'a>>,
}

impl<'a> Ui<'a> {
    pub fn stable_id(&self) -> Id {
        self.stable_id
    }
    
    pub fn next_id(&self) -> Id {
        self.stable_id.with(self.next_auto_id_salt)
    }

    #[inline]
    pub fn theme(&self) -> &UiTheme {
        &self.render.theme
     }

    #[inline]
    pub fn context(&self) -> &Context {
        &self.render.context
    }

    #[inline]
    pub fn pass(&self) -> &RenderPass {
        &self.render.state.pass
    }

    #[inline]
    pub fn pass_mut(&mut self) -> &mut RenderPass {
        &mut self.render.state.pass
    }

    #[inline]
    pub fn last_pass(&self) -> &RenderPass {
        &self.render.state.last_pass
    }

    #[inline]
    pub fn painter<'b>(&'b mut self) -> Painter<'b> {
        Painter::new(self.layer, &mut self.render)
    }

    #[inline]
    pub fn input(&self) -> &Input {
        &self.render.input
    }

    #[inline]
    pub fn output_mut(&mut self) -> &mut Output {
        self.render.output.as_mut().unwrap()
    }
    
    pub(crate) fn render(&self) -> &UiRender {
        self.render
    }

    pub(crate) fn top<R>(
        ctx: &Context,
        render: &mut UiRender,
        id: Id,
        builder: UiBuilder,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            max_bounds,
            ..
        } = builder;

        let bounds = max_bounds.unwrap_or_else(|| ctx.screen_pos());

        let alloc_cache = render.state.last_pass.alloc_map.get(&id).cloned();
        let alloc = Alloc::new(bounds, AllocDirection::Column, alloc_cache.clone());

        let mut ui = Ui {
            stable_id: id,
            id,
            next_auto_id_salt: id.with("auto").value(),
            alloc,

            layer: Layer::Main,
            style: ctx.style(),
            render,

            stack: None,
        };

        let start_rect = Rectangle::ZERO;
        ui.insert_widget(ui.id, start_rect);
    
        let result = (add_content)(&mut ui);

        let new_alloc = AllocPair {
            outer: ui.alloc.alloc_size.clone(),
            inner: ui.alloc.alloc_size.clone(),
        };
    
        let response = ui.end(&alloc_cache, new_alloc);

        ResponseValue::new(result, response)
    }
    
    pub(crate) fn child<R>(
        &mut self,
        builder: impl Into<UiBuilder>,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            id_salt,
            max_bounds,
            view: size,
            margin,
            update: alloc_update,
        } = builder.into();
        
        let id_salt = id_salt.unwrap_or_else(|| Id::from("child"));
        let stable_id = self.stable_id.with(id_salt);
        let child_id = stable_id.with(self.next_auto_id_salt);
        self.next_auto_id_salt = self.next_auto_id_salt.wrapping_add(1);
        let next_auto_id_salt = child_id.value().wrapping_add(1);

        let max_bounds = max_bounds.unwrap_or_else(|| {
            self.alloc.available()
        });

        let update = alloc_update.unwrap_or_else(|| self.alloc.alloc_dir);

        let pos = Rectangle::ZERO;

        self.insert_widget(child_id, pos);

        let alloc_cache = self.last_pass()
            .alloc_map.get(&child_id).cloned();

        let alloc = self.alloc.child(
            max_bounds,
            margin,
            update,
            alloc_cache.clone()
        );

        let mut child = Ui {
            stable_id,
            id: child_id,
            next_auto_id_salt,
            alloc,

            layer: self.layer,
            style: self.style.clone(),
            render: self.render,

            stack: self.stack,
        };

        let result = (add_content)(&mut child);

        let alloc_child = self.alloc.merge_child(&mut child.alloc, size);

        let response = child.end(&alloc_cache, alloc_child);

        ResponseValue::new(result, response)
    }

    fn end(
        &mut self, 
        _old_alloc: &Option<AllocPair>,
        new_alloc: AllocPair,
    ) -> Response {
        let bounds = self.alloc.alloc + self.alloc.margin;

        //if new_alloc.is_changed(&old_alloc) {
        //    println!("AllocChange")
        //}

        let id = self.id;
        self.pass_mut().alloc_map.insert(id, new_alloc);
        self.insert_widget(id, bounds)
    }
    
    pub(crate) fn popup<R>(
        &mut self,
        id: Id,
        builder: impl Into<UiBuilder>,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            id_salt: _id_salt,
            max_bounds,
            view: _size,
            margin,
            update: alloc_update,
        } = builder.into();
        
        let bounds = max_bounds.unwrap_or_else(|| self.context().screen_pos());

        let update = alloc_update.unwrap_or_else(|| self.alloc.alloc_dir);

        let pos = Rectangle::ZERO;

        self.insert_widget(id, pos);

        let alloc_cache = self.last_pass().alloc_map.get(&id).cloned();
        let alloc = Alloc::new(bounds, update, alloc_cache.clone());

        let stack = UiStack {
            parent: self.stack,
            popup_id: id,
        };

        let mut popup_ui = Ui {
            stable_id: id,
            id,
            next_auto_id_salt: id.with("auto").value(),
            alloc,
            layer: Layer::Popup,
            style: self.style.clone(),
            render: self.render,

            stack: Some(&stack),
        };

        let result = (add_content)(&mut popup_ui);

        let bounds = popup_ui.alloc.alloc + margin;

        //let new_alloc = popup_ui.alloc.alloc_size.clone();
        //if new_alloc.is_changed(&alloc_cache) {
        //    println!("AllocChange")
        //}

        let new_alloc = AllocPair {
            outer: popup_ui.alloc.alloc_size.clone(),
            inner: popup_ui.alloc.alloc_size.clone(),
        };

        self.pass_mut().alloc_map.insert(id, new_alloc);
        let response = self.insert_widget(id, bounds);

        ResponseValue::new(result, response)
    }

    pub fn close_popup(&mut self) {
        let mut ui_stack = self.stack;

        while let Some(stack) = ui_stack {
            self.context().memory_mut(|mem| {
                mem.popup_close(stack.popup_id);
            });

            ui_stack = stack.parent;
        }
    }

    pub fn available_bounds(&self) -> Bounds<Canvas> {
        self.alloc.available()
    }

    #[inline]
    pub fn available(&mut self) -> Bounds<Canvas> {
        self.alloc.available()
    }

    #[inline]
    pub fn allocate(&mut self, size: impl Into<Size<Length>>) -> ResponseValue<Bounds<Canvas>> {
        let pos = self.alloc.alloc(size);

        self.add_widget(pos)
    }

    fn add_widget(&mut self, pos: impl Into<Rectangle>) -> ResponseValue<Bounds<Canvas>> {
        let id = self.stable_id.with(self.next_auto_id_salt);
        self.next_auto_id_salt = self.next_auto_id_salt.wrapping_add(1);

        let pos = pos.into();

        let response = self.insert_widget(id, pos);

        ResponseValue::new(pos.into(), response)
    }

    pub(crate) fn insert_widget(&mut self, id: Id, pos: impl Into<Rectangle>) -> Response {
        let layer = self.layer;
        let widget = self.pass_mut().widgets.insert(id, layer, pos);

        Response::new(widget)
    }

    #[inline]
    pub fn draw_widget(&mut self, mut widget: impl DrawWidget) -> Response {
        widget.draw(self)
    }

    #[inline]
    pub fn label(&mut self, label: impl Into<Text>) -> Response {
        self.draw_widget(label.into())
    }

    pub fn draw_size(&mut self, size: impl Into<Size<Length>>, draw: impl Drawable + 'static) -> Response {
        let ResponseValue { response, .. } = self.allocate(size);

        let pos = response.rect(self);

        let mut draw = Box::new(draw);
        
        self.painter().add(move |ui: &mut dyn Renderer| {
            ui.draw_with_clip(pos.into(), Box::new(|ui| {
                draw.draw(ui)
            }))
        });

        response
    }

    pub fn row<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        self.child(UiBuilder::default()
            .update(AllocDirection::Row),
            add_content
        )
    }

    pub fn row_with<R>(
        &mut self, 
        builder: impl Into<UiBuilder>,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        self.child(builder.into()
            .update(AllocDirection::Row),
            add_content
        )
    }

    pub fn column<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        self.child(UiBuilder::default()
            .update(AllocDirection::Column),
            add_content
        )
    }

    pub fn column_with<R>(
        &mut self, 
        builder: impl Into<UiBuilder>,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        self.child(builder.into()
            .update(AllocDirection::Column),
            add_content
        )
    }

    pub fn view<R>(
        &mut self, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let size = Size::new(Length::Fill, Length::Fill);

        self.child(
            UiBuilder::default()
                .size(size)
                .update(AllocDirection::Column),
            add_content
        )
    }

    pub fn app<'b, State, Message>(
        &mut self, 
        state: &'b mut State, 
        view: impl for<'c> View<'c, State, Message>,
        update: impl Update<State, Message>,
    ) -> Response {
        AppState::new(state, update, view).show(self)
    }
    
    pub fn text_size(&mut self, label: &str, style_text: &TextStyle) -> Size {
        self.context().fonts_mut(|fonts| {
            let fonts = &mut fonts.default_font_set;

            let mut width = 0.;
            let mut height = 0.;

            let size = style_text.get_size().unwrap_or(10.);
            let size = 4. / 3. * 2. * size; // ppt

            for ch in label.chars() {
                let rect = fonts.glyph_size(size, ch);

                width += rect.width;
                height = (rect.ascent + rect.descent).max(height);
            }

            Size::new(width, height)
        })
        // let len = label.len();
        // let pt = style_text.get_size().unwrap_or(10.);

        // TODO:
    }
}

pub struct StateBase {}
pub enum MessageBase {}

#[derive(Default)]
pub struct UiBuilder {
    id_salt: Option<Id>,
    max_bounds: Option<Bounds<Canvas>>,
    view: Option<Size<Length>>,
    margin: Padding,
    update: Option<AllocDirection>,
}

impl UiBuilder {
    #[inline]
    pub fn id_salt(mut self, hash: impl hash::Hash) -> Self {
        self.id_salt = Some(Id::new(hash));

        self
    }

    #[inline]
    pub fn max_bounds(mut self, bounds: impl Into<Bounds<Canvas>>) -> Self {
        self.max_bounds = Some(bounds.into());

        self
    }

    #[inline]
    pub fn margin(mut self, margin: impl Into<Padding>) -> Self {
        self.margin = margin.into();

        self
    }

    #[inline]
    pub(crate) fn update(mut self, update: AllocDirection) -> Self {
        self.update = Some(update);

        self
    }

    #[inline]
    pub fn view(mut self, size: impl Into<Size>) -> Self {
        let size = size.into();

        self.view = Some(Size::new(
            Length::View(size.width),
            Length::View(size.height),
        ));

        self
    }

    #[inline]
    pub fn size(mut self, size: impl Into<Size<Length>>) -> Self {
        self.view = Some(size.into());

        self
    }
}

impl From<Padding> for UiBuilder {
    fn from(size: Padding) -> Self {
        UiBuilder::default().margin(size)
    }
}

impl From<Size<Length>> for UiBuilder {
    fn from(size: Size<Length>) -> Self {
        UiBuilder::default().size(size)
    }
}

impl From<Length> for UiBuilder {
    fn from(length: Length) -> Self {
        UiBuilder::default().size(Size::from(length))
    }
}

pub(crate) struct UiStack<'a> {
    parent: Option<&'a UiStack<'a>>,

    popup_id: Id,
}
#[derive(Copy, Clone, Debug)]
pub enum UiSize {
    Canvas(f32, f32),
    View(f32, f32),
}

pub struct OnceView<T: Drawable> {
    view: Option<T>,
}

impl<T: Drawable> OnceView<T> {
    #[inline]
    pub fn get(&self) -> Option<&T> {
        self.view.as_ref()
    } 

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.view.as_mut()
    } 

    #[inline]
    pub fn get_or_init(&mut self, draw: T) -> &T {
        if self.view.is_none() {
            self.view = Some(draw);
        }

        self.view.as_ref().unwrap()
    } 

    #[inline]
    pub fn get_or_init_mut(&mut self, draw: T) -> &mut T {
        if self.view.is_none() {
            self.view = Some(draw);
        }

        self.view.as_mut().unwrap()
    } 
}

impl<T: Drawable> Drawable for OnceView<T> {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        if let Some(view) = &mut self.view {
            view.draw(ui)
        } else {
            Ok(())
        }
    }
}

pub struct ResponseValue<T> {
    pub value: T,
    pub response: Response,
}

impl<T> ResponseValue<T> {
    pub(crate) fn new(value: T, response: Response) -> Self {
        Self {
            value,
            response
        }
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut Response {
        &mut self.response
    }
}

impl<T> ops::Deref for ResponseValue<T> {
    type Target = Response;

    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

impl<T> ops::DerefMut for ResponseValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.response
    }
}
