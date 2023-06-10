use std::{collections::HashMap, sync::Mutex};

use super::Color;

const COLORMAP: Mutex<ColorMap> = Mutex::new(ColorMap { map: None });

pub(crate) fn lookup_color(name: &str) -> Option<Color> {
    COLORMAP.lock().unwrap().color(name)
}

struct ColorMap {
    map: Option<HashMap<String, Color>>,
}

impl ColorMap {
    fn color(&mut self, name: &str) -> Option<Color> {
        if self.map.is_none() {
            self.map = Some(build_colormap());
        }

        match &self.map {
            Some(map) => match map.get(name) {
                Some(color) => Some(color.clone()),
                None => None,
            },
            None => None,
        }
    }
}

fn build_colormap() -> HashMap<String, Color> {
    let mut map = HashMap::<String, Color>::new();

    for (name, rgb) in css_color_data() {
        map.insert(format!("css:{}", name), Color::from(rgb));
    }

    for (name, rgb) in tableau_color_data() {
        map.insert(format!("tab:{}", name), Color::from(rgb));
    }

    for (name, rgb) in xkcd_color_data() {
        map.insert(format!("xkcd:{}", name), Color::from(rgb));
        map.insert(format!("{}", name), Color::from(rgb));
    }

    for (name, rgb) in digit_color_data() {
        map.insert(name.to_string(), Color::from(rgb));
    }

    map
}

///
/// using useful colors insted of RGB primaries
///
fn digit_color_data() -> Vec<(&'static str, u32)> {
    return vec![
        ("k", 0x000000),
        ("w", 0xffffff),
        ("r", 0xd62728), // xkcd:red
        ("g", 0x15b01a), // xkcd:green
        ("b", 0x0343df), // xkcd:blue
        ("c", 0x75bbfd), // xkcd:sky blue
        ("m", 0xc02078), // xkcd:magenta
        ("y", 0xffdf22), // xkcd:sun yellow
        ("o", 0xf97306), // xkcd:orange
        ("i", 0x380282), // xkcd:indigo
        ("v", 0x9a0eea), // xkcd:violet
    ];
}

///
/// Tableau colors
///
fn tableau_color_data() -> Vec<(&'static str, u32)> {
    return vec![
        ("blue", 0x1f77b4),
        ("orange", 0xff7f0e),
        ("green", 0x2ca02c),
        ("red", 0xd62728),
        ("purple", 0x9467bd),
        ("brown", 0x8c564b),
        ("pink", 0xe377c2),
        ("gray", 0x7f7f7f),
        ("olive", 0xbcbd22),
        ("cyan", 0x17becf),
    ];
}

