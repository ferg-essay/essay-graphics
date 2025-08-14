use crate::ui::{ui2::{ResponseValue, Ui2, UiBuilder}, Context, Id};

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
        add_contents: impl FnOnce(&mut Ui2) -> R
    ) -> ResponseValue<R> {
        let id = Id::new("center");

        let builder = UiBuilder::default();
            
        Ui2::top(ctx, id, builder, |ui2| {
            (add_contents)(ui2)
        })
    }
}