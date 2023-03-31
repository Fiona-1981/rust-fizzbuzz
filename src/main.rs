fn main() {
    let num = 1..=40;

    if num % 15 == 0 {
        println!("FizzBuzz");
    } else if num % 3 == 0 {
        println!("Fizz");
    } else if num % 5 == 0 {
        println!("Buzz")
    } else {
        return num;
    }
}
