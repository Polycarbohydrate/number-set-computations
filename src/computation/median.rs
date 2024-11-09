pub fn median_main(mut input: Vec<f64>)    {
    input.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = input.len();
    if len % 2 == 1 {
        input[len / 2];
    } else {
        let mid1 = input[len / 2 - 1];
        let mid2 = input[len / 2];
        let median = (mid1 + mid2) / 2.0;
        println!("==================");
        println!("Median: {}", median);
    }
}