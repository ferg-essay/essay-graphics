use essay_graphics::ui::UiView;
use essay_graphics::layout::LayoutMainLoop;

fn main() { 
    let mut figure = LayoutMainLoop::new();

    figure.view((0.2, 0.2, 2., 2.) , UiView::new(|ui| {
        ui.label("hello");
    }));

    figure.show();
}
