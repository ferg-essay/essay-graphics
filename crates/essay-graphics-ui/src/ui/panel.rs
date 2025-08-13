use essay_graphics_api::renderer::Renderer;

use crate::ui::{ui2::Ui2, Context, Id, Ui};

#[must_use="CentralPanel requires .show() call"]
#[derive(Default)]
pub struct CentralPanel {

}

impl CentralPanel {
    pub fn show<'a, R>(
        self,
        ui: &mut Ui2,
        add_contents: impl FnOnce(&mut Ui2) -> R
    ) -> R {
        self.show_dyn(ui, Box::new(add_contents))
    }

    pub fn show_dyn<'a, R>(
        self,
        ui: &mut Ui2,
        add_contents: Box<dyn FnOnce(&mut Ui2) -> R + 'a>
    ) -> R {
        //let id = ui.id().with("central_panel");

        ui.vertical(add_contents)
    }
}