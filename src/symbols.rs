// Symbols
// 🟥🟧🟨🟩🟦🟪🟫⬛⬜
// 🔴🟠🟡🟢🔵🟣🟤⚫⚪

#[derive(Hash, Eq, PartialEq)]
pub enum Symbols {
    RedSquare,
    OrangeSquare,
    YellowSquare,
    GreenSquare,
    BlueSquare,
    PurpleSquare,
    BrownSquare,
    BlackSquare,
    WhiteSquare,
    RedCircle,
    OrangeCircle,
    YellowCircle,
    GreenCircle,
    BlueCircle,
    PurpleCircle,
    BrownCircle,
    BlackCircle,
    WhiteCircle,
}

pub fn get_audio_file(symbol: Symbols) -> &'static str {
    match symbol {
        Symbols::RedSquare => "100hz.wav",
        Symbols::OrangeSquare => "200hz.wav",
        Symbols::YellowSquare => "300hz.wav",
        Symbols::GreenSquare => "400hz.wav",
        Symbols::BlueSquare => "500hz.wav",
        Symbols::PurpleSquare => "600hz.wav",
        Symbols::BrownSquare => "700hz.wav",
        Symbols::WhiteSquare => "800hz.wav",
        Symbols::BlackSquare => "900hz.wav",
        _ => "300hz.wav",
    }
}
