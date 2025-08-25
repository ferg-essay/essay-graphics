use essay_graphics_api::{input::Input, Rectangle};

use crate::widget2::Shell;

pub trait Widget<Message, Theme, Renderer> {
    #[allow(unused_variables)]
    fn update(
        &mut self,
        input: &Input,
        shell: &mut Shell<'_, Message>,
    ) {
    }

    #[allow(unused_variables)]
    fn draw(
        &mut self,
        renderer: &Renderer,
        bounds: &Rectangle
    ) {
    }
}