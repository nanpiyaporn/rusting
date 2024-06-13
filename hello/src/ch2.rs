fn ch2(){
  println!("{}",is_even(2)); 
 
  pub fn is_even(n: u8) -> bool {
    let digit: u8 = n % 2;
    if digit == 0 {
        return true;
    } else {
        return false;
    }
  }
}