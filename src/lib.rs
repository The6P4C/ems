pub mod text_formatting;

use text_formatting::*;

fn color_to_hex(color: Color) -> String {
    let rgb: RGB = color.into();
    format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2)
}

pub fn format(s: &str, tfs: &[TextFormatting]) -> String {
    let default_tf = TextFormatting::default();

    let mut formatted = String::new();
    for (position, c) in s.chars().enumerate() {
        let tf = tfs
            .iter()
            .filter(|tf| tf.span.contains(position))
            .last()
            .unwrap_or(&default_tf);

        let mut style = String::new();

        if tf.bold {
            style.push_str("font-weight: bold;");
        }

        if tf.italic {
            style.push_str("font-style: italic;");
        }

        if tf.underline || tf.strikethrough {
            style.push_str(&format!("text-decoration:{}{};", if tf.underline { " underline" } else { "" }, if tf.strikethrough { " line-through" } else { "" }));
        }

        style.push_str(&format!("font-size: {};", match tf.font_size {
            FontSize::Normal => "100%",
            FontSize::Large => "150%",
            FontSize::Small => "50%",
        }));

        style.push_str(&format!("color: {};", color_to_hex(tf.foreground_color)));
        style.push_str(&format!("background-color: {};", color_to_hex(tf.background_color)));

        let c = match c {
            ' ' => "&nbsp".to_string(),
            '\n' => "<br>".to_string(),
            _ => format!("{}", c),
        };
        formatted.push_str(&format!("<span style=\"{}\">{}</span>", style, c));
    }

    formatted
}
