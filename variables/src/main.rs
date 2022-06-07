use std::io;

fn main() {
    loop{
        println!("Please input a number to process ");
        
        let mut input_number = String::new();
        io::stdin()
         .read_line(&mut input_number)
         .expect("Failed to read that line");

         let input_number = match input_number.trim().parse() {
             Ok(input_number) => input_number,
             Err(_) => continue,
         };
        
        let fib_number = fib(input_number, 0);

        println!("fibonacci number is: {fib_number}")


    }

}

fn fib(number: u64, total: u16) -> u64 {
    if number <= 1 {
        return 1;
    }
    else {
        return fib(number - 1, total + 1) + fib(number - 2, total + 1);
    }
}
