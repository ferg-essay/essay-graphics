use essay_graphics_api::{renderer::Renderer, Color, Padding, Point, Rectangle, Shapes};

use crate::{style::{Style, UiStyle}, ui::{painter::PaintIndex, ui::{ResponseValue, Ui}}};

#[derive(Clone)]
pub struct Frame {
    pub style: Style,
    pub inner_margin: Option<Padding>,
    pub outer_margin: Option<Padding>,

    pub background: Option<Color>,
    pub is_shadow: bool,
}

impl Frame {
    pub fn new(style: impl Into<Style>) -> Self {
        Self {
            style: style.into(),
            inner_margin: None,
            outer_margin: None,
            background: None,
            is_shadow: false,
        }
    }

    pub fn group() -> Self {
        Self {
            style: Style::Group,
            inner_margin: None,
            outer_margin: None,
            background: None,
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
        self.background = Some(background.into());

        self
    }

    #[inline]
    pub(crate) fn total_margin(&self, ui: &mut Ui) -> Padding {
        let outer_margin = self.outer_margin.unwrap_or_else(|| {
            self.style.margin(ui)
        });
        
        let border = self.style.border_width(ui);
        let corner = self.style.corner_radius(ui);

        let inner_margin = self.inner_margin.unwrap_or_else(|| {
            self.style.padding(ui)
        });

        inner_margin + border + corner + outer_margin
    }
    
    pub(crate) fn padding(&self, ui: &mut Ui) -> Padding {
        let inner_margin = self.inner_margin.unwrap_or_else(|| {
            self.style.padding(ui)
        });

        let corner = self.style.corner_radius(ui);
        let border = self.style.border_width(ui);

        inner_margin + corner + border
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> ResponseValue<R> {
        return ui.child(self, add_contents);
    }

    pub fn draw(self, ui: &mut Ui, pos: Rectangle, index: PaintIndex) {
        let background = self.background.unwrap_or_else(|| {
            self.style.background(ui)
        });

        let corner = self.style.corner_radius(ui);
        let border = self.style.border(ui);
        let border_width = self.style.border_width(ui);
        let is_shadow = self.is_shadow;
        let shadow = ui.theme().shadow;

        ui.painter().set(index, move |ui: &mut dyn Renderer| {
            let pos = pos.snap();

            if is_shadow {
                // cheap shadow
                let px = 5.;

                ui.draw_shape(&Shapes::rect(
                    pos.p0() + Point::new(px, px), pos.size(), corner, shadow,
                ))?;
            }

            ui.draw_shape(&Shapes::quad(
                pos.p0(), 
                pos.size(), 
                corner + border_width, 
                border,
                corner,
                background,
            ))
        });
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self { 
            style: Style::default(),
            inner_margin: None,
            outer_margin: None,
            background: None,
            is_shadow: Default::default() 
        }
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

            Frame::group().background(0x00ff00).show(ui, |ui| {
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

            Frame::group().background(0x00ff00).show(ui, |ui| {
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
