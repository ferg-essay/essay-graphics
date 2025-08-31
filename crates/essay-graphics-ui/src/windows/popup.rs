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

    pub fn from_response(ui: &mut Ui, response: &Response) -> Self {
        let widget = ui.pass().widgets().get(response.id).unwrap();

        Self::new(
            response.id().with("popup"),
            ui.context(),
            [widget.rect.x0(), widget.rect.ymax()],
        )
    }

    pub fn menu(ui: &mut Ui, response: &Response) -> Self {
        Self::from_response(ui, response)
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
            .max_bounds(([self.pos.x, self.pos.y], [400., 100.]));

            /*
        Some(Ui::top(&self.ctx, self.id, builder, |ui| {
            let frame = Frame::group(ui).shadow(true);
        
            let ResponseValue {
                value,
                response,
            } = frame.show(ui, add_content);

            value
        }))
        */
        println!("TODO Popup");

        None
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

#[cfg(test)]
mod test {
    use essay_graphics_test::{TestGraphicsContext, TestRenderer};

    use crate::{context::Context, windows::Popup};

    #[test]
    fn popup() {
        let mut test = TestRenderer::new([1000., 1000.]);
        let ctx = Context::new(Box::new(TestGraphicsContext::new()));

        ctx.run(&mut test, |ui| {
            let response = ui.label("Test");

            Popup::from_response(ui, &response).open(true).show(|ui| {
                ui.label("Popup");
            });
        }).unwrap();

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'
rect (5.0,32.0) 166.0x58.0 #00000020
rect (0.0,27.0) 166.0x58.0 #ffffffff
text (16.0,42.7) 'Popup'");

        if true { return; }

        println!("\n  Pass2");

        ctx.run(&mut test, |ui| {
            let response = ui.label("Test");

            Popup::from_response(ui, &response).open(true).show(|ui| {
                ui.label("Popup");
            });
        });

        assert_eq!(test.take(), "text (0.0,0.0) 'Test'
rect (5.0,32.0) 166.0x58.0 #00000020
rect (0.0,27.0) 166.0x58.0 #ffffffff
text (16.0,42.7) 'Popup'");
    }
}
