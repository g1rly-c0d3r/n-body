mod rk4 {
use std::vec::Vec;

    fn itterate(dydt:fn(f64, Vec<f64>) -> Vec<f64>, Y_n:&[f64], t:f64, h:f64, m:usize) -> Vec<f64> {
        let mut k: Vec<Vec<f64>> = Vec::new();

        for i in 0..4{
            k.push(Vec::new());
        }

        let step = dydt(t, Y_n.to_vec());
        for i in 0..m{
            k[0].push(h * step[i]);
        }

        let step = dydt(t + h/2f64, Y_n.to_vec().iter().zip(k[0].iter()).map(|(x,y)| (x+y)/2f64).collect());
        for i in 0..m{
            k[1].push(
                h * step[i]
            );
        }

        let step = dydt(t + h/2f64, Y_n.to_vec().iter().zip(k[1].iter()).map(|(x,y)| (x+y)/2f64 ).collect());
        for i in 0..m{
            k[2].push(
                h * step[i]
            );
        }
        
        let step = dydt(t + h, Y_n.to_vec().iter().zip(k[2].iter()).map(|(x,y)| x+y).collect());
        for i in 0..m{
            k[3].push(
                h * step[i]
            );
        }

        let a: Vec<_> = k[1].iter().zip(k[2].iter()).map(|(x,y)| 2f64*(x+y)).collect();
        let b: Vec<_> = k[0].iter().zip(k[3].iter()).map(|(x,y)| (x+y)).collect();
        let c: Vec<_> = a.iter().zip(b.iter()).map(|(x,y)| (x+y)/6f64).collect();

        Y_n.iter().zip(c.iter()).map(|(x,y)| x+y).collect()
    }


    fn RK4() -> {

    }
}

