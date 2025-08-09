use crate::symbols::Symbols;
use unicode_segmentation::UnicodeSegmentation;

pub struct Parser {
    pub source: String,
}

impl Parser {
    pub fn parse(&self) -> Vec<Symbols> {
        let iter = self.source.graphemes(true);
        let mut symbols: Vec<Symbols> = Vec::new();

        for x in iter {
            println!("{}", x);
            let symbol: Option<Symbols> = if x == "🟥" {
                Some(Symbols::RedSquare)
            } else if x == "🟧" {
                Some(Symbols::OrangeSquare)
            } else if x == "🟨" {
                Some(Symbols::YellowSquare)
            } else if x == "🟩" {
                Some(Symbols::GreenSquare)
            } else if x == "🟦" {
                Some(Symbols::BlueSquare)
            } else if x == "🟪" {
                Some(Symbols::PurpleSquare)
            } else if x == "🟫" {
                Some(Symbols::BrownSquare)
            } else if x == "⬛" {
                Some(Symbols::BlackSquare)
            } else if x == "⬜" {
                Some(Symbols::WhiteSquare)
            } else if x == "🔴" {
                Some(Symbols::RedCircle)
            } else if x == "🟠" {
                Some(Symbols::OrangeCircle)
            } else if x == "🟡" {
                Some(Symbols::YellowCircle)
            } else if x == "🟢" {
                Some(Symbols::GreenCircle)
            } else if x == "🔵" {
                Some(Symbols::BlueCircle)
            } else if x == "🟣" {
                Some(Symbols::PurpleCircle)
            } else if x == "🟤" {
                Some(Symbols::BrownCircle)
            } else if x == "⚫" {
                Some(Symbols::BlackCircle)
            } else if x == "⚪" {
                Some(Symbols::WhiteCircle)
            } else {
                None
            };

            if let Some(symbol) = symbol {
                symbols.push(symbol);
            }
        }

        symbols
    }
}
