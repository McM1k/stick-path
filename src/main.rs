use std::{fmt::Error, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    let mut diagram = Vec::new();
    for i in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_end().to_string();
        diagram.push(line);
    }

    let answer = solve(w, h, diagram);
    println!("answer: ");
    for tuple in answer.unwrap() {
        println!("  {}{}", tuple.0, tuple.1)
    }
}

fn solve(width: i32, height: i32, input: Vec<String>) -> Result<Vec<(char, char)>, Error> {
    Ok(vec![('A', 'A')])
}

#[cfg(test)]
mod main_tests {
    use crate::solve;

    #[test]
    fn example_1() {
        let w = 7;
        let h = 7;
        let diagram = vec![
            "A  B  C".to_string(),
            "|  |  |".to_string(),
            "|--|  |".to_string(),
            "|  |--|".to_string(),
            "|  |--|".to_string(),
            "|  |  |".to_string(),
            "1  2  3".to_string(),
        ];
        let answer = solve(w, h, diagram);
        assert_eq!(answer.unwrap(), vec![
            ('A', '2'),
            ('B', '1'),
            ('C', '3'),
            ]);
    }

    #[test]
    fn example_2() {
        let w = 18;
        let h = 22;
        let diagram = vec![
            "P  Q  R  S  T  U  V  W".to_string(),
            "|  |  |  |  |--|  |  |".to_string(),
            "|  |  |--|  |  |  |--|".to_string(),
            "|  |--|  |--|  |  |  |".to_string(),
            "|--|  |--|  |  |  |--|".to_string(),
            "|--|  |  |  |  |--|  |".to_string(),
            "|  |--|  |  |--|  |--|".to_string(),
            "|  |  |  |--|  |--|  |".to_string(),
            "|--|  |  |  |--|  |  |".to_string(),
            "|  |  |--|  |  |  |  |".to_string(),
            "|  |  |  |--|  |  |--|".to_string(),
            "|  |  |  |  |--|  |  |".to_string(),
            "|--|  |  |  |  |  |  |".to_string(),
            "|--|  |--|  |  |  |--|".to_string(),
            "|  |--|  |  |--|  |  |".to_string(),
            "|  |  |--|  |  |  |--|".to_string(),
            "|--|  |--|  |  |--|  |".to_string(),
            "1  2  3  4  5  6  7  8".to_string(),
        ];
        let answer = solve(w, h, diagram);
        assert_eq!(answer.unwrap(), vec![
            ('P', '3'),
            ('Q', '7'),
            ('R', '8'),
            ('S', '5'),
            ('T', '6'),
            ('U', '2'),
            ('V', '4'),
            ('W', '1'),
            ]);
    }
}