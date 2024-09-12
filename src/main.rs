extern crate yaml_rust;
use yaml_rust::YamlLoader;
pub mod rk4;

#[derive(Debug)]
struct Body {
    position: [f64; 3],
    velocity: [f64; 3],
    acelleration: [f64; 3],

    mass: f64,
    radius: f64,
}

fn main() {
    let sun = Body {
        position : [0.0,0.0,0.0],
        velocity : [0.0,0.0,0.0],
        acelleration : [0.0,0.0,0.0],
        mass : 1.989E30,
        radius : 6.96E5,
    };

    let s = 
        "
        foo: 
            - list1
            - list2
        bar: 
            - 0
            - 1
        ";

    let docs = YamlLoader::load_from_str(s).unwrap();

    println!("{:?}", docs);

    //println!("{:?}", sun)

}
