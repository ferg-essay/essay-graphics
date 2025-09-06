use essay_graphics::ui::{Tabs};
use essay_graphics_ui::{main_loop::MainLoop, widget2::{button, Element}};

fn main() { 
    let mut hello = StateHello::default();
    let mut there = StateThere::default();
    let mut button_b = StateB::default();
    let mut selected = String::from("a");

    MainLoop::new().show(move |ui| { // let view = UiView::new(move |ui| {

            let mut tabs = Tabs::<String>::new(selected.clone());
            tabs.item(String::from("a"), |ui| {
                ui.row(|ui| {
                    ui.view(|ui| {
                        ui.label("Option A");
                    });
                    ui.view(|ui| {
                        ui.app(&mut hello, StateHello::view, StateHello::update);
                    });
                    ui.view(|ui| {
                        ui.label("more");
                    });
                });
                ui.view(|ui| {
                    ui.app(&mut there, StateThere::view, StateThere::update);
                });
            });

            tabs.item(String::from("b"), |ui| {
                ui.column(|ui| {
                    ui.view(|ui| {
                        ui.label("Option B");
                    });
                    ui.view(|ui| {
                        ui.app(&mut button_b, StateB::view, StateB::update);
                    });
                    ui.view(|ui| {
                        ui.label("more");
                    });
                });
                ui.column(|ui| {
                    ui.label("Tail");
                    ui.label("AfterTail");
                });
            });
            selected = tabs.show(ui).unwrap();
    });

    //MainLoop::new().show(view);
}

#[derive(Default)]
struct StateHello {
    hello: bool,
}

impl StateHello {
    fn update(&mut self, message: Event) {
        match message { Event::Press => { self.hello = !self.hello; } }
    }

    fn view(&self) -> impl Into<Element<'_, Event>> {
        button("hello").press(self.hello).on_press(Event::Press)
    }
}

#[derive(Default)]
struct StateThere {
    there: bool,
}

impl StateThere {
    fn update(&mut self, message: Event) {
        match message { Event::Press => { self.there = !self.there; } }
    }

    fn view(&self) -> impl Into<Element<'_, Event>> {
        button("there").press(self.there).on_press(Event::Press)
    }
}

#[derive(Default)]
struct StateB {
    b: bool,
}

impl StateB {
    fn update(&mut self, message: Event) {
        match message { Event::Press => { self.b = !self.b; } }
    }

    fn view(&self) -> impl Into<Element<'_, Event>> {
        button("Button B").press(self.b).on_press(Event::Press)
    }
}

#[derive(Clone, PartialEq)]
enum Event {
    Press
}
