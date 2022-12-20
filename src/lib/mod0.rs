
/// public function called "bfunc" that also prints bfunc
pub fn bfunc() {
	println!("bfunc");
}

/// another module test 
#[cfg(test)]
mod tests {
    use crate::mod0::*;
  
/// this test here making sure bfunc turns out true using the assert macro
    #[test]
	fn test_bfunc() {
    	bfunc();
    	assert!(true);
    }
}
