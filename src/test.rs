use std::collections::HashSet;

use crate::{generator::{write_css, Buffer}, config};

macro_rules! set {
    ($($element:expr),*) => {
        {
            let mut v = HashSet::new();
            $(v.insert(format!("{}", $element));)*
            v
        }
    };
}

#[test]
fn test_font_size() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("text-xs", ".text-xs {\n\tfont-size: 0.75rem;\n\tline-height:  1rem;\n}"),
        ("text-sm", ".text-sm {\n\tfont-size: 0.875rem;\n\tline-height: 1.25rem;\n}"),
        ("text-base", ".text-base {\n\tfont-size: 1rem;\n\tline-height: 1.5rem;\n}"),
        ("text-lg", ".text-lg {\n\tfont-size: 1.125rem;\n\tline-height: 1.75rem;\n}"),
        ("text-xl", ".text-xl {\n\tfont-size: 1.25rem;\n\tline-height: 1.75rem;\n}"),
    ];
    for (class, expected) in test_case {
        impl Buffer for &str {
            fn write(&mut self, data: &str) {
                assert_eq!(data, *self)
            }
        }
        write_css(Box::new(expected), &config_set, &set![class]);
    }
}
