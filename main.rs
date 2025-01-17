mod matrix;
mod network;
mod acivation;
//use lib::{activations::SIGMOID,network::Network};
use crate::network::Network;
use crate::acivation::SIGMOID;
//pub mod lib;
fn main() {
    let inputs=vec![
        vec![0.0,0.0],
        vec![0.0,1.0],
        vec![1.0,0.0],
        vec![1.0,1.0],
    ];
    let targets=vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
    ];
    let mut network=Network::new(vec![2,3,1],0.5,SIGMOID);

    println!("0 and 0 : {:?}",network.feed_forward(vec![0.0,0.0]));
    println!("0 and 1: {:?}",network.feed_forward(vec![0.0,1.0]));
    println!("1 and 0 :{:?}",network.feed_forward(vec![1.0,0.0]));
    println!("1 and 1 :{:?}",network.feed_forward(vec![1.0,1.0]));

    network.train(inputs,targets,10000);

    println!("0 and 0 : {:?}",network.feed_forward(vec![0.0,0.0]));
    println!("0 and 1: {:?}",network.feed_forward(vec![0.0,1.0]));
    println!("1 and 0 :{:?}",network.feed_forward(vec![1.0,0.0]));
    println!("1 and 1 :{:?}",network.feed_forward(vec![1.0,1.0]));
}

