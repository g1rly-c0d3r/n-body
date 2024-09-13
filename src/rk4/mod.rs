
mod rk4 {
use std::result::Result;
use std::vec::Vec;

    fn itterate(dydt:fn(f64, Vec<f64>) -> Vec<f64>, Y_n:&[f64], t:f64, h:f64, m:usize) -> Result< Vec<f64> , &'static str> {
        let mut k: Vec<Vec<f64>> = Vec::new();

        for i in 0..4{
            k.push(Vec::new());
        }

        let step = dydt(t, Y_n.to_vec());
        for i in 0..m{
            k[0].push(h * step[i]);
        }

        let step = Y_n.to_vec().iter().zip(k[0].iter()).map(|(x,y)| x+y).collect();
        for i in 0..m{
            k[1].push(
                h * dydt(t + h/2f64, step[i]);
            )
        }


        
        
        Ok(vec![1.0f64])
    }

}
