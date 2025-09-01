use std::marker::PhantomData;

use essay_graphics_api::{Size};

use crate::{ui::{ui::{self, MessageBase}, Response, Shell, Ui, Widget}, widget2::{Element, Task}};

pub fn application<State, Message>(
    update: impl Update<State, Message>,
    view: impl for<'a> View<'a, State, Message>,
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

pub trait View<'a, State, Message> {
    fn view(&self, state: &'a State) -> Element<'a, Message>;
}

impl<'a, T, State, Message, Widget> View<'a, State, Message> for T
where
    T: Fn(&'a State) -> Widget,
    State: 'static,
    Widget: Into<Element<'a, Message>>,
{
    fn view(&self, state: &'a State) -> Element<'a, Message> {
        self(state).into()
    }
}

pub struct AppState<'a, State, Message, View>
where
    View: for<'b> self::View<'b, State, Message>
{
    state: &'a mut State,
    update: Box<dyn Update<State, Message> + 'a>,
    view: Box<View>,
}

impl<'a, State, Message, View> AppState<'a, State, Message, View>
where
    View: for<'b> self::View<'b, State, Message>
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
        }
    } 

    fn view(&mut self, ui: &mut Ui) -> Response {
        let mut messages: Vec<Message> = Vec::new();
        let mut shell = Shell::new(&mut messages);

        self.draw(ui, &mut shell);
        // let response = (self.view)(ui);

        for message in messages {
            let _ = self.update(message);
        }

        ui.allocate_view(Size::new(1., 1.)).response
    }

    fn draw(&self, ui: &mut Ui, shell: &mut Shell<Message>) -> Response {
        let mut element = self.view.view(self.state);

        element.ui(ui, shell)
    }

    fn update(&mut self, message: Message) {
        let _ = self.update.update(self.state, message);
    }
}

impl<'a, State, Message, View> Widget<MessageBase>
    for AppState<'a, State, Message, View>
where
    View: for<'b> self::View<'b, State, Message>
{
    fn ui(
        &mut self, 
        ui: &mut ui::Ui,
        shell: &mut Shell<MessageBase>,
    ) -> crate::ui::Response {
        let mut messages: Vec<Message> = Vec::new();

        let mut shell = Shell::new(&mut messages);

        let response = self.draw(ui, &mut shell);

        for message in messages {
            let _ = self.update.update(self.state, message);
        }

        response
    }
}
