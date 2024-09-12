
mod rk4 {
use std::result::Result;
use std::vec::Vec;

    fn itterate(dydt:fn(f64, Vec<f64>) -> Vec<f64>, Y_n:&[f64], t:f64, h:f64, m:const u8) -> Result< Vec<f64> , &'static str> {
        let mut k = [[0f64; 4]; m];
        
        Ok(vec![1.0f64])
    }

}
