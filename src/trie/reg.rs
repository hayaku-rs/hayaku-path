use regex;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Regex(pub regex::Regex);

impl Regex {
    pub fn new(re: &str) -> Self {
        let regex = regex::Regex::new(re).unwrap();
        Regex(regex)
    }
}

impl Deref for Regex {
    type Target = regex::Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Regex) -> bool {
        self.as_str() == other.as_str()
    }
}
