use essay_graphics_api::{renderer::Renderer, Color, Margin, Path, PathStyle};

use crate::ui::ui::{ResponseValue, Ui, UiBuilder};

pub struct Frame {
    pub inner_margin: Margin,
    pub outer_margin: Margin,
}

impl Frame {
    pub fn group() -> Self {
        Self {
            inner_margin: Margin::from_all(6.),
            outer_margin: Margin::from_all(0.),
        }
    }

    #[inline]
    pub fn total_margin(&self) -> Margin {
        self.inner_margin + self.outer_margin
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let max_bounds = ui.available_bounds() - self.total_margin();

        // todo: negative bounds
        assert!(max_bounds.x0() < max_bounds.x1());
        assert!(max_bounds.y0() < max_bounds.y1());

        let builder = UiBuilder::default()
            .max_bounds(max_bounds);

        let ResponseValue {
            value,
            response
        } = ui.child(builder, add_contents);

        let mut rect = ui.context().pass(|pass| {
            *pass.widgets().get(response.id()).unwrap()
        });

        rect.rect = rect.rect + self.total_margin();

        let ResponseValue {
            response,
            ..
        } = ui.alloc_response(rect.rect);

        let pos = rect.rect;

        ui.painter_mut().add(move |ui: &mut dyn Renderer| {
            let mut style = PathStyle::new();
            style.face_color(Color(0x0));
            style.edge_color(0x808080);
            
            ui.draw_path(&Path::rect(pos), &style)
        });

        ResponseValue::new(value, response)
    }
}