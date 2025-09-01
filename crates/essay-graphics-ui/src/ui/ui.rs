use core::hash;
use std::{marker::PhantomData, ops, sync::Arc};

use essay_graphics_api::{
    input::Input, output::Output, renderer::{self, Canvas, Drawable, Renderer}, Bounds, Margin, Size, TextStyle
};

use crate::{
    painter::Painter, style::UiStyle, ui::{Context, RenderPass, Response, UiRender, WidgetRect}, util::Id, widget2::{AppState, Update, View}, widgets::{Button, Label, Radio, SelectableLabel}, windows::MenuButton
};

use super::alloc::{Alloc, AllocDirection};

pub struct Ui<'a, Message=MessageBase> {
    id: Id,
    unique_id: Id,
    next_auto_id_salt: u64,
    
    alloc: Alloc,

    render: &'a mut UiRender,
    painter: Painter,
    style: Arc<UiStyle>,

    cache_index: usize,

    marker: PhantomData<Message>,
}

impl<'a> Ui<'a> {
    #[inline]
    pub fn style(&self) -> &UiStyle {
        &self.render.theme
     }

    #[inline]
    pub fn context(&self) -> &Context {
        &self.render.context
    }

    #[inline]
    pub fn pass(&self) -> &RenderPass {
        &self.render.pass
    }

    #[inline]
    pub fn pass_mut(&mut self) -> &mut RenderPass {
        &mut self.render.pass
    }

    #[inline]
    pub fn last_pass(&self) -> &RenderPass {
        &self.render.last_pass
    }

    #[inline]
    pub fn painter_mut(&mut self) -> &mut Painter {
        &mut self.painter
    }

    pub(crate) fn top<R>(
        ctx: &Context,
        render: &mut UiRender,
        id: Id,
        builder: UiBuilder,
        add_content: impl FnOnce(&mut Ui) -> R
    ) {
        let UiBuilder {
            max_bounds,
            ..
        } = builder;

        let bounds = max_bounds.unwrap_or_else(|| ctx.screen_pos());

        let alloc_cache = render.last_pass.alloc_map.get(&id).cloned();

        let alloc = Alloc::new(bounds, AllocDirection::Vertical, alloc_cache.clone());

        let mut ui = Ui {
            id,
            unique_id: id,
            next_auto_id_salt: id.with("auto").value(),
            alloc,
            painter: Painter::new(&ctx),
            style: ctx.style(),
            render,
    
            cache_index: 0,
            marker: Default::default(),
        };

        let start_rect = Bounds::none();
        ui.create_widget(WidgetRect {
            id: ui.unique_id,
            rect: start_rect,
        });
    
        let result = (add_content)(&mut ui);
    
        let response = ui.child_end();

        let new_alloc = ui.alloc.to_cache();
        if new_alloc.is_changed(&alloc_cache) {
            println!("AllocChangeTop")
        }

        render.pass.alloc_map.insert(id, new_alloc);

        //ctx.replace_render(render);

        // ResponseValue::new(result, self.create_response(response))
    }
    
