use std::fmt;

#[derive(Debug)]
pub struct AocResponse<T> {
    day: isize,
    part: isize,
    description: String,
    answer: T,
}

impl fmt::Display for AocResponse<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Day {:02}: {} ({}/2): {}",
            self.day, self.description, self.part, self.answer
        )
    }
}

impl AocResponse<i32> {
    pub fn new(day: isize, part: isize, description: &str, answer: i32) -> Self {
        AocResponse {
            day,
            part,
            description: description.to_string(),
            answer,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self) -> i32 {
        self.answer
    }
}
