/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> String {
    if i % 3 == 0 {
        if i % 5 == 0 {
            "FizzBuzz".to_owned()
        } else {
            "Fizz".to_owned()
        }
    } else if i % 5 == 0 {
        "Buzz".to_owned()
    } else {
        format!("{i}")
    }
}

#[test]
fn test_fizz_buzz() {
    let expected: Vec<String> = include_str!("../fizzbuzz.out")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    for i in 1..1001 {
        let actual = fizz_buzz(i);
        let exp = expected.get((i - 1) as usize).unwrap().to_owned();

        assert_eq!(actual, exp);
    }
}
