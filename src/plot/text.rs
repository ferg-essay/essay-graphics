use essay_graphics_api::Point;

use crate::{artist::{TextOpt, Text}, graph::Graph};

pub fn text(
    graph: &mut Graph, 
    pos: impl Into<Point>, 
    text: impl AsRef<str>, 
) -> TextOpt {
    let text = Text::new(pos, text);

    graph.artist(text)
}
