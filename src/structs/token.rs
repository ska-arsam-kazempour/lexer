#[derive(Clone, Copy)]
pub struct Token<'a> {
    pub token_value: &'a str,
    pub token_type: &'a str
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt<'b>(&self, formatter: &mut std::fmt::Formatter<'b>) -> std::fmt::Result {
        return write!(formatter, "TOKEN: {} = \"{}\"", self.token_type, self.token_value);
    }
}

impl<'a> std::fmt::Debug for Token<'a> {
    fn fmt<'b>(&self, formatter: &mut std::fmt::Formatter<'b>) -> std::fmt::Result {
        return write!(formatter, "TOKEN: {} = \"{}\"", self.token_type, self.token_value);
    }
}