fn xkcd_color_data() -> Vec<(&'static str, u32)> {
    // https://xkcd.com/color/rgb.txt
    // # License: https://creativecommons.org/publicdomain/zero/1.0/
    return vec![
        ("cloudy blue", 0xacc2d9),
        ("dark pastel green", 0x56ae57),
        ("dust", 0xb2996e),
        ("electric lime", 0xa8ff04),
        ("fresh green", 0x69d84f),
        ("light eggplant", 0x894585),
        ("nasty green", 0x70b23f),
        ("really light blue", 0xd4ffff),
        ("tea", 0x65ab7c),
        ("warm purple", 0x952e8f),
        ("yellowish tan", 0xfcfc81),
        ("cement", 0xa5a391),
        ("dark grass green", 0x388004),
        ("dusty teal", 0x4c9085),
        ("grey teal", 0x5e9b8a),
        ("macaroni and cheese", 0xefb435),
        ("pinkish tan", 0xd99b82),
        ("spruce", 0x0a5f38),
        ("strong blue", 0x0c06f7),
        ("toxic green", 0x61de2a),
        ("windows blue", 0x3778bf),
        ("blue blue", 0x2242c7),
        ("blue with a hint of purple", 0x533cc6),
        ("booger", 0x9bb53c),
        ("bright sea green", 0x05ffa6),
        ("dark green blue", 0x1f6357),
        ("deep turquoise", 0x017374),
        ("green teal", 0x0cb577),
        ("strong pink", 0xff0789),
        ("bland", 0xafa88b),
        ("deep aqua", 0x08787f),
        ("lavender pink", 0xdd85d7),
        ("light moss green", 0xa6c875),
        ("light seafoam green", 0xa7ffb5),
        ("olive yellow", 0xc2b709),
        ("pig pink", 0xe78ea5),
        ("deep lilac", 0x966ebd),
        ("desert", 0xccad60),
        ("dusty lavender", 0xac86a8),
        ("purpley grey", 0x947e94),
        ("purply", 0x983fb2),
        ("candy pink", 0xff63e9),
        ("light pastel green", 0xb2fba5),
        ("boring green", 0x63b365),
        ("kiwi green", 0x8ee53f),
        ("light grey green", 0xb7e1a1),
        ("orange pink", 0xff6f52),
        ("tea green", 0xbdf8a3),
        ("very light brown", 0xd3b683),
        ("egg shell", 0xfffcc4),
        ("eggplant purple", 0x430541),
        ("powder pink", 0xffb2d0),
        ("reddish grey", 0x997570),
        ("baby shit brown", 0xad900d),
        ("liliac", 0xc48efd),
        ("stormy blue", 0x507b9c),
        ("ugly brown", 0x7d7103),
        ("custard", 0xfffd78),
        ("darkish pink", 0xda467d),
        ("deep brown", 0x410200),
        ("greenish beige", 0xc9d179),
        ("manilla", 0xfffa86),
        ("off blue", 0x5684ae),
        ("battleship grey", 0x6b7c85),
        ("browny green", 0x6f6c0a),
        ("bruise", 0x7e4071),
        ("kelley green", 0x009337),
        ("sickly yellow", 0xd0e429),
        ("sunny yellow", 0xfff917),
        ("azul", 0x1d5dec),
        ("darkgreen", 0x054907),
        ("green/yellow", 0xb5ce08),
        ("lichen", 0x8fb67b),
        ("light light green", 0xc8ffb0),
        ("pale gold", 0xfdde6c),
        ("sun yellow", 0xffdf22),
        ("tan green", 0xa9be70),
        ("burple", 0x6832e3),
        ("butterscotch", 0xfdb147),
        ("toupe", 0xc7ac7d),
        ("dark cream", 0xfff39a),
        ("indian red", 0x850e04),
        ("light lavendar", 0xefc0fe),
        ("poison green", 0x40fd14),
        ("baby puke green", 0xb6c406),
        ("bright yellow green", 0x9dff00),
        ("charcoal grey", 0x3c4142),
        ("squash", 0xf2ab15),
        ("cinnamon", 0xac4f06),
        ("light pea green", 0xc4fe82),
        ("radioactive green", 0x2cfa1f),
        ("raw sienna", 0x9a6200),
        ("baby purple", 0xca9bf7),
        ("cocoa", 0x875f42),
        ("light royal blue", 0x3a2efe),
        ("orangeish", 0xfd8d49),
        ("rust brown", 0x8b3103),
        ("sand brown", 0xcba560),
        ("swamp", 0x698339),
        ("tealish green", 0x0cdc73),
        ("burnt siena", 0xb75203),
        ("camo", 0x7f8f4e),
        ("dusk blue", 0x26538d),
        ("fern", 0x63a950),
        ("old rose", 0xc87f89),
        ("pale light green", 0xb1fc99),
        ("peachy pink", 0xff9a8a),
        ("rosy pink", 0xf6688e),
        ("light bluish green", 0x76fda8),
        ("light bright green", 0x53fe5c),
        ("light neon green", 0x4efd54),
        ("light seafoam", 0xa0febf),
        ("tiffany blue", 0x7bf2da),
        ("washed out green", 0xbcf5a6),
        ("browny orange", 0xca6b02),
        ("nice blue", 0x107ab0),
        ("sapphire", 0x2138ab),
        ("greyish teal", 0x719f91),
        ("orangey yellow", 0xfdb915),
        ("parchment", 0xfefcaf),
        ("straw", 0xfcf679),
        ("very dark brown", 0x1d0200),
        ("terracota", 0xcb6843),
        ("ugly blue", 0x31668a),
        ("clear blue", 0x247afd),
        ("creme", 0xffffb6),
        ("foam green", 0x90fda9),
        ("grey/green", 0x86a17d),
        ("light gold", 0xfddc5c),
        ("seafoam blue", 0x78d1b6),
        ("topaz", 0x13bbaf),
        ("violet pink", 0xfb5ffc),
        ("wintergreen", 0x20f986),
        ("yellow tan", 0xffe36e),
        ("dark fuchsia", 0x9d0759),
        ("indigo blue", 0x3a18b1),
        ("light yellowish green", 0xc2ff89),
        ("pale magenta", 0xd767ad),
        ("rich purple", 0x720058),
        ("sunflower yellow", 0xffda03),
        ("green/blue", 0x01c08d),
        ("leather", 0xac7434),
        ("racing green", 0x014600),
        ("vivid purple", 0x9900fa),
        ("dark royal blue", 0x02066f),
        ("hazel", 0x8e7618),
        ("muted pink", 0xd1768f),
        ("booger green", 0x96b403),
        ("canary", 0xfdff63),
        ("cool grey", 0x95a3a6),
        ("dark taupe", 0x7f684e),
        ("darkish purple", 0x751973),
        ("true green", 0x089404),
        ("coral pink", 0xff6163),
        ("dark sage", 0x598556),
        ("dark slate blue", 0x214761),
        ("flat blue", 0x3c73a8),
        ("mushroom", 0xba9e88),
        ("rich blue", 0x021bf9),
        ("dirty purple", 0x734a65),
        ("greenblue", 0x23c48b),
        ("icky green", 0x8fae22),
        ("light khaki", 0xe6f2a2),
        ("warm blue", 0x4b57db),
        ("dark hot pink", 0xd90166),
        ("deep sea blue", 0x015482),
        ("carmine", 0x9d0216),
        ("dark yellow green", 0x728f02),
        ("pale peach", 0xffe5ad),
        ("plum purple", 0x4e0550),
        ("golden rod", 0xf9bc08),
        ("neon red", 0xff073a),
        ("old pink", 0xc77986),
        ("very pale blue", 0xd6fffe),
        ("blood orange", 0xfe4b03),
        ("grapefruit", 0xfd5956),
        ("sand yellow", 0xfce166),
        ("clay brown", 0xb2713d),
        ("dark blue grey", 0x1f3b4d),
        ("flat green", 0x699d4c),
        ("light green blue", 0x56fca2),
        ("warm pink", 0xfb5581),
        ("dodger blue", 0x3e82fc),
        ("gross green", 0xa0bf16),
        ("ice", 0xd6fffa),
        ("metallic blue", 0x4f738e),
        ("pale salmon", 0xffb19a),
        ("sap green", 0x5c8b15),
        ("algae", 0x54ac68),
        ("bluey grey", 0x89a0b0),
        ("greeny grey", 0x7ea07a),
        ("highlighter green", 0x1bfc06),
        ("light light blue", 0xcafffb),
        ("light mint", 0xb6ffbb),
        ("raw umber", 0xa75e09),
        ("vivid blue", 0x152eff),
        ("deep lavender", 0x8d5eb7),
        ("dull teal", 0x5f9e8f),
        ("light greenish blue", 0x63f7b4),
        ("mud green", 0x606602),
        ("pinky", 0xfc86aa),
        ("red wine", 0x8c0034),
        ("shit green", 0x758000),
        ("tan brown", 0xab7e4c),
        ("darkblue", 0x030764),
        ("rosa", 0xfe86a4),
        ("lipstick", 0xd5174e),
        ("pale mauve", 0xfed0fc),
        ("claret", 0x680018),
        ("dandelion", 0xfedf08),
        ("orangered", 0xfe420f),
        ("poop green", 0x6f7c00),
        ("ruby", 0xca0147),
        ("dark", 0x1b2431),
        ("greenish turquoise", 0x00fbb0),
        ("pastel red", 0xdb5856),
        ("piss yellow", 0xddd618),
        ("bright cyan", 0x41fdfe),
        ("dark coral", 0xcf524e),
        ("algae green", 0x21c36f),
        ("darkish red", 0xa90308),
        ("reddy brown", 0x6e1005),
        ("blush pink", 0xfe828c),
        ("camouflage green", 0x4b6113),
        ("lawn green", 0x4da409),
        ("putty", 0xbeae8a),
        ("vibrant blue", 0x0339f8),
        ("dark sand", 0xa88f59),
        ("purple/blue", 0x5d21d0),
        ("saffron", 0xfeb209),
        ("twilight", 0x4e518b),
        ("warm brown", 0x964e02),
        ("bluegrey", 0x85a3b2),
        ("bubble gum pink", 0xff69af),
        ("duck egg blue", 0xc3fbf4),
        ("greenish cyan", 0x2afeb7),
        ("petrol", 0x005f6a),
        ("royal", 0x0c1793),
        ("butter", 0xffff81),
        ("dusty orange", 0xf0833a),
        ("off yellow", 0xf1f33f),
        ("pale olive green", 0xb1d27b),
        ("orangish", 0xfc824a),
        ("leaf", 0x71aa34),
        ("light blue grey", 0xb7c9e2),
        ("dried blood", 0x4b0101),
        ("lightish purple", 0xa552e6),
        ("rusty red", 0xaf2f0d),
        ("lavender blue", 0x8b88f8),
        ("light grass green", 0x9af764),
        ("light mint green", 0xa6fbb2),
        ("sunflower", 0xffc512),
        ("velvet", 0x750851),
        ("brick orange", 0xc14a09),
        ("lightish red", 0xfe2f4a),
        ("pure blue", 0x0203e2),
        ("twilight blue", 0x0a437a),
        ("violet red", 0xa50055),
        ("yellowy brown", 0xae8b0c),
        ("carnation", 0xfd798f),
        ("muddy yellow", 0xbfac05),
        ("dark seafoam green", 0x3eaf76),
        ("deep rose", 0xc74767),
        ("dusty red", 0xb9484e),
        ("grey/blue", 0x647d8e),
        ("lemon lime", 0xbffe28),
        ("purple/pink", 0xd725de),
        ("brown yellow", 0xb29705),
        ("purple brown", 0x673a3f),
        ("wisteria", 0xa87dc2),
        ("banana yellow", 0xfafe4b),
        ("lipstick red", 0xc0022f),
        ("water blue", 0x0e87cc),
        ("brown grey", 0x8d8468),
        ("vibrant purple", 0xad03de),
        ("baby green", 0x8cff9e),
        ("barf green", 0x94ac02),
        ("eggshell blue", 0xc4fff7),
        ("sandy yellow", 0xfdee73),
        ("cool green", 0x33b864),
        ("pale", 0xfff9d0),
        ("blue/grey", 0x758da3),
        ("hot magenta", 0xf504c9),
        ("greyblue", 0x77a1b5),
        ("purpley", 0x8756e4),
        ("baby shit green", 0x889717),
        ("brownish pink", 0xc27e79),
        ("dark aquamarine", 0x017371),
        ("diarrhea", 0x9f8303),
        ("light mustard", 0xf7d560),
        ("pale sky blue", 0xbdf6fe),
        ("turtle green", 0x75b84f),
        ("bright olive", 0x9cbb04),
        ("dark grey blue", 0x29465b),
        ("greeny brown", 0x696006),
        ("lemon green", 0xadf802),
        ("light periwinkle", 0xc1c6fc),
        ("seaweed green", 0x35ad6b),
        ("sunshine yellow", 0xfffd37),
        ("ugly purple", 0xa442a0),
        ("medium pink", 0xf36196),
        ("puke brown", 0x947706),
        ("very light pink", 0xfff4f2),
        ("viridian", 0x1e9167),
        ("bile", 0xb5c306),
        ("faded yellow", 0xfeff7f),
        ("very pale green", 0xcffdbc),
        ("vibrant green", 0x0add08),
        ("bright lime", 0x87fd05),
        ("spearmint", 0x1ef876),
        ("light aquamarine", 0x7bfdc7),
        ("light sage", 0xbcecac),
        ("yellowgreen", 0xbbf90f),
        ("baby poo", 0xab9004),
        ("dark seafoam", 0x1fb57a),
        ("deep teal", 0x00555a),
        ("heather", 0xa484ac),
        ("rust orange", 0xc45508),
        ("dirty blue", 0x3f829d),
        ("fern green", 0x548d44),
        ("bright lilac", 0xc95efb),
        ("weird green", 0x3ae57f),
        ("peacock blue", 0x016795),
        ("avocado green", 0x87a922),
        ("faded orange", 0xf0944d),
        ("grape purple", 0x5d1451),
        ("hot green", 0x25ff29),
        ("lime yellow", 0xd0fe1d),
        ("mango", 0xffa62b),
        ("shamrock", 0x01b44c),
        ("bubblegum", 0xff6cb5),
        ("purplish brown", 0x6b4247),
        ("vomit yellow", 0xc7c10c),
        ("pale cyan", 0xb7fffa),
        ("key lime", 0xaeff6e),
        ("tomato red", 0xec2d01),
        ("lightgreen", 0x76ff7b),
        ("merlot", 0x730039),
        ("night blue", 0x040348),
        ("purpleish pink", 0xdf4ec8),
        ("apple", 0x6ecb3c),
        ("baby poop green", 0x8f9805),
        ("green apple", 0x5edc1f),
        ("heliotrope", 0xd94ff5),
        ("yellow/green", 0xc8fd3d),
        ("almost black", 0x070d0d),
        ("cool blue", 0x4984b8),
        ("leafy green", 0x51b73b),
        ("mustard brown", 0xac7e04),
        ("dusk", 0x4e5481),
        ("dull brown", 0x876e4b),
        ("frog green", 0x58bc08),
        ("vivid green", 0x2fef10),
        ("bright light green", 0x2dfe54),
        ("fluro green", 0x0aff02),
        ("kiwi", 0x9cef43),
        ("seaweed", 0x18d17b),
        ("navy green", 0x35530a),
        ("ultramarine blue", 0x1805db),
        ("iris", 0x6258c4),
        ("pastel orange", 0xff964f),
        ("yellowish orange", 0xffab0f),
        ("perrywinkle", 0x8f8ce7),
        ("tealish", 0x24bca8),
        ("dark plum", 0x3f012c),
        ("pear", 0xcbf85f),
        ("pinkish orange", 0xff724c),
        ("midnight purple", 0x280137),
        ("light urple", 0xb36ff6),
        ("dark mint", 0x48c072),
        ("greenish tan", 0xbccb7a),
        ("light burgundy", 0xa8415b),
        ("turquoise blue", 0x06b1c4),
        ("ugly pink", 0xcd7584),
        ("sandy", 0xf1da7a),
        ("electric pink", 0xff0490),
        ("muted purple", 0x805b87),
        ("mid green", 0x50a747),
        ("greyish", 0xa8a495),
        ("neon yellow", 0xcfff04),
        ("banana", 0xffff7e),
        ("carnation pink", 0xff7fa7),
        ("tomato", 0xef4026),
        ("sea", 0x3c9992),
        ("muddy brown", 0x886806),
        ("turquoise green", 0x04f489),
        ("buff", 0xfef69e),
        ("fawn", 0xcfaf7b),
        ("muted blue", 0x3b719f),
        ("pale rose", 0xfdc1c5),
        ("dark mint green", 0x20c073),
        ("amethyst", 0x9b5fc0),
        ("blue/green", 0x0f9b8e),
        ("chestnut", 0x742802),
        ("sick green", 0x9db92c),
        ("pea", 0xa4bf20),
        ("rusty orange", 0xcd5909),
        ("stone", 0xada587),
        ("rose red", 0xbe013c),
        ("pale aqua", 0xb8ffeb),
        ("deep orange", 0xdc4d01),
        ("earth", 0xa2653e),
        ("mossy green", 0x638b27),
        ("grassy green", 0x419c03),
        ("pale lime green", 0xb1ff65),
        ("light grey blue", 0x9dbcd4),
        ("pale grey", 0xfdfdfe),
        ("asparagus", 0x77ab56),
        ("blueberry", 0x464196),
        ("purple red", 0x990147),
        ("pale lime", 0xbefd73),
        ("greenish teal", 0x32bf84),
        ("caramel", 0xaf6f09),
        ("deep magenta", 0xa0025c),
        ("light peach", 0xffd8b1),
        ("milk chocolate", 0x7f4e1e),
        ("ocher", 0xbf9b0c),
        ("off green", 0x6ba353),
        ("purply pink", 0xf075e6),
        ("lightblue", 0x7bc8f6),
        ("dusky blue", 0x475f94),
        ("golden", 0xf5bf03),
        ("light beige", 0xfffeb6),
        ("butter yellow", 0xfffd74),
        ("dusky purple", 0x895b7b),
        ("french blue", 0x436bad),
        ("ugly yellow", 0xd0c101),
        ("greeny yellow", 0xc6f808),
        ("orangish red", 0xf43605),
        ("shamrock green", 0x02c14d),
        ("orangish brown", 0xb25f03),
        ("tree green", 0x2a7e19),
        ("deep violet", 0x490648),
        ("gunmetal", 0x536267),
        ("blue/purple", 0x5a06ef),
        ("cherry", 0xcf0234),
        ("sandy brown", 0xc4a661),
        ("warm grey", 0x978a84),
        ("dark indigo", 0x1f0954),
        ("midnight", 0x03012d),
        ("bluey green", 0x2bb179),
        ("grey pink", 0xc3909b),
        ("soft purple", 0xa66fb5),
        ("blood", 0x770001),
        ("brown red", 0x922b05),
        ("medium grey", 0x7d7f7c),
        ("berry", 0x990f4b),
        ("poo", 0x8f7303),
        ("purpley pink", 0xc83cb9),
        ("light salmon", 0xfea993),
        ("snot", 0xacbb0d),
        ("easter purple", 0xc071fe),
        ("light yellow green", 0xccfd7f),
        ("dark navy blue", 0x00022e),
        ("drab", 0x828344),
        ("light rose", 0xffc5cb),
        ("rouge", 0xab1239),
        ("purplish red", 0xb0054b),
        ("slime green", 0x99cc04),
        ("baby poop", 0x937c00),
        ("irish green", 0x019529),
        ("pink/purple", 0xef1de7),
        ("dark navy", 0x000435),
        ("greeny blue", 0x42b395),
        ("light plum", 0x9d5783),
        ("pinkish grey", 0xc8aca9),
        ("dirty orange", 0xc87606),
        ("rust red", 0xaa2704),
        ("pale lilac", 0xe4cbff),
        ("orangey red", 0xfa4224),
        ("primary blue", 0x0804f9),
        ("kermit green", 0x5cb200),
        ("brownish purple", 0x76424e),
        ("murky green", 0x6c7a0e),
        ("wheat", 0xfbdd7e),
        ("very dark purple", 0x2a0134),
        ("bottle green", 0x044a05),
        ("watermelon", 0xfd4659),
        ("deep sky blue", 0x0d75f8),
        ("fire engine red", 0xfe0002),
        ("yellow ochre", 0xcb9d06),
        ("pumpkin orange", 0xfb7d07),
        ("pale olive", 0xb9cc81),
        ("light lilac", 0xedc8ff),
        ("lightish green", 0x61e160),
        ("carolina blue", 0x8ab8fe),
        ("mulberry", 0x920a4e),
        ("shocking pink", 0xfe02a2),
        ("auburn", 0x9a3001),
        ("bright lime green", 0x65fe08),
        ("celadon", 0xbefdb7),
        ("pinkish brown", 0xb17261),
        ("poo brown", 0x885f01),
        ("bright sky blue", 0x02ccfe),
        ("celery", 0xc1fd95),
        ("dirt brown", 0x836539),
        ("strawberry", 0xfb2943),
        ("dark lime", 0x84b701),
        ("copper", 0xb66325),
        ("medium brown", 0x7f5112),
        ("muted green", 0x5fa052),
        ("robin's egg", 0x6dedfd),
        ("bright aqua", 0x0bf9ea),
        ("bright lavender", 0xc760ff),
        ("ivory", 0xffffcb),
        ("very light purple", 0xf6cefc),
        ("light navy", 0x155084),
        ("pink red", 0xf5054f),
        ("olive brown", 0x645403),
        ("poop brown", 0x7a5901),
        ("mustard green", 0xa8b504),
        ("ocean green", 0x3d9973),
        ("very dark blue", 0x000133),
        ("dusty green", 0x76a973),
        ("light navy blue", 0x2e5a88),
        ("minty green", 0x0bf77d),
        ("adobe", 0xbd6c48),
        ("barney", 0xac1db8),
        ("jade green", 0x2baf6a),
        ("bright light blue", 0x26f7fd),
        ("light lime", 0xaefd6c),
        ("dark khaki", 0x9b8f55),
        ("orange yellow", 0xffad01),
        ("ocre", 0xc69c04),
        ("maize", 0xf4d054),
        ("faded pink", 0xde9dac),
        ("british racing green", 0x05480d),
        ("sandstone", 0xc9ae74),
        ("mud brown", 0x60460f),
        ("light sea green", 0x98f6b0),
        ("robin egg blue", 0x8af1fe),
        ("aqua marine", 0x2ee8bb),
        ("dark sea green", 0x11875d),
        ("soft pink", 0xfdb0c0),
        ("orangey brown", 0xb16002),
        ("cherry red", 0xf7022a),
        ("burnt yellow", 0xd5ab09),
        ("brownish grey", 0x86775f),
        ("camel", 0xc69f59),
        ("purplish grey", 0x7a687f),
        ("marine", 0x042e60),
        ("greyish pink", 0xc88d94),
        ("pale turquoise", 0xa5fbd5),
        ("pastel yellow", 0xfffe71),
        ("bluey purple", 0x6241c7),
        ("canary yellow", 0xfffe40),
        ("faded red", 0xd3494e),
        ("sepia", 0x985e2b),
        ("coffee", 0xa6814c),
        ("bright magenta", 0xff08e8),
        ("mocha", 0x9d7651),
        ("ecru", 0xfeffca),
        ("purpleish", 0x98568d),
        ("cranberry", 0x9e003a),
        ("darkish green", 0x287c37),
        ("brown orange", 0xb96902),
        ("dusky rose", 0xba6873),
        ("melon", 0xff7855),
        ("sickly green", 0x94b21c),
        ("silver", 0xc5c9c7),
        ("purply blue", 0x661aee),
        ("purpleish blue", 0x6140ef),
        ("hospital green", 0x9be5aa),
        ("shit brown", 0x7b5804),
        ("mid blue", 0x276ab3),
        ("amber", 0xfeb308),
        ("easter green", 0x8cfd7e),
        ("soft blue", 0x6488ea),
        ("cerulean blue", 0x056eee),
        ("golden brown", 0xb27a01),
        ("bright turquoise", 0x0ffef9),
        ("red pink", 0xfa2a55),
        ("red purple", 0x820747),
        ("greyish brown", 0x7a6a4f),
        ("vermillion", 0xf4320c),
        ("russet", 0xa13905),
        ("steel grey", 0x6f828a),
        ("lighter purple", 0xa55af4),
        ("bright violet", 0xad0afd),
        ("prussian blue", 0x004577),
        ("slate green", 0x658d6d),
        ("dirty pink", 0xca7b80),
        ("dark blue green", 0x005249),
        ("pine", 0x2b5d34),
        ("yellowy green", 0xbff128),
        ("dark gold", 0xb59410),
        ("bluish", 0x2976bb),
        ("darkish blue", 0x014182),
        ("dull red", 0xbb3f3f),
        ("pinky red", 0xfc2647),
        ("bronze", 0xa87900),
        ("pale teal", 0x82cbb2),
        ("military green", 0x667c3e),
        ("barbie pink", 0xfe46a5),
        ("bubblegum pink", 0xfe83cc),
        ("pea soup green", 0x94a617),
        ("dark mustard", 0xa88905),
        ("shit", 0x7f5f00),
        ("medium purple", 0x9e43a2),
        ("very dark green", 0x062e03),
        ("dirt", 0x8a6e45),
        ("dusky pink", 0xcc7a8b),
        ("red violet", 0x9e0168),
        ("lemon yellow", 0xfdff38),
        ("pistachio", 0xc0fa8b),
        ("dull yellow", 0xeedc5b),
        ("dark lime green", 0x7ebd01),
        ("denim blue", 0x3b5b92),
        ("teal blue", 0x01889f),
        ("lightish blue", 0x3d7afd),
        ("purpley blue", 0x5f34e7),
        ("light indigo", 0x6d5acf),
        ("swamp green", 0x748500),
        ("brown green", 0x706c11),
        ("dark maroon", 0x3c0008),
        ("hot purple", 0xcb00f5),
        ("dark forest green", 0x002d04),
        ("faded blue", 0x658cbb),
        ("drab green", 0x749551),
        ("light lime green", 0xb9ff66),
        ("snot green", 0x9dc100),
        ("yellowish", 0xfaee66),
        ("light blue green", 0x7efbb3),
        ("bordeaux", 0x7b002c),
        ("light mauve", 0xc292a1),
        ("ocean", 0x017b92),
        ("marigold", 0xfcc006),
        ("muddy green", 0x657432),
        ("dull orange", 0xd8863b),
        ("steel", 0x738595),
        ("electric purple", 0xaa23ff),
        ("fluorescent green", 0x08ff08),
        ("yellowish brown", 0x9b7a01),
        ("blush", 0xf29e8e),
        ("soft green", 0x6fc276),
        ("bright orange", 0xff5b00),
        ("lemon", 0xfdff52),
        ("purple grey", 0x866f85),
        ("acid green", 0x8ffe09),
        ("pale lavender", 0xeecffe),
        ("violet blue", 0x510ac9),
        ("light forest green", 0x4f9153),
        ("burnt red", 0x9f2305),
        ("khaki green", 0x728639),
        ("cerise", 0xde0c62),
        ("faded purple", 0x916e99),
        ("apricot", 0xffb16d),
        ("dark olive green", 0x3c4d03),
        ("grey brown", 0x7f7053),
        ("green grey", 0x77926f),
        ("true blue", 0x010fcc),
        ("pale violet", 0xceaefa),
        ("periwinkle blue", 0x8f99fb),
        ("light sky blue", 0xc6fcff),
        ("blurple", 0x5539cc),
        ("green brown", 0x544e03),
        ("bluegreen", 0x017a79),
        ("bright teal", 0x01f9c6),
        ("brownish yellow", 0xc9b003),
        ("pea soup", 0x929901),
        ("forest", 0x0b5509),
        ("barney purple", 0xa00498),
        ("ultramarine", 0x2000b1),
        ("purplish", 0x94568c),
        ("puke yellow", 0xc2be0e),
        ("bluish grey", 0x748b97),
        ("dark periwinkle", 0x665fd1),
        ("dark lilac", 0x9c6da5),
        ("reddish", 0xc44240),
        ("light maroon", 0xa24857),
        ("dusty purple", 0x825f87),
        ("terra cotta", 0xc9643b),
        ("avocado", 0x90b134),
        ("marine blue", 0x01386a),
        ("teal green", 0x25a36f),
        ("slate grey", 0x59656d),
        ("lighter green", 0x75fd63),
        ("electric green", 0x21fc0d),
        ("dusty blue", 0x5a86ad),
        ("golden yellow", 0xfec615),
        ("bright yellow", 0xfffd01),
        ("light lavender", 0xdfc5fe),
        ("umber", 0xb26400),
        ("poop", 0x7f5e00),
        ("dark peach", 0xde7e5d),
        ("jungle green", 0x048243),
        ("eggshell", 0xffffd4),
        ("denim", 0x3b638c),
        ("yellow brown", 0xb79400),
        ("dull purple", 0x84597e),
        ("chocolate brown", 0x411900),
        ("wine red", 0x7b0323),
        ("neon blue", 0x04d9ff),
        ("dirty green", 0x667e2c),
        ("light tan", 0xfbeeac),
        ("ice blue", 0xd7fffe),
        ("cadet blue", 0x4e7496),
        ("dark mauve", 0x874c62),
        ("very light blue", 0xd5ffff),
        ("grey purple", 0x826d8c),
        ("pastel pink", 0xffbacd),
        ("very light green", 0xd1ffbd),
        ("dark sky blue", 0x448ee4),
        ("evergreen", 0x05472a),
        ("dull pink", 0xd5869d),
        ("aubergine", 0x3d0734),
        ("mahogany", 0x4a0100),
        ("reddish orange", 0xf8481c),
        ("deep green", 0x02590f),
        ("vomit green", 0x89a203),
        ("purple pink", 0xe03fd8),
        ("dusty pink", 0xd58a94),
        ("faded green", 0x7bb274),
        ("camo green", 0x526525),
        ("pinky purple", 0xc94cbe),
        ("pink purple", 0xdb4bda),
        ("brownish red", 0x9e3623),
        ("dark rose", 0xb5485d),
        ("mud", 0x735c12),
        ("brownish", 0x9c6d57),
        ("emerald green", 0x028f1e),
        ("pale brown", 0xb1916e),
        ("dull blue", 0x49759c),
        ("burnt umber", 0xa0450e),
        ("medium green", 0x39ad48),
        ("clay", 0xb66a50),
        ("light aqua", 0x8cffdb),
        ("light olive green", 0xa4be5c),
        ("brownish orange", 0xcb7723),
        ("dark aqua", 0x05696b),
        ("purplish pink", 0xce5dae),
        ("dark salmon", 0xc85a53),
        ("greenish grey", 0x96ae8d),
        ("jade", 0x1fa774),
        ("ugly green", 0x7a9703),
        ("dark beige", 0xac9362),
        ("emerald", 0x01a049),
        ("pale red", 0xd9544d),
        ("light magenta", 0xfa5ff7),
        ("sky", 0x82cafc),
        ("light cyan", 0xacfffc),
        ("yellow orange", 0xfcb001),
        ("reddish purple", 0x910951),
        ("reddish pink", 0xfe2c54),
        ("orchid", 0xc875c4),
        ("dirty yellow", 0xcdc50a),
        ("orange red", 0xfd411e),
        ("deep red", 0x9a0200),
        ("orange brown", 0xbe6400),
        ("cobalt blue", 0x030aa7),
        ("neon pink", 0xfe019a),
        ("rose pink", 0xf7879a),
        ("greyish purple", 0x887191),
        ("raspberry", 0xb00149),
        ("aqua green", 0x12e193),
        ("salmon pink", 0xfe7b7c),
        ("tangerine", 0xff9408),
        ("brownish green", 0x6a6e09),
        ("red brown", 0x8b2e16),
        ("greenish brown", 0x696112),
        ("pumpkin", 0xe17701),
        ("pine green", 0x0a481e),
        ("charcoal", 0x343837),
        ("baby pink", 0xffb7ce),
        ("cornflower", 0x6a79f7),
        ("blue violet", 0x5d06e9),
        ("chocolate", 0x3d1c02),
        ("greyish green", 0x82a67d),
        ("scarlet", 0xbe0119),
        ("green yellow", 0xc9ff27),
        ("dark olive", 0x373e02),
        ("sienna", 0xa9561e),
        ("pastel purple", 0xcaa0ff),
        ("terracotta", 0xca6641),
        ("aqua blue", 0x02d8e9),
        ("sage green", 0x88b378),
        ("blood red", 0x980002),
        ("deep pink", 0xcb0162),
        ("grass", 0x5cac2d),
        ("moss", 0x769958),
        ("pastel blue", 0xa2bffe),
        ("bluish green", 0x10a674),
        ("green blue", 0x06b48b),
        ("dark tan", 0xaf884a),
        ("greenish blue", 0x0b8b87),
        ("pale orange", 0xffa756),
        ("vomit", 0xa2a415),
        ("forrest green", 0x154406),
        ("dark lavender", 0x856798),
        ("dark violet", 0x34013f),
        ("purple blue", 0x632de9),
        ("dark cyan", 0x0a888a),
        ("olive drab", 0x6f7632),
        ("pinkish", 0xd46a7e),
        ("cobalt", 0x1e488f),
        ("neon purple", 0xbc13fe),
        ("light turquoise", 0x7ef4cc),
        ("apple green", 0x76cd26),
        ("dull green", 0x74a662),
        ("wine", 0x80013f),
        ("powder blue", 0xb1d1fc),
        ("off white", 0xffffe4),
        ("electric blue", 0x0652ff),
        ("dark turquoise", 0x045c5a),
        ("blue purple", 0x5729ce),
        ("azure", 0x069af3),
        ("bright red", 0xff000d),
        ("pinkish red", 0xf10c45),
        ("cornflower blue", 0x5170d7),
        ("light olive", 0xacbf69),
        ("grape", 0x6c3461),
        ("greyish blue", 0x5e819d),
        ("purplish blue", 0x601ef9),
        ("yellowish green", 0xb0dd16),
        ("greenish yellow", 0xcdfd02),
        ("medium blue", 0x2c6fbb),
        ("dusty rose", 0xc0737a),
        ("light violet", 0xd6b4fc),
        ("midnight blue", 0x020035),
        ("bluish purple", 0x703be7),
        ("red orange", 0xfd3c06),
        ("dark magenta", 0x960056),
        ("greenish", 0x40a368),
        ("ocean blue", 0x03719c),
        ("coral", 0xfc5a50),
        ("cream", 0xffffc2),
        ("reddish brown", 0x7f2b0a),
        ("burnt sienna", 0xb04e0f),
        ("brick", 0xa03623),
        ("sage", 0x87ae73),
        ("grey green", 0x789b73),
        ("white", 0xffffff),
        ("robin's egg blue", 0x98eff9),
        ("moss green", 0x658b38),
        ("steel blue", 0x5a7d9a),
        ("eggplant", 0x380835),
        ("light yellow", 0xfffe7a),
        ("leaf green", 0x5ca904),
        ("light grey", 0xd8dcd6),
        ("puke", 0xa5a502),
        ("pinkish purple", 0xd648d7),
        ("sea blue", 0x047495),
        ("pale purple", 0xb790d4),
        ("slate blue", 0x5b7c99),
        ("blue grey", 0x607c8e),
        ("hunter green", 0x0b4008),
        ("fuchsia", 0xed0dd9),
        ("crimson", 0x8c000f),
        ("pale yellow", 0xffff84),
        ("ochre", 0xbf9005),
        ("mustard yellow", 0xd2bd0a),
        ("light red", 0xff474c),
        ("cerulean", 0x0485d1),
        ("pale pink", 0xffcfdc),
        ("deep blue", 0x040273),
        ("rust", 0xa83c09),
        ("light teal", 0x90e4c1),
        ("slate", 0x516572),
        ("goldenrod", 0xfac205),
        ("dark yellow", 0xd5b60a),
        ("dark grey", 0x363737),
        ("army green", 0x4b5d16),
        ("grey blue", 0x6b8ba4),
        ("seafoam", 0x80f9ad),
        ("puce", 0xa57e52),
        ("spring green", 0xa9f971),
        ("dark orange", 0xc65102),
        ("sand", 0xe2ca76),
        ("pastel green", 0xb0ff9d),
        ("mint", 0x9ffeb0),
        ("light orange", 0xfdaa48),
        ("bright pink", 0xfe01b1),
        ("chartreuse", 0xc1f80a),
        ("deep purple", 0x36013f),
        ("dark brown", 0x341c02),
        ("taupe", 0xb9a281),
        ("pea green", 0x8eab12),
        ("puke green", 0x9aae07),
        ("kelly green", 0x02ab2e),
        ("seafoam green", 0x7af9ab),
        ("blue green", 0x137e6d),
        ("khaki", 0xaaa662),
        ("burgundy", 0x610023),
        ("dark teal", 0x014d4e),
        ("brick red", 0x8f1402),
        ("royal purple", 0x4b006e),
        ("plum", 0x580f41),
        ("mint green", 0x8fff9f),
        ("gold", 0xdbb40c),
        ("baby blue", 0xa2cffe),
        ("yellow green", 0xc0fb2d),
        ("bright purple", 0xbe03fd),
        ("dark red", 0x840000),
        ("pale blue", 0xd0fefe),
        ("grass green", 0x3f9b0b),
        ("navy", 0x01153e),
        ("aquamarine", 0x04d8b2),
        ("burnt orange", 0xc04e01),
        ("neon green", 0x0cff0c),
        ("bright blue", 0x0165fc),
        ("rose", 0xcf6275),
        ("light pink", 0xffd1df),
        ("mustard", 0xceb301),
        ("indigo", 0x380282),
        ("lime", 0xaaff32),
        ("sea green", 0x53fca1),
        ("periwinkle", 0x8e82fe),
        ("dark pink", 0xcb416b),
        ("olive green", 0x677a04),
        ("peach", 0xffb07c),
        ("pale green", 0xc7fdb5),
        ("light brown", 0xad8150),
        ("hot pink", 0xff028d),
        ("black", 0x000000),
        ("lilac", 0xcea2fd),
        ("navy blue", 0x001146),
        ("royal blue", 0x0504aa),
        ("beige", 0xe6daa6),
        ("salmon", 0xff796c),
        ("olive", 0x6e750e),
        ("maroon", 0x650021),
        ("bright green", 0x01ff07),
        ("dark purple", 0x35063e),
        ("mauve", 0xae7181),
        ("forest green", 0x06470c),
        ("aqua", 0x13eac9),
        ("cyan", 0x00ffff),
        ("tan", 0xd1b26f),
        ("dark blue", 0x00035b),
        ("lavender", 0xc79fef),
        ("turquoise", 0x06c2ac),
        ("dark green", 0x033500),
        ("violet", 0x9a0eea),
        ("light purple", 0xbf77f6),
        ("lime green", 0x89fe05),
        ("grey", 0x929591),
        ("sky blue", 0x75bbfd),
        ("yellow", 0xffff14),
        ("magenta", 0xc20078),
        ("light green", 0x96f97b),
        ("orange", 0xf97306),
        ("teal", 0x029386),
        ("light blue", 0x95d0fc),
        ("red", 0xe50000),
        ("brown", 0x653700),
        ("pink", 0xff81c0),
        ("blue", 0x0343df),
        ("green", 0x15b01a),
        ("purple", 0x7e1e9c),
    ];
}
// const TABLEAU : ColorCycle = ColorCycle::tableau();

