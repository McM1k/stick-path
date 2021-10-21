use log::debug;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    env_logger::init();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let _w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    let mut diagram = Vec::new();
    for _i in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_end().to_string();
        diagram.push(line);
    }

    let answer = solve(&diagram);
    for tuple in answer {
        println!("{}{}", tuple.0, tuple.1)
    }
}

fn solve(input: &Vec<String>) -> Vec<(char, char)> {
    let mut answers: Vec<(char, char)> = Vec::new();
    let (top, splitted_input) = input.split_first().unwrap();
    let (bottom, diagram) = splitted_input.split_last().unwrap();

    for (mut x, t) in top.chars().enumerate() {
        if !t.is_ascii_whitespace() {
            debug!("starting from {} \n", t);
            for line in diagram {
                if x > 0 && line.chars().nth(x - 1).unwrap() == '-' {
                    x -= 3;
                    debug!("left, ");
                } else if x < top.len() - 1 && line.chars().nth(x + 1).unwrap() == '-' {
                    x += 3;
                    debug!("right, ");
                } else {
                    debug!("straight, ");
                }
            }
            debug!("arriving at {} \n", bottom.chars().nth(x).unwrap());
            answers.push((t, bottom.chars().nth(x).unwrap()));
        }
    }
    answers
}

#[cfg(test)]
mod main_tests {
    use crate::solve;

    #[test]
    fn example_1() {
        let diagram = vec![
            "A  B  C".to_string(),
            "|  |  |".to_string(),
            "|--|  |".to_string(),
            "|  |--|".to_string(),
            "|  |--|".to_string(),
            "|  |  |".to_string(),
            "1  2  3".to_string(),
        ];
        let answer = solve(&diagram);
        assert_eq!(answer, vec![('A', '2'), ('B', '1'), ('C', '3'),]);
    }

    #[test]
    fn example_2() {
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
        let answer = solve(&diagram);
        assert_eq!(
            answer,
            vec![
                ('P', '3'),
                ('Q', '7'),
                ('R', '8'),
                ('S', '5'),
                ('T', '6'),
                ('U', '2'),
                ('V', '4'),
                ('W', '1'),
            ]
        );
    }
}
