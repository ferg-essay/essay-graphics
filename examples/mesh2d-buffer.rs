use renderer::Renderer;
use essay_graphics::prelude::*;
use essay_graphics::layout::MainLoop;

fn main() { 
    MainLoop::new().show(move |ui: &mut dyn Renderer| {
        let mut mesh = Mesh2d::new();

        let p = Point::new(100., 200.);
        mesh.triangle(Point::new(0., 0.) + p, Point::new(100., 0.) + p, Point::new(100., 100.) + p);

        let buffer = ui.create_mesh2d_buffer(
            &mesh, 
        )?;

        ui.draw_mesh2d_buffer(
            &buffer, 
            TextureId::default(),
            &[Color::from("teal").into()]
        )?;

        let mut mesh = Mesh2d::new();

        let p = Point::new(300., 200.);
        // widdershins
        mesh.triangle(Point::new(0., 0.) + p, Point::new(100., 0.) + p, Point::new(100., 100.) + p);
        // clockwise
        mesh.triangle(Point::new(0., 100.) + p, Point::new(100., 100.) + p, Point::new(0., 0.) + p);
        mesh.triangle(Point::new(0., 100.) + p, Point::new(100., 100.) + p, Point::new(50., 200.) + p);

        ui.draw_mesh2d(
            &mesh, 
            TextureId::default(), 
            &[Color::from("orange").into()]
        )?;

        let p = Point::new(500., 200.);

        let mut mesh = Mesh2d::new();
        // overlap
        mesh.triangle(Point::new(0., 0.) + p, Point::new(100., 0.) + p, Point::new(100., 100.) + p);
        mesh.triangle(Point::new(0., 0.) + p, Point::new(100., 0.) + p, Point::new(0., 100.) + p);

        ui.draw_mesh2d(
            &mesh, 
            TextureId::default(),
            &[Color::from("azure").with_alpha(0.25).into()]
        )?;

        Ok(())
    });
}
