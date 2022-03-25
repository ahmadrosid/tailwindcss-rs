use std::collections::HashSet;

use crate::{
    config,
    generator::{write_css, Buffer},
};

#[macro_export]
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
fn test_aspect_ratio() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("aspect-auto", ".aspect-auto {\n\taspect-ratio: auto;\n}"),
        (
            "aspect-square",
            ".aspect-square {\n\taspect-ratio: 1 / 1;\n}",
        ),
        (
            "aspect-video",
            ".aspect-video {\n\taspect-ratio: 16 / 9;\n}",
        ),
    ];

    struct Buf(String);
    impl Buffer for Buf {
        fn write(&mut self, data: &str) {
            assert_eq!(data, self.0)
        }
    }

    for (class, expected) in test_case {
        write_css(Box::new(Buf(expected.into())), &config_set, &set![class]);
    }
}

#[test]
fn test_columns() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("columns-1", ".columns-1 {\n\tcolumns: 0.25rem;\n}"),
        ("columns-2", ".columns-2 {\n\tcolumns: 0.5rem;\n}"),
        ("columns-3", ".columns-3 {\n\tcolumns: 0.75rem;\n}"),
        ("columns-4", ".columns-4 {\n\tcolumns: 1rem;\n}"),
        ("columns-5", ".columns-5 {\n\tcolumns: 1.25rem;\n}"),
    ];

    struct Buf(String);
    impl Buffer for Buf {
        fn write(&mut self, data: &str) {
            assert_eq!(data, self.0)
        }
    }

    for (class, expected) in test_case {
        write_css(Box::new(Buf(expected.into())), &config_set, &set![class]);
    }
}

#[test]
fn test_font_size() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        (
            "text-xs",
            ".text-xs {\n\tfont-size: 0.75rem;\n\tline-height:  1rem;\n}",
        ),
        (
            "text-sm",
            ".text-sm {\n\tfont-size: 0.875rem;\n\tline-height: 1.25rem;\n}",
        ),
        (
            "text-base",
            ".text-base {\n\tfont-size: 1rem;\n\tline-height: 1.5rem;\n}",
        ),
        (
            "text-lg",
            ".text-lg {\n\tfont-size: 1.125rem;\n\tline-height: 1.75rem;\n}",
        ),
        (
            "text-xl",
            ".text-xl {\n\tfont-size: 1.25rem;\n\tline-height: 1.75rem;\n}",
        ),
        (
            "text-2xl",
            ".text-2xl {\n\tfont-size: 1.5rem;\n\tline-height: 2rem;\n}",
        ),
        (
            "text-3xl",
            ".text-3xl {\n\tfont-size: 1.875rem;\n\tline-height: 2.25rem;\n}",
        ),
        (
            "text-4xl",
            ".text-4xl {\n\tfont-size: 2.25rem;\n\tline-height: 2.5rem;\n}",
        ),
        (
            "text-5xl",
            ".text-5xl {\n\tfont-size: 3rem;\n\tline-height: 1;\n}",
        ),
        (
            "text-6xl",
            ".text-6xl {\n\tfont-size: 3.75rem;\n\tline-height: 1;\n}",
        ),
        (
            "text-7xl",
            ".text-7xl {\n\tfont-size: 4.5rem;\n\tline-height: 1;\n}",
        ),
        (
            "text-8xl",
            ".text-8xl {\n\tfont-size: 6rem;\n\tline-height: 1;\n}",
        ),
        (
            "text-9xl",
            ".text-9xl {\n\tfont-size: 8rem;\n\tline-height: 1;\n}",
        ),
    ];

    struct Buf(String);
    impl Buffer for Buf {
        fn write(&mut self, data: &str) {
            assert_eq!(data, self.0)
        }
    }

    for (class, expected) in test_case {
        write_css(Box::new(Buf(expected.into())), &config_set, &set![class]);
    }
}
