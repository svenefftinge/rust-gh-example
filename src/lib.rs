/// Hello from Magrathea
pub fn answer() -> u32 {
    42
}

pub fn factorial(n: u32) -> u32 {
    let mut fact = 1;
    
    for num in 1..(n+1) {
       fact *= num; 
    } 
    return fact;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_thought_test() {
        assert_eq!(answer(), 42);
    }
}
