use std::io;

fn catalan_number(n: usize) -> u128 {
    let mut nums: Vec<u128> = vec![0; n];
    nums[0] = 1;
    for i in 0..n {
        for j in 0..i {
            nums[i] += nums[j] * nums[(i-1)-j];
        }
    }
    nums[n - 1]
}

fn main() {
    println!("Enter n: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let text = input.trim();

    let n = text.parse::<usize>()
                .expect(&format!("It was not an integer: {}", text));

    println!("Catalan's number {} equals to {}", n, catalan_number(n));
}
