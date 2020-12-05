use super::BoardingPass;

pub fn parse(content: &[u8]) -> Vec<BoardingPass> {
    content.split(|b| *b == 10).filter(|pass| pass.len() > 0).map(|pass| {
        process_boarding(pass)
    }).collect::<Vec<BoardingPass>>()
}

fn process_boarding(input: &[u8]) -> BoardingPass {
    let mut row: Vec<usize> = (0..=127).collect();
    let mut column: Vec<usize> = (0..=7).collect();
    for b in input.iter() {
        if *b == 70 {
            row = take(row, Side::Bottom);
        } else if *b == 66 {
            row = take(row, Side::Top);
        } else if *b == 76 {
            column = take(column, Side::Bottom);
        } else if *b == 82 {
            column = take(column, Side::Top);
        }
    }
    BoardingPass {
        row: row[0],
        column: column[0],
        seat_id: row[0] as u32 * 8 + column[0] as u32
    }
}

enum Side {
    Top, Bottom
}

fn take(input: Vec<usize>, side: Side) -> Vec<usize> {
    let middle = input.len()/2;
    match side {
        Side::Top => input.split_at(middle).1.to_vec(),
        Side::Bottom => input.split_at(middle).0.to_vec()   
    }
}

#[test]
pub fn parse_test() {
    use super::TEST;
    let passes = parse(TEST);
    assert_eq!(4, passes.len());

    assert_eq!(44, passes[0].row);
    assert_eq!(5, passes[0].column);
    assert_eq!(357, passes[0].seat_id);

    assert_eq!(102, passes[3].row);
    assert_eq!(4, passes[3].column);
    assert_eq!(820, passes[3].seat_id);
}
