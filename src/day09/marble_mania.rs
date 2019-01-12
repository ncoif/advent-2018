use crate::common::error::AocError;
use crate::common::response::AocResponse;

// change pointers of elements around, so never move/remove any element
fn play(players: usize, points: usize) -> usize {
    // the index of the vector is the marble number, which contains a tuple (cw, ccw) to link to
    // the marbles on each side
    let mut marbles: Vec<(usize, usize)> = Vec::with_capacity(points);
    marbles.push((0, 0));
    let mut current = 0;
    let mut scores = vec![0; players];

    // 1..=points to use an inclusive range
    for i in 1..=points {
        if i % 23 != 0 {
            // insert a marble between the current and the next element
            current = marbles[current].0;
            let next = marbles[current].0;
            marbles.push((next, current));
            // modify the pointers of current and next to point to this new marble
            marbles[next].1 = i;
            marbles[current].0 = i;
            // this new marble is at index i
            current = i;
        } else {
            // follow pointers for 7 marbles counter-clock-wise
            for _ in 0..7 {
                current = marbles[current].1;
            }

            // remove this marbles by updating the pointers of the marbles pointing to it
            let (cw, ccw) = marbles[current];
            marbles[cw].1 = ccw;
            marbles[ccw].0 = cw;
            // compute the score
            scores[(i % players)] += i + current;
            // because we never remove marbles, and that the index points to the marbles number
            // we insert a fake marble that doesn't point to anything
            current = cw;
            marbles.push((0, 0));
        }
    }

    scores.into_iter().max().unwrap()
}

pub fn answer1() -> Result<AocResponse<usize>, AocError> {
    Ok(AocResponse::new(9, 1, "Marble Mania", play(486, 70833)))
}

pub fn answer2() -> Result<AocResponse<usize>, AocError> {
    Ok(AocResponse::new(
        9,
        2,
        "Marble Mania",
        play(486, 70833 * 100),
    ))
}

#[test]
fn test() {
    assert_eq!(play(9, 25), 32);
}
