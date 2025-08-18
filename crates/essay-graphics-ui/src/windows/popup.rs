use essay_graphics_api::{Point};

use crate::{context::Context, ui::{ui::{ResponseValue, Ui, UiBuilder}, Frame, Response}, util::Id};

pub struct Popup {
    id: Id,
    ctx: Context,
    pos: Point,

    is_enabled: bool,
}

impl Popup {
    pub fn new(id: Id, ctx: &Context, pos: impl Into<Point>) -> Self {
        Self {
            id,
            ctx: ctx.clone(),
            pos: pos.into(),
            is_enabled: true,
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

    pub fn open(mut self, is_enabled: bool) -> Self {
        self.is_enabled = is_enabled;
        self
    }

    pub fn show<R>(self, add_content: impl FnOnce(&mut Ui) -> R) -> Option<ResponseValue<R>> {
        if ! self.is_enabled {
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
}