    pub(crate) fn child<R>(
        &mut self,
        builder: UiBuilder,
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let UiBuilder {
            id_salt,
            max_bounds,
            view,
            margin,
            update: alloc_update,
        } = builder;
        
        let id_salt = id_salt.unwrap_or_else(|| Id::from("child"));
        let stable_id = self.id.with(id_salt);
        let unique_id = stable_id.with(self.next_auto_id_salt);
        self.next_auto_id_salt = self.next_auto_id_salt.wrapping_add(1);
        let next_auto_id_salt = unique_id.value().wrapping_add(1);

        let max_bounds = max_bounds.unwrap_or_else(|| {
            self.alloc.available()
        });

        let update = alloc_update.unwrap_or_else(|| self.alloc.alloc_dir);

        let bounds = Bounds::none();

        self.create_widget(WidgetRect {
            id: unique_id,
            rect: bounds,
        });

        let alloc_cache = self.last_pass()
            .alloc_map.get(&unique_id).cloned();

        let alloc = self.alloc.child(
            max_bounds,
            view,
            margin,
            update,
            alloc_cache.clone()
        );

        let mut child = Ui {
            id: stable_id,
            unique_id,
            next_auto_id_salt,
            alloc,
            painter: Painter::new(self.painter.context()),
            style: self.style.clone(),
            render: self.render,

            cache_index: self.cache_index,
            marker: Default::default(),
        };

        let result = (add_content)(&mut child);

        self.alloc.merge_child(&child.alloc);

        let child_widget = child.child_end();

        let new_alloc = child.alloc.to_cache();
        if new_alloc.is_changed(&alloc_cache) {
            println!("AllocChange")
        }

        self.pass_mut().alloc_map.insert(unique_id, new_alloc);

        let response = Response::new(self, child_widget);

        ResponseValue::new(result, response)
    }

    fn child_end(&mut self) -> WidgetRect {
        let bounds = self.alloc.alloc + self.alloc.margin;
        WidgetRect {
            id: self.unique_id,
            rect: bounds,
        }
    }

    pub(crate) fn create_widget(&mut self, widget: WidgetRect) -> Response {
        self.pass_mut().widgets.insert(widget);

        Response::new(self, widget)
    }

    pub fn available_bounds(&self) -> Bounds<Canvas> {
        self.alloc.available()
    }

    #[inline]
    pub fn allocate_rect(&mut self, size: Size) -> ResponseValue<Bounds<Canvas>> {
        let pos = self.alloc.alloc_canvas(size);

        self.alloc_response(pos)
    }
    
    #[inline]
    pub fn allocate_view(&mut self, size: Size) -> ResponseValue<Bounds<Canvas>> {
        let pos = self.alloc.alloc_view(size);

        self.alloc_response(pos)
    }

    pub fn alloc_response(&mut self, pos: Bounds<Canvas>) -> ResponseValue<Bounds<Canvas>> {
        let id = self.id.with(self.next_auto_id_salt);
        self.next_auto_id_salt = self.next_auto_id_salt.wrapping_add(1);

        let widget = WidgetRect {
            id,
            rect: pos,
        };

        let response = self.create_widget(widget);

        ResponseValue::new(pos, response)
    }

    #[inline]
    pub fn remaining_size(&mut self) -> Size {
        self.alloc.canvas_free()
    }

    #[inline]
    pub fn add(&mut self, widget: impl Widget) -> Response {
        widget.ui(self)
    }

    #[inline]
    pub fn label(&mut self, label: &str) -> Response {
        let label = Label::new(label);

        self.add(label)
    }

    #[inline]
    pub fn button(&mut self, label: &str, press: bool) -> Response {
        let button = Button::new(label, press);

        self.add(button)
    }

    #[inline]
    pub fn menu_button<R>(
        &mut self, 
        label: &str, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<Option<R>> {
        let (button, _popup) = {
            MenuButton::new(label).ui(self, add_content)
        };

        ResponseValue::new(None, button)
    }

    #[must_use="Check for input with ui.selectable_label(...).clicked()"]
    pub fn selectable_label(&mut self, is_checked: bool, text: &str) -> Response {
        SelectableLabel::new(text, is_checked).ui(self)
    }

    pub fn selectable_value<V: PartialEq>(
        &mut self, 
        var: &mut V,
        value: V,
        text: &str, 
    ) -> Response {
        let response = SelectableLabel::new(text, *var == value).ui(self);

        if response.clicked() && *var != value {
            *var = value;
            // response.mark_changed();
        }

        response
    }

    #[must_use="Check for input with ui.radio(...).clicked()"]
    pub fn radio(&mut self, is_checked: bool, text: &str) -> Response {
        Radio::new(text, is_checked).ui(self)
    }

    pub fn radio_value<V: PartialEq>(
        &mut self, 
        var: &mut V,
        value: V,
        text: &str, 
    ) -> Response {
        let response = Radio::new(text, *var == value).ui(self);

        if response.clicked() && *var != value {
            *var = value;
            // response.mark_changed();
        }

        response
    }

    pub fn draw(&mut self, draw: impl Drawable + 'static) -> Response {
        self.draw_size(Size::UNIT, draw)
    }

    pub fn draw_size(&mut self, size: Size, draw: impl Drawable + 'static) -> Response {
        let ResponseValue { response, .. } = self.allocate_view(size);
        
        self.painter_mut().add(draw);

        response
    }

    pub fn row<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let result = self.child(UiBuilder::default()
            .update(AllocDirection::Horizontal),
            add_content
        );

        result
    }