fn css_color_data() -> Vec<(&'static str, u32)> {
    vec![
        ("aliceblue", 0xf0f8ff),
        ("antiquewhite", 0xfaebd7),
        ("aqua", 0x00ffff),
        ("aquamarine", 0x7fffd4),
        ("azure", 0xf0ffff),
        ("beige", 0xf5f5dc),
        ("bisque", 0xffe4c4),
        ("black", 0x000000),
        ("blanchedalmond", 0xffebcd),
        ("blue", 0x0000ff),
        ("blueviolet", 0x8a2be2),
        ("brown", 0xa52a2a),
        ("burlywood", 0xdeb887),
        ("cadetblue", 0x5f9ea0),
        ("chartreuse", 0x7fff00),
        ("chocolate", 0xd2691e),
        ("coral", 0xff7f50),
        ("cornflowerblue", 0x6495ed),
        ("cornsilk", 0xfff8dc),
        ("crimson", 0xdc143c),
        ("cyan", 0x00ffff),
        ("darkblue", 0x00008b),
        ("darkcyan", 0x008b8b),
        ("darkgoldenrod", 0xb8860b),
        ("darkgray", 0xa9a9a9),
        ("darkgreen", 0x006400),
        ("darkgrey", 0xa9a9a9),
        ("darkkhaki", 0xbdb76b),
        ("darkmagenta", 0x8b008b),
        ("darkolivegreen", 0x556b2f),
        ("darkorange", 0xff8c00),
        ("darkorchid", 0x9932cc),
        ("darkred", 0x8b0000),
        ("darksalmon", 0xe9967a),
        ("darkseagreen", 0x8fbc8f),
        ("darkslateblue", 0x483d8b),
        ("darkslategray", 0x2f4f4f),
        ("darkslategrey", 0x2f4f4f),
        ("darkturquoise", 0x00ced1),
        ("darkviolet", 0x9400d3),
        ("deeppink", 0xff1493),
        ("deepskyblue", 0x00bfff),
        ("dimgray", 0x696969),
        ("dimgrey", 0x696969),
        ("dodgerblue", 0x1e90ff),
        ("firebrick", 0xb22222),
        ("floralwhite", 0xfffaf0),
        ("forestgreen", 0x228b22),
        ("fuchsia", 0xff00ff),
        ("gainsboro", 0xdcdcdc),
        ("ghostwhite", 0xf8f8ff),
        ("gold", 0xffd700),
        ("goldenrod", 0xdaa520),
        ("gray", 0x808080),
        ("green", 0x008000),
        ("greenyellow", 0xadff2f),
        ("grey", 0x808080),
        ("honeydew", 0xf0fff0),
        ("hotpink", 0xff69b4),
        ("indianred", 0xcd5c5c),
        ("indigo", 0x4b0082),
        ("ivory", 0xfffff0),
        ("khaki", 0xf0e68c),
        ("lavender", 0xe6e6fa),
        ("lavenderblush", 0xfff0f5),
        ("lawngreen", 0x7cfc00),
        ("lemonchiffon", 0xfffacd),
        ("lightblue", 0xadd8e6),
        ("lightcoral", 0xf08080),
        ("lightcyan", 0xe0ffff),
        ("lightgoldenrodyellow", 0xfafad2),
        ("lightgray", 0xd3d3d3),
        ("lightgreen", 0x90ee90),
        ("lightgrey", 0xd3d3d3),
        ("lightpink", 0xffb6c1),
        ("lightsalmon", 0xffa07a),
        ("lightseagreen", 0x20b2aa),
        ("lightskyblue", 0x87cefa),
        ("lightslategray", 0x778899),
        ("lightslategrey", 0x778899),
        ("lightsteelblue", 0xb0c4de),
        ("lightyellow", 0xffffe0),
        ("lime", 0x00ff00),
        ("limegreen", 0x32cd32),
        ("linen", 0xfaf0e6),
        ("magenta", 0xff00ff),
        ("maroon", 0x800000),
        ("mediumaquamarine", 0x66cdaa),
        ("mediumblue", 0x0000cd),
        ("mediumorchid", 0xba55d3),
        ("mediumpurple", 0x9370db),
        ("mediumseagreen", 0x3cb371),
        ("mediumslateblue", 0x7b68ee),
        ("mediumspringgreen", 0x00fa9a),
        ("mediumturquoise", 0x48d1cc),
        ("mediumvioletred", 0xc71585),
        ("midnightblue", 0x191970),
        ("mintcream", 0xf5fffa),
        ("mistyrose", 0xffe4e1),
        ("moccasin", 0xffe4b5),
        ("navajowhite", 0xffdead),
        ("navy", 0x000080),
        ("oldlace", 0xfdf5e6),
        ("olive", 0x808000),
        ("olivedrab", 0x6b8e23),
        ("orange", 0xffa500),
        ("orangered", 0xff4500),
        ("orchid", 0xda70d6),
        ("palegoldenrod", 0xeee8aa),
        ("palegreen", 0x98fb98),
        ("paleturquoise", 0xafeeee),
        ("palevioletred", 0xdb7093),
        ("papayawhip", 0xffefd5),
        ("peachpuff", 0xffdab9),
        ("peru", 0xcd853f),
        ("pink", 0xffc0cb),
        ("plum", 0xdda0dd),
        ("powderblue", 0xb0e0e6),
        ("purple", 0x800080),
        ("rebeccapurple", 0x663399),
        ("red", 0xff0000),
        ("rosybrown", 0xbc8f8f),
        ("royalblue", 0x4169e1),
        ("saddlebrown", 0x8b4513),
        ("salmon", 0xfa8072),
        ("sandybrown", 0xf4a460),
        ("seagreen", 0x2e8b57),
        ("seashell", 0xfff5ee),
        ("sienna", 0xa0522d),
        ("silver", 0xc0c0c0),
        ("skyblue", 0x87ceeb),
        ("slateblue", 0x6a5acd),
        ("slategray", 0x708090),
        ("slategrey", 0x708090),
        ("snow", 0xfffafa),
        ("springgreen", 0x00ff7f),
        ("steelblue", 0x4682b4),
        ("tan", 0xd2b48c),
        ("teal", 0x008080),
        ("thistle", 0xd8bfd8),
        ("tomato", 0xff6347),
        ("turquoise", 0x40e0d0),
        ("violet", 0xee82ee),
        ("wheat", 0xf5deb3),
        ("white", 0xffffff),
        ("whitesmoke", 0xf5f5f5),
        ("yellow", 0xffff00),
        ("yellowgreen", 0x9acd32),
    ]
}
