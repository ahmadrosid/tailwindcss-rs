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
fn test_break_after() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        (
            "break-after-auto",
            ".break-after-auto {\n\tbreak-after: auto;\n}",
        ),
        (
            "break-after-avoid",
            ".break-after-avoid {\n\tbreak-after: avoid;\n}",
        ),
        (
            "break-after-all",
            ".break-after-all {\n\tbreak-after: all;\n}",
        ),
        (
            "break-after-avoid-page",
            ".break-after-avoid-page {\n\tbreak-after: avoid-page;\n}",
        ),
        (
            "break-after-page",
            ".break-after-page {\n\tbreak-after: page;\n}",
        ),
        (
            "break-after-column",
            ".break-after-column {\n\tbreak-after: column;\n}",
        ),
        (
            "break-after-left",
            ".break-after-left {\n\tbreak-after: left;\n}",
        ),
        (
            "break-after-right",
            ".break-after-right {\n\tbreak-after: right;\n}",
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
fn test_break_before() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        (
            "break-before-auto",
            ".break-before-auto {\n\tbreak-before: auto;\n}",
        ),
        (
            "break-before-avoid",
            ".break-before-avoid {\n\tbreak-before: avoid;\n}",
        ),
        (
            "break-before-all",
            ".break-before-all {\n\tbreak-before: all;\n}",
        ),
        (
            "break-before-avoid-page",
            ".break-before-avoid-page {\n\tbreak-before: avoid-page;\n}",
        ),
        (
            "break-before-page",
            ".break-before-page {\n\tbreak-before: page;\n}",
        ),
        (
            "break-before-column",
            ".break-before-column {\n\tbreak-before: column;\n}",
        ),
        (
            "break-before-left",
            ".break-before-left {\n\tbreak-before: left;\n}",
        ),
        (
            "break-before-right",
            ".break-before-right {\n\tbreak-before: right;\n}",
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
fn test_break_inside() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        (
            "break-inside-auto",
            ".break-inside-auto {\n\tbreak-inside: auto;\n}",
        ),
        (
            "break-inside-avoid",
            ".break-inside-avoid {\n\tbreak-inside: avoid;\n}",
        ),
        (
            "break-inside-all",
            ".break-inside-all {\n\tbreak-inside: all;\n}",
        ),
        (
            "break-inside-avoid-page",
            ".break-inside-avoid-page {\n\tbreak-inside: avoid-page;\n}",
        ),
        (
            "break-inside-page",
            ".break-inside-page {\n\tbreak-inside: page;\n}",
        ),
        (
            "break-inside-column",
            ".break-inside-column {\n\tbreak-inside: column;\n}",
        ),
        (
            "break-inside-left",
            ".break-inside-left {\n\tbreak-inside: left;\n}",
        ),
        (
            "break-inside-right",
            ".break-inside-right {\n\tbreak-inside: right;\n}",
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
fn test_flex_basis() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("basis-0", ".basis-0 {\n\tflex-basis: 0px;\n}"),
        ("basis-1", ".basis-1 {\n\tflex-basis: 0.25rem;\n}"),
        ("basis-2", ".basis-2 {\n\tflex-basis: 0.5rem;\n}"),
        ("basis-3", ".basis-3 {\n\tflex-basis: 0.75rem;\n}"),
        ("basis-4", ".basis-4 {\n\tflex-basis: 1rem;\n}"),
        ("basis-5", ".basis-5 {\n\tflex-basis: 1.25rem;\n}"),
        ("basis-6", ".basis-6 {\n\tflex-basis: 1.5rem;\n}"),
        ("basis-7", ".basis-7 {\n\tflex-basis: 1.75rem;\n}"),
        ("basis-8", ".basis-8 {\n\tflex-basis: 2rem;\n}"),
        ("basis-9", ".basis-9 {\n\tflex-basis: 2.25rem;\n}"),
        ("basis-10", ".basis-10 {\n\tflex-basis: 2.5rem;\n}"),
        ("basis-11", ".basis-11 {\n\tflex-basis: 2.75rem;\n}"),
        ("basis-12", ".basis-12 {\n\tflex-basis: 3rem;\n}"),
        ("basis-14", ".basis-14 {\n\tflex-basis: 3.5rem;\n}"),
        ("basis-16", ".basis-16 {\n\tflex-basis: 4rem;\n}"),
        ("basis-20", ".basis-20 {\n\tflex-basis: 5rem;\n}"),
        ("basis-24", ".basis-24 {\n\tflex-basis: 6rem;\n}"),
        ("basis-28", ".basis-28 {\n\tflex-basis: 7rem;\n}"),
        ("basis-32", ".basis-32 {\n\tflex-basis: 8rem;\n}"),
        ("basis-36", ".basis-36 {\n\tflex-basis: 9rem;\n}"),
        ("basis-40", ".basis-40 {\n\tflex-basis: 10rem;\n}"),
        ("basis-44", ".basis-44 {\n\tflex-basis: 11rem;\n}"),
        ("basis-48", ".basis-48 {\n\tflex-basis: 12rem;\n}"),
        ("basis-52", ".basis-52 {\n\tflex-basis: 13rem;\n}"),
        ("basis-56", ".basis-56 {\n\tflex-basis: 14rem;\n}"),
        ("basis-60", ".basis-60 {\n\tflex-basis: 15rem;\n}"),
        ("basis-64", ".basis-64 {\n\tflex-basis: 16rem;\n}"),
        ("basis-72", ".basis-72 {\n\tflex-basis: 18rem;\n}"),
        ("basis-80", ".basis-80 {\n\tflex-basis: 20rem;\n}"),
        ("basis-96", ".basis-96 {\n\tflex-basis: 24rem;\n}"),
        ("basis-auto", ".basis-auto {\n\tflex-basis: auto;\n}"),
        // Not all spacing variant need to be tested, this sample is enough to verify.
        ("basis-px", ".basis-px {\n\tflex-basis: 1px;\n}"),
        ("basis-0.5", ".basis-0\\.5 {\n\tflex-basis: 0.125rem;\n}"),
        ("basis-1.5", ".basis-1\\.5 {\n\tflex-basis: 0.375rem;\n}"),
        ("basis-2.5", ".basis-2\\.5 {\n\tflex-basis: 0.625rem;\n}"),
        ("basis-3.5", ".basis-3\\.5 {\n\tflex-basis: 0.875rem;\n}"),
        ("basis-1/2", ".basis-1\\/2 {\n\tflex-basis: 50%;\n}"),
        ("basis-1/3", ".basis-1\\/3 {\n\tflex-basis: 33.333333%;\n}"),
        ("basis-2/3", ".basis-2\\/3 {\n\tflex-basis: 66.666667%;\n}"),
        ("basis-full", ".basis-full {\n\tflex-basis: 100%;\n}"),
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
fn test_flex_direction() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("flex-row", ".flex-row {\n\tflex-direction: row;\n}"),
        (
            "flex-row-reverse",
            ".flex-row-reverse {\n\tflex-direction: row-reverse;\n}",
        ),
        ("flex-col", ".flex-col {\n\tflex-direction: column;\n}"),
        (
            "flex-col-reverse",
            ".flex-col-reverse {\n\tflex-direction: column-reverse;\n}",
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
fn test_flex_wrap() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("flex-wrap", ".flex-wrap {\n\tflex-wrap: wrap;\n}"),
        (
            "flex-wrap-reverse",
            ".flex-wrap-reverse {\n\tflex-wrap: wrap-reverse;\n}",
        ),
        ("flex-nowrap", ".flex-nowrap {\n\tflex-wrap: nowrap;\n}"),
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
fn test_flex() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("flex-1", ".flex-1 {\n\tflex: 1 1 0%;\n}"),
        ("flex-auto", ".flex-auto {\n\tflex: 1 1 auto;\n}"),
        ("flex-initial", ".flex-initial {\n\tflex: 0 1 auto;\n}"),
        ("flex-none", ".flex-none {\n\tflex: none;\n}"),
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
fn test_flex_grow() {
    let config_set = config::parse(include_str!("default-config.json")).unwrap();
    let test_case = vec![
        ("grow", ".flex-1 {\n\tflex: 1 1 0%;\n}"),
        ("grow-0", ".flex-auto {\n\tflex: 1 1 auto;\n}"),
    ];

    struct Buf<>(String);
    impl Buffer for Buf {
        fn write(&mut self, data: &str) {
            assert_eq!(data, self.0);
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
