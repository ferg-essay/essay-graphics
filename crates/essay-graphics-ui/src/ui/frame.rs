use essay_graphics_api::{renderer::Renderer, Color, Padding, Point, Shapes};

use crate::{ui::ui::{ResponseValue, Ui, UiBuilder}};

pub struct Frame {
    pub inner_margin: Padding,
    pub outer_margin: Padding,

    pub background: Color,
    pub is_shadow: bool,
}

impl Frame {
    pub fn group(ui: &Ui) -> Self {

        Self {
            inner_margin: Padding::from_all(6.),
            outer_margin: Padding::from_all(0.),
            background: ui.style().background,
            is_shadow: false,
        }
    }

    #[inline]
    pub fn shadow(mut self, is_shadow: bool) -> Self {
        self.is_shadow = is_shadow;

        self
    }

    #[inline]
    pub fn background(mut self, background: impl Into<Color>) -> Self {
        self.background = background.into();

        self
    }

    #[inline]
    pub fn total_margin(&self) -> Padding {
        self.inner_margin + self.outer_margin
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        let corner_margin = Padding::from_all(ui.style().corner_radius);

        let index = ui.painter_mut().add(Shapes::None);

        let margin = self.total_margin() + corner_margin;

        let builder = UiBuilder::default()
            .margin(margin);

        let ResponseValue {
            value,
            response
        } = ui.child(builder, add_contents);

        let rect = ui.pass_mut().widgets().get(response.id()).unwrap();

        let pos = rect.pos; //  + self.inner_margin + corner_margin;

        let background = self.background;
        let corner = ui.style().corner_radius;
        let border = ui.style().border;
        let is_shadow = self.is_shadow;
        let shadow = ui.style().shadow;

        ui.painter_mut().set(index, move |ui: &mut dyn Renderer| {
            let pos = pos.snap();

            if is_shadow {
                // cheap shadow
                let px = 5.;

                ui.draw_shape(&Shapes::Rectangle(
                    pos.p0() + Point::new(px, px), pos.size(), corner, shadow,
                ))?;
            }

            ui.draw_shape(&Shapes::Rectangle(
                pos.p0(), pos.size(), corner, background,
            ))
        });

        ResponseValue::new(value, response)
    }
}

#[cfg(test)]
mod test {
    use essay_graphics_test::{TestGraphicsContext, TestRenderer};

    use crate::ui::{Context, Frame};

    #[test]
    fn frame() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            ui.label("Ante");

            Frame::group(&ui).background(0x00ff00).show(ui, |ui| {
                ui.label("Frame");
            });

            // ui.frame(Frame::group().background(0x00ff00), |ui| {
            //   ui.label("Frame");
            // })
            //
            //

            ui.label("Post");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Ante'
rect (0.0,27.0) 166.0x59.0 #00ff00ff
text (16.0,42.7) 'Frame'
text (0.0,85.3) 'Post'");

        ctx.run(&mut test, |ui| {
            ui.label("Ante");

            Frame::group(&ui).background(0x00ff00).show(ui, |ui| {
                ui.label("Frame");
            });

            ui.label("Post");
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Ante'
rect (0.0,27.0) 166.0x59.0 #00ff00ff
text (16.0,42.7) 'Frame'
text (0.0,85.3) 'Post'");
    }
}
