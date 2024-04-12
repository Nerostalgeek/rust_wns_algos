use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Ad {
    pub title: String,
    pub price: u32,
    pub tags: Vec<String>,
}

impl Display for Ad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - ${} - Tags: {}", self.title, self.price, self.tags.join(", "))
    }
}
