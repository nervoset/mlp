mod graph;

use std::ops::{Add, Mul, Sub};
use crate::graph::draw_graph;

fn main() {
    println!("========= START MLP ===========");

    println!("========= TEST ===========");
    draw_graph();
    println!("========= TEST ===========");
    let x1 = Tensor::new(2f64);
    let x2 = Tensor::new(-3f64);
    x1.represent();
    x2.represent();

    let x1x2 = x1 + x2;
    x1x2.represent();
}

#[derive(Debug)]
struct Tensor {
    pub data: f64,
    pub children: Option<Tensor>,
    pub operation: String,
    pub label: String,
}

impl Tensor {
    fn new(data: f64) -> Self {
        Tensor { 
            data,
            children: None,
            operation: "".to_string(),
            label: "".to_string(),
        }
    }

    fn represent(&self) {
        println!("Tensor(data: {})", self.data);
    }
}

impl Add for Tensor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tensor {
            data: self.data + other.data,
            children: Option::from(other),
            operation: "+".to_string(),
            label: "".to_string(),
        }
    }
}

impl Sub for Tensor {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tensor {
            data: self.data - other.data,
        }
    }
}

impl Mul for Tensor {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Tensor {
            data: self.data * other.data,
        }
    }
}