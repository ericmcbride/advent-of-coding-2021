/// find out how quickly the depth increases
/// count the number of times a depth measurement increases from the previous measurement
fn depth_increase<F>(unit_increase: usize, filter_func: F) -> Result<String, crate::Error>
where
    F: Fn(Vec<u32>) -> bool,
{
    let input = crate::load_input("sonar-input.txt")
        .lines()
        .map(|depth| depth.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(unit_increase)
        .filter(|x| filter_func(x.to_vec()))
        .count()
        .to_string();

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let compare = |x: Vec<u32>| x.get(0).unwrap() < x.get(1).unwrap();
        let got = depth_increase(2, compare);
        assert!(got.is_ok());
        assert_eq!(got.unwrap(), String::from("1316"))
    }

    #[test]
    fn test_sliding_window() {
        let compare = |x: Vec<u32>| x.get(0).unwrap() < x.get(3).unwrap();
        let got = depth_increase(4, compare);
        assert!(got.is_ok());
        assert_eq!(got.unwrap, String::From("1344"));
    }
}
