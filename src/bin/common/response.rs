use std::fmt;

//TODO: AdventOfCodeResponse<T> for different types of response
#[derive(Debug)]
pub struct AdventOfCodeResponse {
    day: isize,
    part: isize,
    description: String,
    answer: String,
}

impl fmt::Display for AdventOfCodeResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Day {:02}: {} ({}/2): {}",
            self.day, self.description, self.part, self.answer
        )
    }
}

impl AdventOfCodeResponse {
    pub fn new(day: isize, part: isize, description: &str, answer: &str) -> Self {
        AdventOfCodeResponse {
            day,
            part,
            description: description.to_string(),
            answer: answer.to_string(),
        }
    }
}
