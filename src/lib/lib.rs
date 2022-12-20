/// public test function
pub fn test() {
  println!("Test");
}
///public modules from 0 to 2
pub mod mod0;
pub mod mod1;
pub mod mod2;
//pub use mod2::bar;

///testing the modules
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