    pub fn column<R>(&mut self, add_content: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let result = self.child(UiBuilder::default()
            .update(AllocDirection::Vertical),
            add_content
        );

        result
    }

    pub fn view<R>(
        &mut self, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let pos = self.alloc.alloc_view(Size::UNIT);

        let result = self.child(
            UiBuilder::default()
                .max_bounds(pos)
                .view(Size::UNIT)
                .update(AllocDirection::Vertical),
            add_content
        );

        // self.update_pos();

        result
    }

    pub fn horizontal_size<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let mut builder = UiBuilder::default();

        let bounds = match size {
            UiSize::Canvas(width, height) => {
                self.alloc.alloc_canvas([width, height])
            }
            UiSize::View(width, height) => {
                builder = builder.view([width, height]);

                self.alloc.alloc_view([width, height])
            }
        };

        let result = self.child(builder
            .max_bounds(bounds)
            .update(AllocDirection::Horizontal),
            add_content
        );

        // self.update_pos();

        result
    }

    pub fn vertical_size<R>(
        &mut self, 
        size: UiSize, 
        add_content: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let mut builder = UiBuilder::default();

        let bounds = match size {
            UiSize::Canvas(width, height) => {
                self.alloc.alloc_canvas([width, height])
            }
            UiSize::View(width, height) => {
                builder = builder.view([width, height]);
                self.alloc.alloc_view([width, height])
            }
        };

        let result = self.child(builder
            .max_bounds(bounds)
            .update(AllocDirection::Vertical),
            add_content
        );

        // self.update_pos();

        result
    }

    pub fn app<'b, State, Message>(
        &mut self, 
        state: &'b mut State, 
        update: impl Update<State, Message>,
        view: impl for<'c> View<'c, State, Message>,
    ) -> Response {
        self.add(AppState::new(state, update, view))
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

    #[inline]
    pub fn input<R>(&self, reader: impl FnOnce(&Input) -> R) -> R {
        self.context().input(reader)
    }

    #[inline]
    pub fn output_mut(&mut self) -> &mut Output {
        self.pass_mut().output.as_mut().unwrap()
    }
    
    pub fn id(&self) -> Id {
        self.id
    }
    
    pub fn next_id(&self) -> Id {
        self.id.with(self.next_auto_id_salt)
    }
    
    pub(crate) fn render_mut(&'a mut self) -> &'a mut UiRender {
        self.render
    }
}

pub struct StateBase {}
pub enum MessageBase {}

pub struct Shell<'a, M> {
    messages: &'a Vec<M>,
}

impl<'a, M> Shell<'a, M> {
    pub fn new(vec: &'a Vec<M>) -> Self {
        Self {
            messages: vec,
        }
    }
}
#[derive(Default)]
pub(crate) struct UiBuilder {
    id_salt: Option<Id>,
    max_bounds: Option<Bounds<Canvas>>,
    view: Option<Size>,
    margin: Margin,
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
    pub fn margin(mut self, margin: impl Into<Margin>) -> Self {
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
        self.view = Some(size.into());

        self
    }
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


pub trait Widget {
    fn ui(
        self, 
        ui: &mut Ui,
    ) -> Response;
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
