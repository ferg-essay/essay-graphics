use essay_graphics_api::{Point, Size};

use crate::ui::{context::Response, ui2::{ResponseValue, Ui2, UiBuilder}, Context, Id};

pub struct Popup {
    id: Id,
    ctx: Context,
    pos: Point,
}

impl Popup {
    pub fn new(id: Id, ctx: &Context, pos: impl Into<Point>) -> Self {
        Self {
            id,
            ctx: ctx.clone(),
            pos: pos.into(),
        }
    }

    pub fn from_response(response: &Response) -> Self {
        let widget = response.context().pass(|pass| {
            *pass.widgets().get(response.id).unwrap()
        });

        Self::new(
            response.id().with("popup"),
            response.context(),
            [widget.rect.x0(), widget.rect.ymin()],
        )
    }

    pub fn show<R>(&self, add_content: impl FnOnce(&mut Ui2) -> R) -> ResponseValue<R> {
        let builder = UiBuilder::default()
            .max_bounds(([self.pos.x(), self.pos.y() - 100.], [400., 100.]));

        Ui2::top(&self.ctx, self.id, builder, add_content)
    }
}