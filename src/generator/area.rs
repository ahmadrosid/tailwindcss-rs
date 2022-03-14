enum Area {
    All,
    Left,
    Right,
    Top,
    Bottom,
    Vertical,
    Horizontal,
    None
}

fn get_property(name: &str) -> Option<&'static str> {
    match &name[0..1] {
        "p" => Some("padding"),
        "m" => Some("margin"),
        _ => None
    }
}

fn get_area(name: &str) -> Area {
    if name.len() == 1 {
        return Area::All;
    }

    match &name[1..] {
        "t" => Area::Top,
        "b" => Area::Bottom,
        "l" => Area::Left,
        "r" => Area::Right,
        "x" => Area::Horizontal,
        "y" => Area::Vertical,
        _ => Area::None
    }
}

pub fn generate_variant(name: &str, variant: &str, value: &str) -> Option<String> {
    let property = get_property(name)?;
    return match get_area(name) {
        Area::All => {
            Some(
                format!(".{} {{\n\t{}: {};\n}}", name, property, value)
            )
        }
        Area::Top => {
            Some(
                format!(".{}-{} {{\n\t{}-top: {};\n}}", name, variant, property, value)
            )
        }
        Area::Bottom => {
            Some(
                format!(".{}-{} {{\n\t{}-bottom: {};\n}}", name, variant, property, value)
            )
        }
        Area::Left => {
            Some(
                format!(".{}-{} {{\n\t{}-left: {};\n}}", name, variant, property, value)
            )
        }
        Area::Right => {
            Some(
                format!(".{}-{} {{\n\t{}-right: {};\n}}", name, variant, property, value)
            )
        }
        Area::Vertical => {
            let body = format!("{property}-top: {value};\n\t{property}-bottom: {value};", property=property, value=value);
            Some(
                format!(".{}-{} {{\n\t{}\n}}", name, variant, body)
            )
        }
        Area::Horizontal => {
            let body = format!("{property}-left: {value};\n\t{property}-right: {value};", property=property, value=value);
            Some(
                format!(".{}-{} {{\n\t{}\n}}", name, variant, body)
            )
        }
        Area::None => None
    };
}
