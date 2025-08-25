mod renderer;
mod theme;
mod task;
mod application;
pub mod element;
mod button;
mod shell;
mod widget;

pub use application::{application, View, Update};
pub use button::Button;
pub use shell::Shell;
pub use task::Task;
pub use theme::Theme;
pub use widget::Widget;

pub type Element<
    'a, 
    Message, 
    Theme = theme::Theme,
    Renderer = renderer::Renderer,
> = element::Element<'a, Message, Theme, Renderer>;
