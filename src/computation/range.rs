pub fn range_main(mut input: Vec<f64>) {
    input.retain(|x| !x.is_nan());
    input.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let smallest = input[0];
    let greatest = input[input.len() - 1];
    println!("The range is: {}", greatest - smallest);
}