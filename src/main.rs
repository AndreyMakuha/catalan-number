use std::io;

fn catalan_number_recursively(n: usize) -> u64 {
    let mut sum: u64 = 0;
    if n < 1 {
        return 1;
    }
    for i in 0..n {
        sum += catalan_number_recursively(i) * catalan_number_recursively((n-1)-i);
    }
    sum
}

fn catalan_number(n: usize) -> u64 {
    let mut nums : Vec<u64> = vec![0; n];
    nums[0] = 1;
    for i in 0..n {
        for j in 0..i {
            nums[i] += nums[j] * nums[(i-1)-j];
        }
    }
    nums[n-1]
}

#[allow(unused_assignments)]
fn main() {
    let mut input_text = String::new();
    let mut n : usize = 0;
    println!("Enter n: ");
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => n = i,
        Err(..) => { println!("this was not an integer: {}", trimmed); return; }
    };
    println!("Catalan's number {} equals to {}", n, catalan_number(n));
    println!("By recursion {}", catalan_number_recursively(n-1));
}
