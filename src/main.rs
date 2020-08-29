use ems::{*, text_formatting::*};

fn print_message(s: &str, tfs: &[TextFormatting]) {
    println!("<!DOCTYPE html><html><body>");
    println!("<div>Formatted EMS message:</div>");
    println!("<div style=\"font-family: monospace; font-size: 200%; margin: 20px 0 20px 0;\">{}</div>", format(s, &tfs));
    println!("<div>Original message: {:?}</div>", s);
    println!("<div>Text formatting IEs:<br><ul style=\"font-size: 75%;\">");
    for tf in tfs {
        println!("<li>{:?}</li>", tf);
    }
    println!("</ul></div>");
    println!("</body></html>");
}

fn main() {
    let message = "!trans  rights!\nbold italic\nunderline strikethrough\nbig smol";
    let blue = TextFormatting {
        foreground_color: Color::White,
        background_color: Color::BrightBlue,
        ..TextFormatting::default()
    };
    let pink = TextFormatting {
        foreground_color: Color::White,
        background_color: Color::BrightMagenta,
        ..TextFormatting::default()
    };

    let tfs = [
        TextFormatting {
            span: Span::StartLength {
                start_position: 0,
                length: 3,
            },
            ..blue
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 3,
                length: 3,
            },
            ..pink
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 9,
                length: 3,
            },
            ..pink
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 12,
                length: 3,
            },
            ..blue
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 16,
                length: 4,
            },
            bold: true,
            ..TextFormatting::default()
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 21,
                length: 6,
            },
            italic: true,
            ..TextFormatting::default()
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 28,
                length: 9,
            },
            underline: true,
            ..TextFormatting::default()
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 38,
                length: 13,
            },
            strikethrough: true,
            ..TextFormatting::default()
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 52,
                length: 3,
            },
            font_size: FontSize::Large,
            ..TextFormatting::default()
        },
        TextFormatting {
            span: Span::StartLength {
                start_position: 56,
                length: 4,
            },
            font_size: FontSize::Small,
            ..TextFormatting::default()
        },
    ];
    print_message(message, &tfs);
}
