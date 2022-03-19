#[derive(Clone, Copy)]
pub struct Character<'a> {
    pub character_value: char,
    pub character_type: &'a str
}

impl<'a> std::fmt::Display for Character<'a> {
    fn fmt<'b>(&self, formatter: &mut std::fmt::Formatter<'b>) -> std::fmt::Result {
        return write!(formatter, "CHARACTER: {} = \"{}\"", self.character_type, self.character_value);
    }
}

impl<'a> std::fmt::Debug for Character<'a> {
    fn fmt<'b>(&self, formatter: &mut std::fmt::Formatter<'b>) -> std::fmt::Result {
        return write!(formatter, "CHARACTER: {} = \"{}\"", self.character_type, self.character_value);
    }
}