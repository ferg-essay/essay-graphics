use essay_graphics_ui::UiView;
use essay_graphics::layout::LayoutMainLoop;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    figure.view((0., 0., 2., 2.) , UiView::new(|ui| {
        ui.button("hello");
        ui.button("there");
    }));

    figure.show();
}
