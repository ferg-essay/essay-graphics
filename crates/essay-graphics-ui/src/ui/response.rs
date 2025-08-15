use essay_graphics_api::input::Input;

use crate::ui::{tooltip::Tooltip, Context, Id, Ui};

pub struct Response {
    pub id: Id,
    pub ctx: Context,
    pub is_hover: bool,

    #[doc(hidden)]
    pub flags: Flags,
}

impl Response {
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