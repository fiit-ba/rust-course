fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let max = input.iter().max();
    let min = input.iter().min();

    println!("{:?} is largest and {:?} is smallest", max, min);
}
