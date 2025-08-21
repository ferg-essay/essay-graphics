use essay_graphics_api::{Point};

use crate::{context::Context, ui::{ui::{ResponseValue, Ui, UiBuilder}, Frame, Response}, util::Id};

pub struct Popup {
    id: Id,
    ctx: Context,
    pos: Point,

    open: Open,
}

impl Popup {
    pub fn new(id: Id, ctx: &Context, pos: impl Into<Point>) -> Self {
        Self {
            id,
            ctx: ctx.clone(),
            pos: pos.into(),
            open: Open::Open,
        }
    }

    pub fn from_response(response: &Response) -> Self {
        let widget = response.context().pass(|pass| {
            *pass.widgets().get(response.id).unwrap()
        });

        Self::new(
            response.id().with("popup"),
            response.context(),
            [widget.rect.x0(), widget.rect.ymax()],
        )
    }

    pub fn menu(response: &Response) -> Self {
        Self::from_response(response)
            .open_memory(response.clicked().then_some(OpenMemory::Toggle))
    }

    pub fn open(mut self, is_enabled: bool) -> Self {
        self.open = Open::Bool(is_enabled);
        self
    }

    pub fn open_memory(mut self, open: Option<OpenMemory>) -> Self {
        self.open = Open::Memory(open);
        self
    }

    pub fn show<R>(self, add_content: impl FnOnce(&mut Ui) -> R) -> Option<ResponseValue<R>> {
        self.update_open();

        if ! self.is_open() {
            return None;
        }

        let builder = UiBuilder::default()
            .max_bounds(([self.pos.x(), self.pos.y()], [400., 100.]));

        let frame = Frame::group();
        
        Some(Ui::top(&self.ctx, self.id, builder, |ui| {
            let ResponseValue {
                value,
                ..
            } = frame.show(ui, add_content);

            value
        }))
    }

    fn update_open(&self) {
        if let Open::Memory(memory) = &self.open {
            match memory {
                Some(OpenMemory::Toggle) => {
                    self.ctx.memory_mut(|memory| {
                        memory.popup_toggle(self.id)
                    });
                },
                None =>{
                }
            }
        }
    }

    fn is_open(&self) -> bool {
        match self.open {
            Open::Open => true,
            Open::Close => false,
            Open::Bool(is_open) => is_open,
            Open::Memory(_) => self.ctx.memory(|memory| {
                memory.popup_open(self.id)
            })
        }
    }
}

enum Open {
    Open,
    Close,
    Bool(bool),
    Memory(Option<OpenMemory>)
}

impl Open {
    fn is_open(&self, ctx: &Context, id: Id) -> bool {
        match self {
            Open::Open => true,
            Open::Close => false,
            Open::Bool(is_open) => *is_open,
            Open::Memory(_) => ctx.memory(|memory| {
                memory.popup_open(id)
            })
        }
    }
}

pub enum OpenMemory {
    Toggle
}