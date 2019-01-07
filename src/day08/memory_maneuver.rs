use crate::common::error::AocError;
use crate::common::response::AocResponse;

struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn parse(it: &mut impl Iterator<Item = i32>) -> Node {
        let children_count = it.next().unwrap();
        let metadata_count = it.next().unwrap();

        let children = (0..children_count).map(|_| Node::parse(it)).collect();
        let metadata = it.take(metadata_count as usize).collect();

        Node { children, metadata }
    }

    fn metadata_count(&self) -> i32 {
        self.metadata.iter().sum::<i32>()
            + self
                .children
                .iter()
                .map(|c| c.metadata_count())
                .sum::<i32>()
    }

    fn answer2_count(&self) -> i32 {
        if self.children.is_empty() {
            self.metadata.iter().sum::<i32>()
        } else {
            self.metadata
                .iter()
                .map(|&n| {
                    if n <= self.children.len() as i32 {
                        self.children[(n - 1) as usize].answer2_count()
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        }
    }
}

fn read_file(filename: &str) -> Result<Vec<i32>, AocError> {
    let s = std::fs::read_to_string(filename)?;

    let mut nodes = vec![];
    for e in s.split_whitespace() {
        let node = e.parse::<i32>()?;
        nodes.push(node);
    }

    Ok(nodes)
}

pub fn answer1() -> Result<AocResponse<i32>, AocError> {
    //let nodes = read_file("input/input8_debug.txt".to_string());
    let nodes = read_file("input/input8.txt")?;

    let answer1 = Node::parse(&mut nodes.into_iter());
    Ok(AocResponse::new(
        8,
        1,
        "Memory Maneuver",
        answer1.metadata_count(),
    ))
}

pub fn answer2() -> Result<AocResponse<i32>, AocError> {
    //let nodes = read_file("input/input8_debug.txt".to_string());
    let nodes = read_file("input/input8.txt")?;

    let answer2 = Node::parse(&mut nodes.into_iter());
    Ok(AocResponse::new(
        8,
        2,
        "Memory Maneuver",
        answer2.answer2_count(),
    ))
}
