use essay_graphics_api::{Rectangle};

use crate::{ui::{Ui, WidgetPos}, util::Id, windows::Tooltip};

pub struct Response {
    pub id: Id,
}

impl Response {
    pub(crate) fn new(widget: WidgetPos) -> Self {
        let response = Self {
            id: widget.id,
        };

        response
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn is_hover(&self, ui: &mut Ui) -> bool {
        ui.context().viewport(|viewport| {
            viewport.hover.contains(self.id)
        })
    }

    pub(crate) fn rect(&self, ui: &Ui) -> Rectangle {
        ui.pass().widgets.get(self.id).unwrap().pos
    }

    pub fn on_hover_ui(&self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) -> &Self {
        if self.is_hover(ui) {
            Tooltip::for_enabled(ui, &self).show(ui, add_contents);
        }

        &self
    }
    
    #[inline]
    pub fn clicked(&self, ui: &mut Ui) -> bool {
        ui.context().viewport(|viewport| {
            viewport.interact.clicked == Some(self.id)
        })
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