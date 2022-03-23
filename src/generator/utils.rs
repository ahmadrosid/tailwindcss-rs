pub trait EscapeClassName {
    fn escape_class_name(&self) -> String;
}

impl EscapeClassName for str {
    fn escape_class_name(&self) -> String {
        self.replace(".", "\\.").replace("/", "\\/")
    }
}

impl EscapeClassName for String {
    fn escape_class_name(&self) -> String {
        self.replace(".", "\\.").replace("/", "\\/")
    }
}
