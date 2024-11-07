mod graph;

use std::ops::{Add, Mul, Sub};
use crate::graph::draw_graph;

fn main() {
    println!("========= START MLP ===========");

    println!("========= Inputs + weights ===========");
    let mut x1 = Tensor::new(2f64, "x1".to_string());
    let mut w1 = Tensor::new(-3f64, "w1".to_string());
    let mut x2 = Tensor::new(0f64, "x2".to_string());
    let mut w2 = Tensor::new(1f64, "w2".to_string());

    x1.represent();
    w1.represent();
    x2.represent();
    w2.represent();

    let mut x1w1 = &x1 * &w1;
    x1w1.label = "x1w1".to_string();
    x1w1.represent();

    let mut x2w2 = &x2 * &w2;
    x2w2.label = "x2w2".to_string();
    x2w2.represent();

    println!("========= Linear regresion ===========");
    let mut x1w1x2w2 = &x1w1 + &x2w2;
    x1w1x2w2.label = "x1w1 + x2w2".to_string();
    x1w1x2w2.represent();

    println!("========= Bies ===========");
    let mut b = Tensor::new(6.881375870195432f64, "Bies".to_string());
    b.represent();

    let mut n = &x1w1x2w2 + &b;
    n.label = "n".to_string();
    n.represent();
    
    println!("========= Call function ===========");
    let mut o = n.tanh();
    o.label = "o".to_string();
    o.represent();

    println!("========= Manual grad calculation ===========");
    o.grad = 1.0;
    o.represent();
    n.grad = 0.5;
    n.represent();
    b.grad = 0.5;
    b.represent();
    x1w1x2w2.grad = 0.5;
    x1w1x2w2.represent();
    x2w2.grad = 0.5;
    x2w2.represent();
    x1w1.grad = 0.5;
    x1w1.represent();
    x2.grad = 0.5;
    x2.represent();
    w2.grad = 0.0;
    w2.represent();
    x1.grad = -1.5;
    x1.represent();
    w1.grad = 1.0;
    w1.represent();
}

#[derive(Debug, Clone)]
struct Tensor {
    pub data: f64,
    pub children: Vec<Box<Tensor>>,
    pub operation: String,
    pub label: String,
    pub grad: f64,
}

impl Tensor {
    fn new(data: f64, label: String) -> Self {
        Tensor {
            data,
            children: vec![],
            operation: "".to_string(),
            label,
            grad: 0.0,
        }
    }

    fn tanh(&self) -> Self {
        let x = self.data;
        let t = ((2.0 * x).exp() - 1.0) / ((2.0 * x).exp() + 1.0);
        let out = Tensor::new(t, "tanh".to_string());
        out
    }

    fn represent(&self) {
        println!("Tensor(data: {:?}, label: {:?}, gradient: {:?})",
                 self.data,
                 self.label,
                 self.grad
        );

    }
}

impl Add for &Tensor {
    type Output = Tensor;

    fn add(self, other: &Tensor) -> Tensor {
        let mut tensor = Tensor::new(
            self.data + other.data,
            "".to_string(),
        );
        tensor.operation = "+".to_string();
        tensor.children = vec![Box::new(other.clone())];
        tensor
    }
}

impl Sub for &Tensor {
    type Output = Tensor;

    fn sub(self, other: &Tensor) -> Tensor {
        let mut tensor = Tensor::new(
            self.data - other.data,
            "".to_string(),
        );
        tensor.operation = "-".to_string();
        tensor.children = vec![Box::new(other.clone())];
        tensor
    }
}

impl Mul for &Tensor {
    type Output = Tensor;

    fn mul(self, other: &Tensor) -> Tensor {
        let mut tensor = Tensor::new(
            self.data * other.data,
            "".to_string(),
        );
        tensor.operation = "*".to_string();
        tensor.children = vec![Box::new(other.clone())];
        tensor
    }
}