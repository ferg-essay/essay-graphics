use essay_graphics_api::input::Input;

use crate::{context::{Context, WidgetRect}, ui::Ui, util::Id, windows::Tooltip};

pub struct Response {
    pub id: Id,
    pub ctx: Context,
    pub is_hover: bool,

    #[doc(hidden)]
    pub flags: Flags,
}

impl Response {
    pub(crate) fn new(ctx: &Context, widget: WidgetRect) -> Self {
        let mut response = Self {
            id: widget.id,
            ctx: ctx.clone(),
            is_hover: false,
            flags: Flags::empty(),
        };

        ctx.viewport(|viewport| {
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
        &self.ctx
    }

    pub fn input<R>(&self, reader: impl FnOnce(&Input) -> R) -> R {
        self.ctx.input(reader)
    }

    #[inline]
    pub fn is_hover(&self) -> bool {
        self.is_hover
    }

    pub fn on_hover_ui(&self, add_contents: impl FnOnce(&mut Ui)) -> &Self {
        if self.is_hover() {
            Tooltip::for_enabled(&self).show(add_contents);
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