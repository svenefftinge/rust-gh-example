fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn print_sum(a: u32, b: u32) {
    println!("print_sum says {}", a + b);
}

fn main() {
    let sum = add(1, 2);
    println!("the sum is {}", sum);

    print_sum(5, 6);

    let zzzzz = rust_gh_example::answer();
    println!("Answer is {}", zzzzz);
}
