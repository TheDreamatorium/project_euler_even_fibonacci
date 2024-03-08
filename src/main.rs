fn fibonacci_sequence(nth: i64) -> i64 {
    
    if nth <= 2 {
        return nth;
    }

    fibonacci_sequence(nth-1) + fibonacci_sequence(nth-2)
}
 
fn main() {

    let mut i = 0;
    let mut current_term = 0; 
    let mut sum = 0;

    while current_term < 4000000 {
        current_term = fibonacci_sequence(i + 1);

        if current_term % 2 == 0 {
            sum  = sum + current_term;
        }
        
        i = i + 1;
    }

    println!("Sum of even fibonacci terms: {}", sum);
}
