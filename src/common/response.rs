use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct AocResponse<T: Display> {
    day: isize,
    part: isize,
    description: String,
    answer: T,
}

impl<T: Display> Display for AocResponse<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Day {:02}: {} ({}/2): {}",
            self.day, self.description, self.part, self.answer
        )
    }
}

impl<T: Display + Clone> AocResponse<T> {
    pub fn new(day: isize, part: isize, description: &str, answer: T) -> Self {
        AocResponse {
            day,
            part,
            description: description.to_string(),
            answer,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self) -> T {
        self.answer.clone()
    }
}
