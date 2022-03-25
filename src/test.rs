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
        ("columns-1", ".columns-1 {\n\tcolumns: 1;\n}"),
        ("columns-2", ".columns-2 {\n\tcolumns: 2;\n}"),
        ("columns-3", ".columns-3 {\n\tcolumns: 3;\n}"),
        ("columns-4", ".columns-4 {\n\tcolumns: 4;\n}"),
        ("columns-5", ".columns-5 {\n\tcolumns: 5;\n}"),
        ("columns-6", ".columns-6 {\n\tcolumns: 6;\n}"),
        ("columns-7", ".columns-7 {\n\tcolumns: 7;\n}"),
        ("columns-8", ".columns-8 {\n\tcolumns: 8;\n}"),
        ("columns-9", ".columns-9 {\n\tcolumns: 9;\n}"),
        ("columns-10", ".columns-10 {\n\tcolumns: 10;\n}"),
        ("columns-11", ".columns-11 {\n\tcolumns: 11;\n}"),
        ("columns-12", ".columns-12 {\n\tcolumns: 12;\n}"),
        ("columns-auto", ".columns-auto {\n\tcolumns: auto;\n}"),
        ("columns-2xs", ".columns-2xs {\n\tcolumns: 18rem;\n}"),
        ("columns-3xs", ".columns-3xs {\n\tcolumns: 16rem;\n}"),
        ("columns-xs", ".columns-xs {\n\tcolumns: 20rem;\n}"),
        ("columns-sm", ".columns-sm {\n\tcolumns: 24rem;\n}"),
        ("columns-md", ".columns-md {\n\tcolumns: 28rem;\n}"),
        ("columns-lg", ".columns-lg {\n\tcolumns: 32rem;\n}"),
        ("columns-xl", ".columns-xl {\n\tcolumns: 36rem;\n}"),
        ("columns-2xl", ".columns-2xl {\n\tcolumns: 42rem;\n}"),
        ("columns-3xl", ".columns-3xl {\n\tcolumns: 48rem;\n}"),
        ("columns-4xl", ".columns-4xl {\n\tcolumns: 56rem;\n}"),
        ("columns-5xl", ".columns-5xl {\n\tcolumns: 64rem;\n}"),
        ("columns-6xl", ".columns-6xl {\n\tcolumns: 72rem;\n}"),
        ("columns-7xl", ".columns-7xl {\n\tcolumns: 80rem;\n}"),
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
