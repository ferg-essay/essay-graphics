use essay_graphics::layout::Figure;

fn main() {
    //let mut gui = WgpuBackend::new();

    let mut figure = Figure::new();
    //let mut graph = figure.new_graph([1., 1.]);

    //let x = linspace(0., 2. * PI, 30);
    //let y = x.sin();

    //let x = linspace(0., 6.28, 20);
    //let y = x.sin();

    // graph.title("My Title"); // .color(0x008033).size(18.);
    // graph.plot(&x, &y);

    // figure.show();
    figure.save("test.png", 200.);
}
