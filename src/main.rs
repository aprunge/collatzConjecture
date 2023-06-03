fn collatz(mut num: i32) -> u32 {
    let mut steps: u32 = 0;
        while num > 1{  
        if num % 2 == 0 {
            num = num / 2;
            steps += 1;
        } else if num % 2 == 1 {
            num = (num * 3) + 1;
            steps += 1;
        }
    } 
    return steps;
}

fn main() {
    println!("{}", collatz(1));
    println!("{}", collatz(2));
    println!("{}", collatz(3));
    println!("{}", collatz(4));
    println!("{}", collatz(5));
}
