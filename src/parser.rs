use crate::symbols::Symbols;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

struct Parser {
    source: String,
}

impl Parser {
    fn parse(&self) -> Vec<Symbols> {
        let iter = self.source.graphemes(true);
        let mut symbols: Vec<Symbols> = Vec::new();

        for x in iter {
            let symbol: Option<Symbols> = None;
            match x {
                "🟥" => Some(Symbols::RedSquare),
                "🟧" => Some(Symbols::OrangeSquare),
                "🟨" => Some(Symbols::YellowSquare),
                "🟩" => Some(Symbols::GreenSquare),
                "🟦" => Some(Symbols::BlueSquare),
                "🟪" => Some(Symbols::PurpleSquare),
                "🟫" => Some(Symbols::BrownSquare),
                "⬛" => Some(Symbols::BlackSquare),
                "⬜" => Some(Symbols::WhiteSquare),
                "🔴" => Some(Symbols::RedCircle),
                "🟠" => Some(Symbols::OrangeCircle),
                "🟡" => Some(Symbols::YellowCircle),
                "🟢" => Some(Symbols::GreenCircle),
                "🔵" => Some(Symbols::BlueCircle),
                "🟣" => Some(Symbols::PurpleCircle),
                "🟤" => Some(Symbols::BrownCircle),
                "⚫" => Some(Symbols::BlackCircle),
                "⚪" => Some(Symbols::WhiteCircle),
                _ => None,
            };

            if let Some(symbol) = symbol {
                symbols.push(symbol);
            } else {
                eprintln!("Unrecognized symbol: {x}!");
                break;
            }
        }

        symbols
    }
}
