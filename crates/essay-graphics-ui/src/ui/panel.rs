use crate::{context::Context, ui::ui::{ResponseValue, Ui, UiBuilder}, util::Id};

#[must_use="CentralPanel requires .show() call"]
#[derive(Default)]
pub struct CentralPanel {

}

impl CentralPanel {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn show<'a, R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> R
    ) -> ResponseValue<R> {
        let id = Id::new("center");

        let builder = UiBuilder::default();
            
        Ui::top(ctx, id, builder, |ui2| {
            (add_contents)(ui2)
        })
    }
}