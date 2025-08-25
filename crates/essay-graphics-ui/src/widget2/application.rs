use std::marker::PhantomData;

use crate::{context, ui::ui, widget2::{element::Element, Shell, Task}};

pub fn application<State, Message, Theme, Renderer>(
    update: impl Update<State, Message>,
    view: impl for<'a> View<'a, State, Message, Theme, Renderer>,
) {
}

pub trait Update<State, Message> {
    fn update(&self, state: &mut State, message: Message) -> Task<Message>;
}

impl<T> Into<Task<T>> for () {
    fn into(self) -> Task<T> {
        Task::none()
    }
}
impl<T, State, Message, C> Update<State, Message> for T
where
    T: Fn(&mut State, Message) -> C,
    C: Into<Task<Message>>,
{
    fn update(&self, state: &mut State, message: Message) -> Task<Message> {
        self(state, message).into()
    }
}

pub trait View<'a, State, Message, Theme, Renderer> {
    fn view(&self, state: &'a State) -> Element<'a, Message, Theme, Renderer>;
}

impl<'a, T, State, Message, Theme, Renderer, Widget>
    View<'a, State, Message, Theme, Renderer> for T
where
    T: Fn(&'a State) -> Widget,
    State: 'static,
    Widget: Into<Element<'a, Message, Theme, Renderer>>,
{
    fn view(&self, state: &'a State) -> Element<'a, Message, Theme, Renderer> {
        self(state).into()
    }
}

pub struct AppState<'a, State, Message, Theme, Renderer, View>
where
    View: for<'b> self::View<'b, State, Message, Theme, Renderer>
{
    state: &'a mut State,
    update: Box<dyn Update<State, Message> + 'a>,
    view: Box<View>,
    _theme: PhantomData<Theme>,
    _renderer: PhantomData<Renderer>,
}

impl<'a, State, Message, Theme, Renderer, View>
    AppState<'a, State, Message, Theme, Renderer, View>
where
    View: for<'b> self::View<'b, State, Message, Theme, Renderer>
{
    pub fn new(
        state: &'a mut State, 
        update: impl Update<State, Message> + 'a,
        view: View,
    ) -> Self
    {
        Self {
            state,
            update: Box::new(update),
            view: Box::new(view),
            _theme: Default::default(),
            _renderer: Default::default(),
        }
    } 

    fn view(&self) {
        self.view.view(self.state);
    }

    fn update(&mut self, message: Message) {
        let _ = self.update.update(self.state, message);
    }
}

impl<'a, State, Message, Theme, Renderer, View> ui::Widget
    for AppState<'a, State, Message, Theme, Renderer, View>
where
    View: for<'b> self::View<'b, State, Message, Theme, Renderer>
{
    fn ui(
        self, 
        ui: &mut ui::Ui,
    ) -> crate::ui::Response {
        let mut messages: Vec<Message> = Vec::new();

        let shell = Shell::new(&mut messages);

        self.view();

        for message in messages {
            let _ = self.update.update(self.state, message);
        }

        todo!()
    }
}
