#[derive(Debug, Copy, Clone, PartialEq)]
enum Alignment {
    Left,
    Center,
    Right,
    LanguageDependent,
}

impl From<u8> for Alignment {
    fn from(bits: u8) -> Self {
        match bits {
            0b00 => Alignment::Left,
            0b01 => Alignment::Center,
            0b10 => Alignment::Right,
            0b11 => Alignment::LanguageDependent,
            _ => panic!("alignment bits out of range"),
        }
    }
}

impl From<Alignment> for u8 {
    fn from(alignment: Alignment) -> Self {
        match alignment {
            Alignment::Left => 0b00,
            Alignment::Center => 0b01,
            Alignment::Right => 0b10,
            Alignment::LanguageDependent => 0b11,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum FontSize {
    Normal,
    Large,
    Small,
}

impl From<u8> for FontSize {
    fn from(bits: u8) -> Self {
        match bits {
            0b00 => FontSize::Normal,
            0b01 => FontSize::Large,
            0b10 => FontSize::Small,
            0b11 => panic!("reserved font size"),
            _ => panic!("font size bits out of range"),
        }
    }
}

impl From<FontSize> for u8 {
    fn from(font_size: FontSize) -> Self {
        match font_size {
            FontSize::Normal => 0b00,
            FontSize::Large => 0b01,
            FontSize::Small => 0b10,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    Black,
    DarkGrey,
    DarkRed,
    DarkYellow,
    DarkGreen,
    DarkCyan,
    DarkBlue,
    DarkMagenta,
    Grey,
    White,
    BrightRed,
    BrightYellow,
    BrightGreen,
    BrightCyan,
    BrightBlue,
    BrightMagenta,
}

impl From<u8> for Color {
    fn from(nibble: u8) -> Self {
        match nibble {
            0b0000 => Color::Black,
            0b0001 => Color::DarkGrey,
            0b0010 => Color::DarkRed,
            0b0011 => Color::DarkYellow,
            0b0100 => Color::DarkGreen,
            0b0101 => Color::DarkCyan,
            0b0110 => Color::DarkBlue,
            0b0111 => Color::DarkMagenta,
            0b1000 => Color::Grey,
            0b1001 => Color::White,
            0b1010 => Color::BrightRed,
            0b1011 => Color::BrightYellow,
            0b1100 => Color::BrightGreen,
            0b1101 => Color::BrightCyan,
            0b1110 => Color::BrightBlue,
            0b1111 => Color::BrightMagenta,
            _ => panic!("color nibble out of range")
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
             Color::Black => 0b0000,
             Color::DarkGrey => 0b0001,
             Color::DarkRed => 0b0010,
             Color::DarkYellow => 0b0011,
             Color::DarkGreen => 0b0100,
             Color::DarkCyan => 0b0101,
             Color::DarkBlue => 0b0110,
             Color::DarkMagenta => 0b0111,
             Color::Grey => 0b1000,
             Color::White => 0b1001,
             Color::BrightRed => 0b1010,
             Color::BrightYellow => 0b1011,
             Color::BrightGreen => 0b1100,
             Color::BrightCyan => 0b1101,
             Color::BrightBlue => 0b1110,
             Color::BrightMagenta => 0b1111,
        }
    }
}

type RGB = (u8, u8, u8);

impl From<Color> for RGB {
    fn from(color: Color) -> Self {
        const I0: u8 = 0x00;
        const I0P5: u8 = 0x3f;
        const I1: u8 = 0x7f;
        const I1P5: u8 = 0xbf;
        const I2: u8 = 0xff;
        match color {
             Color::Black => (I0, I0, I0),
             Color::DarkGrey => (I0P5, I0P5, I0P5),
             Color::DarkRed => (I1, I0, I0),
             Color::DarkYellow => (I1, I1, I0),
             Color::DarkGreen => (I0, I1, I0),
             Color::DarkCyan => (I0, I1, I1),
             Color::DarkBlue => (I0, I0, I1),
             Color::DarkMagenta => (I1, I0, I1),
             Color::Grey => (I1P5, I1P5, I1P5),
             Color::White => (I2, I2, I2),
             Color::BrightRed => (I2, I0, I0),
             Color::BrightYellow => (I2, I2, I0),
             Color::BrightGreen => (I0, I2, I0),
             Color::BrightCyan => (I0, I2, I2),
             Color::BrightBlue => (I0, I0, I2),
             Color::BrightMagenta => (I2, I0, I2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Span {
    StartLength {
        start_position: usize,
        length: usize,
    },
    Default,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct TextFormatting {
    span: Span,
    alignment: Alignment,
    font_size: FontSize,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    foreground_color: Color,
    background_color: Color,
}

impl TextFormatting {
    fn from_ied(ied: &[u8]) -> TextFormatting {
        let [start_position, length, mode, color] = match ied {
            [a, b, c, d] => [a, b, c, d],
            _ => panic!("ied must have length 4"),
        };

        let span = if *start_position != 0 {
            let start_position = *start_position as usize;
            let length = *length as usize;
            Span::StartLength { start_position, length }
        } else {
            Span::Default
        };

        let alignment = Alignment::from((mode >> 0) & 0b11);
        let font_size = FontSize::from((mode >> 2) & 0b11);
        let bold = (mode >> 4) & 0b1 == 0b1;
        let italic = (mode >> 5) & 0b1 == 0b1;
        let underline = (mode >> 6) & 0b1 == 0b1;
        let strikethrough = (mode >> 7) & 0b1 == 0b1;
        let foreground_color = Color::from((color >> 0) & 0b1111);
        let background_color = Color::from((color >> 4) & 0b1111);
        
        TextFormatting {
            span,
            alignment,
            font_size,
            bold,
            italic,
            underline,
            strikethrough,
            foreground_color,
            background_color,
        }
    }

    fn into_ied(&self) -> Vec<u8> {
        let (start_position, length) = match self.span {
            Span::StartLength { start_position, length } => (start_position as u8, length as u8),
            Span::Default => (0, 0),
        };

        let mut mode = 0u8;
        mode |= u8::from(self.alignment) << 0;
        mode |= u8::from(self.font_size) << 2;
        mode |= (self.bold as u8) << 4;
        mode |= (self.italic as u8) << 5;
        mode |= (self.underline as u8) << 6;
        mode |= (self.italic as u8) << 7;

        let color = (u8::from(self.background_color) << 4) | u8::from(self.foreground_color);

        vec![start_position, length, mode, color]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_u8() {
        assert_eq!(Alignment::from(0b00), Alignment::Left);
        assert_eq!(u8::from(Alignment::Left), 0b00);
        assert_eq!(Alignment::from(0b01), Alignment::Center);
        assert_eq!(u8::from(Alignment::Center), 0b01);
        assert_eq!(Alignment::from(0b10), Alignment::Right);
        assert_eq!(u8::from(Alignment::Right), 0b10);
        assert_eq!(Alignment::from(0b11), Alignment::LanguageDependent);
        assert_eq!(u8::from(Alignment::LanguageDependent), 0b11);
    }

    #[test]
    fn font_size_u8() {
        assert_eq!(FontSize::from(0b00), FontSize::Normal);
        assert_eq!(u8::from(FontSize::Normal), 0b00);
        assert_eq!(FontSize::from(0b01), FontSize::Large);
        assert_eq!(u8::from(FontSize::Large), 0b01);
        assert_eq!(FontSize::from(0b10), FontSize::Small);
        assert_eq!(u8::from(FontSize::Small), 0b10);
    }

    #[test]
    fn color_u8() {
        assert_eq!(Color::from(0b0000), Color::Black);
        assert_eq!(u8::from(Color::Black), 0b0000u8);
        assert_eq!(Color::from(0b0110), Color::DarkBlue);
        assert_eq!(u8::from(Color::DarkBlue), 0b0110);
        assert_eq!(Color::from(0b1001), Color::White);
        assert_eq!(u8::from(Color::White), 0b1001);
        assert_eq!(Color::from(0b1110), Color::BrightBlue);
        assert_eq!(u8::from(Color::BrightBlue), 0b1110);
    }

    #[test]
    fn color_rgb() {
        assert_eq!(RGB::from(Color::Black), (0x00, 0x00, 0x00));
        assert_eq!(RGB::from(Color::DarkBlue), (0x00, 0x00, 0x7f));
        assert_eq!(RGB::from(Color::White), (0xff, 0xff, 0xff));
        assert_eq!(RGB::from(Color::BrightBlue), (0x00, 0x00, 0xff));
    }

    #[test]
    fn text_formatting_span_start_length() {
        let ied = [1, 5, 0b0000_00_00, 0b1001_0000];
        let tf = TextFormatting {
            span: Span::StartLength {
                start_position: 1,
                length: 5,
            },
            alignment: Alignment::Left,
            font_size: FontSize::Normal,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            foreground_color: Color::Black,
            background_color: Color::White,
        };

        assert_eq!(TextFormatting::from_ied(&ied), tf);
        assert_eq!(tf.into_ied(), ied);
    }

    #[test]
    fn text_formatting_span_default() {
        let ied = [0, 0, 0b0000_00_00, 0b1001_0000];
        let tf = TextFormatting {
            span: Span::Default,
            alignment: Alignment::Left,
            font_size: FontSize::Normal,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            foreground_color: Color::Black,
            background_color: Color::White,
        };

        assert_eq!(TextFormatting::from_ied(&ied), tf);
        assert_eq!(tf.into_ied(), ied);
    }

    #[test]
    fn text_formatting() {
        let ied = [1, 5, 0b1010_01_10, 0b0101_1010];
        let tf = TextFormatting {
            span: Span::StartLength {
                start_position: 1,
                length: 5,
            },
            alignment: Alignment::Right,
            font_size: FontSize::Large,
            bold: false,
            italic: true,
            underline: false,
            strikethrough: true,
            foreground_color: Color::BrightRed,
            background_color: Color::DarkCyan,
        };

        assert_eq!(TextFormatting::from_ied(&ied), tf);
        assert_eq!(tf.into_ied(), ied);
    }
}
