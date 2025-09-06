use essay_graphics_api::{input::Input, renderer::Pos, Rectangle};

use crate::{ui::{Context, Ui, WidgetPos}, util::Id, windows::Tooltip};

pub struct Response {
    pub id: Id,
    pub is_hover: bool,

    #[doc(hidden)]
    pub flags: Flags,

    // render: &'a mut Render,
}

impl Response {
    pub(crate) fn new(ui: &mut Ui, widget: WidgetPos) -> Self {
        let mut response = Self {
            id: widget.id,
            is_hover: false,
            flags: Flags::empty(),

            // render: ui.render_mut(),
        };

        ui.context().viewport(|viewport| {
            let id = widget.id;

            if viewport.hover.contains(id) {
                response.is_hover = true;
                response.flags.set(Flags::HOVERED, true);
            }

            if viewport.interact.clicked == Some(id) {
                response.flags.set(Flags::CLICKED, true);
            }
        });

        response
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn context(&self) -> &Context {
        // &self.ctx
        todo!();
    }

    pub fn input<R>(&self, reader: impl FnOnce(&Input) -> R) -> R {
        // self.ctx.input(reader)
        todo!();
    }

    #[inline]
    pub fn is_hover(&self) -> bool {
        self.is_hover
    }

    pub(crate) fn rect(&self, ui: &Ui) -> Rectangle {
        // self.ui.pass().widgets.get(self.id).unwrap().rect
        ui.pass().widgets.get(self.id).unwrap().pos
    }

    pub fn on_hover_ui(&self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) -> &Self {
        if self.is_hover() {
            Tooltip::for_enabled(ui, &self).show(ui, add_contents);
        }

        &self
    }
    
    #[inline(always)]
    pub fn clicked(&self) -> bool {
        self.flags.contains(Flags::CLICKED)
    }
    
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
pub struct Flags(u16);

bitflags::bitflags! {
    impl Flags: u16 {
        const ENABLED = 1<<0;
        const HOVERED = 1<<2;
        const CLICKED = 1<<4;
    }
}