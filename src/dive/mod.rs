use std::str::FromStr;

#[derive(Debug)]
enum DiveDirection {
    Up,
    Down,
    Forward,
}

impl FromStr for DiveDirection {
    type Err = ();
    fn from_str(input: &str) -> Result<DiveDirection, Self::Err> {
        match input {
            "up" => Ok(DiveDirection::Up),
            "down" => Ok(DiveDirection::Down),
            "forward" => Ok(DiveDirection::Forward),
            _ => Err(()),
        }
    }
}

fn horizontal_and_depth() -> Result<String, crate::Error> {
    let input = crate::load_input("horizontal_depth.txt");
    let mut depth: u32 = Default::default();
    let mut horizontal: u32 = Default::default();

    for i in input.lines() {
        let line_split = i.split(' ').collect::<Vec<&str>>();
        let spaces_moved = line_split.get(1).unwrap().parse::<u32>().unwrap();
        let direction = DiveDirection::from_str(line_split.get(0).unwrap()).unwrap();
        match direction {
            DiveDirection::Up => depth -= spaces_moved,
            DiveDirection::Down => depth += spaces_moved,
            DiveDirection::Forward => horizontal += spaces_moved,
        }
    }

    let total = depth * horizontal;
    Ok(total.to_string())
}

fn horizontal_and_depth_pt_2() -> Result<String, crate::Error> {
    let input = crate::load_input("horizontal_depth.txt");
    let mut depth: u32 = Default::default();
    let mut horizontal: u32 = Default::default();
    let mut aim: u32 = Default::default();

    for i in input.lines() {
        let line_split = i.split(' ').collect::<Vec<&str>>();
        let spaces_moved = line_split.get(1).unwrap().parse::<u32>().unwrap();
        let direction = DiveDirection::from_str(line_split.get(0).unwrap()).unwrap();
        match direction {
            DiveDirection::Up => aim -= spaces_moved,
            DiveDirection::Down => aim += spaces_moved,
            DiveDirection::Forward => {
                horizontal += spaces_moved;
                depth += aim * spaces_moved;
            }
        }
    }

    let total = depth * horizontal;
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let got = horizontal_and_depth();
        assert!(got.is_ok());
        assert_eq!(got.unwrap(), String::from("1936494"));
    }

    #[test]
    fn test_part_two() {
        let got = horizontal_and_depth_pt_2();
        assert!(got.is_ok());
        assert_eq!(got.unwrap(), String::from("1997106066"));
    }
}
