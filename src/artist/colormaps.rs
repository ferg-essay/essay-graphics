use crate::artist::color::Hsv;

use super::ColorMap;

pub enum ColorMaps {
    Default,
    BlueOrange,
    WhiteRed,
    WhiteBlue,

    BlueYellow,
    BlackWhite,
    WhiteBlack,

    RedYellow,
    BlueWhite,
}

impl From<ColorMaps> for ColorMap {
    fn from(value: ColorMaps) -> Self {
        match value {
            ColorMaps::BlueOrange => {
                // use color temperature (hue) to reinforce transition from
                // unsaturated/bright to saturated/dark, which distinguishes
                // quartiles
                ColorMap::from([
                    // cool, saturated blue to warm, unsaturated blue
                    (0., Hsv(0.69, 0.92, 0.45)), // "css:midnightblue" bottom 1% distinct
                    (0.01, Hsv(0.66, 0.98, 0.65)), // "cobalt blue" 
                    (0.1, Hsv(0.61, 0.99, 0.87)), // "blue",
                    (0.25, Hsv(0.56, 0.80, 0.95)), // "css:dodgerblue",

                    (0.5, Hsv(0.25, 0.10, 0.97)), // "css:beige // touch of color between amber and azure

                    // cool, unsaturated orange to warm, saturated orange
                    (0.70, Hsv(0.13, 0.90, 0.97)), // "golden yellow"
                    (0.90, Hsv(0.06, 0.95, 0.97)), // "bright orange"
                    (0.99, Hsv(0.02, 1.0, 0.94)), // "tomato red"
                    (1.0, Hsv(0.99, 1., 0.90)), // "red" // top 1% distinct
                ])
            }

            ColorMaps::WhiteRed => {
                // use color temperature (hue) to reinforce transition from
                // unsaturated/bright to saturated/dark, which distinguishes
                // quartiles
                ColorMap::from([
                    (0.0, Hsv(0.25, 0.10, 0.97)), // "css:beige // touch of color between amber and azure

                    // cool, unsaturated orange to warm, saturated orange
                    (0.50, Hsv(0.13, 0.90, 0.97)), // "golden yellow"
                    (0.80, Hsv(0.06, 0.95, 0.97)), // "bright orange"
                    (0.99, Hsv(0.02, 1.0, 0.94)), // "tomato red"
                    (1.0, Hsv(0.99, 1., 0.90)), // "red" // top 1% distinct
                ])
            }

            ColorMaps::WhiteBlue => {
                // use color temperature (hue) to reinforce transition from
                // unsaturated/bright to saturated/dark, which distinguishes
                // quartiles
                ColorMap::from([
                    (0.0, Hsv(0.25, 0.10, 0.97)), // "css:beige // touch of color between amber and azure

                    (0.25, Hsv(0.56, 0.80, 0.95)), // "css:dodgerblue",
                    (0.80, Hsv(0.61, 0.99, 0.87)), // "blue",
                    (0.99, Hsv(0.66, 0.98, 0.65)), // "cobalt blue" 
                    (1., Hsv(0.69, 0.92, 0.45)), // "css:midnightblue" bottom 1% distinct
                ])
            }

            ColorMaps::RedYellow => {
                ColorMap::from([
                    (0.0, Hsv(0.95, 1., 0.05)),
                    (0.1, Hsv(0.95, 1., 0.2)),
                    (0.2, Hsv(0.95, 1., 0.4)),
                    (0.4, Hsv(0.02, 1., 0.85)),
                    (0.6, Hsv(0.05, 0.9, 0.9)),
                    (0.8, Hsv(0.10, 0.5, 0.97)),
                    (0.9, Hsv(0.13, 0.3, 0.97)),
                    (1.0, Hsv(0.15, 0.1, 0.97)),
                ])
            }

            ColorMaps::BlueYellow => {
                // Top 1% options: vermillion, red, bright red, tomato red
                // Bottom 1% options: navy, dark navy, ultramarine blue, night blue
                // royal blue, cobalt blue
                // TODO: possibly use hsv instead of color names

                ColorMap::from([
                    (0., "deep blue"),  // bottom 1% distinct
                    // cool, saturated blue to warm, unsaturated blue
                    (0.01, "cobalt blue"), (0.1, "blue"), (0.2, "azure"),
                    (0.5, "#a8a8a8"),
                    // cool, unsaturated orange to warm, saturated orange
                    (0.8, "amber"), (1., "yellow"), 
                ])
            }
            ColorMaps::BlueWhite | ColorMaps::Default => {
                // use color temperature (hue) to reinforce transition from
                // unsaturated/bright to saturated/dark, which distinguishes
                // quartiles
                ColorMap::from([
                    // cool, saturated blue to warm, unsaturated blue
                    (0., Hsv(0.69, 0.92, 0.45)), // "css:midnightblue" bottom 1% distinct
                    (0.02, Hsv(0.66, 0.98, 0.65)), // "cobalt blue" 
                    (0.2, Hsv(0.61, 0.99, 0.87)), // "blue",
                    (0.5, Hsv(0.56, 0.80, 0.95)), // "css:dodgerblue",

                    (1.0, Hsv(0.25, 0.10, 0.97)), // "css:beige // touch of color between amber and azure
                ])
            }

            ColorMaps::BlackWhite => {
                ColorMap::from([
                    (0., "black"),
                    (1., "white"),
                ])
            }

            ColorMaps::WhiteBlack => {
                ColorMap::from([
                    (0., "white"),
                    (1., "black"),
                ])
            }
        }
    }
}