pub fn mean_main(input: Vec<f64>)   {
    let one: f64 = input.iter().sum();
    let mean_answer = one / input.len() as f64;
    println!("==================");
    println!("Mean: {}", mean_answer);
}