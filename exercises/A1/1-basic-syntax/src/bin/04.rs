fn main() {
    //  the vec![10, 20, ...].iter() yields immutable references to the elements (&10, &20, ...),
    //  with &n the elements will be pattern matched, so the n will only be the elements
    //  value and not the reference
    for &n in vec![10, 20, 30, 40].iter() {
        let mult = if n < 25 { n * 4 } else { n * 3 };
        println!("{}", mult);
    }
}
