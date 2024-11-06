use petgraph::dot::{Dot, Config};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashSet, HashMap};
use petgraph::Graph;

pub fn draw_graph() {
    let mut graph = Graph::<&str, u32>::new();
    let origin = graph.add_node("data: -3.0");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");

    graph.extend_with_edges(&[
        (origin, destination_1, 250),
        (origin, destination_2, 1099)
    ]);

    println!("{}", Dot::new(&graph));
}

#[derive(Debug)]
struct Node {
    data: f64,
    op: Option<String>,
}

fn trace(root: NodeIndex, graph: &DiGraph<Node, ()>) -> (HashSet<NodeIndex>, HashSet<(NodeIndex, NodeIndex)>) {
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();

    fn build(v: NodeIndex, nodes: &mut HashSet<NodeIndex>, edges: &mut HashSet<(NodeIndex, NodeIndex)>, graph: &DiGraph<Node, ()>) {
        if nodes.insert(v) {
            for neighbor in graph.neighbors_directed(v, petgraph::Incoming) {
                edges.insert((neighbor, v));
                build(neighbor, nodes, edges, graph);
            }
        }
    }

    build(root, &mut nodes, &mut edges, graph);
    (nodes, edges)
}

fn draw_dot(root: NodeIndex, graph: &DiGraph<Node, ()>) -> String {
    let (nodes, edges) = trace(root, graph);
    let mut dot_output = String::new();

    for n in nodes {
        let node = &graph[n];
        let label = match node.op {
            Some(ref op) => format!("{} | op: {}", node.data, op),
            None => format!("data: {:.4}", node.data),
        };
        dot_output.push_str(&format!("\"{}\" [label=\"{}\", shape=\"record\"];\n", n.index(), label));
    }

    for (n1, n2) in edges {
        dot_output.push_str(&format!("\"{}\" -> \"{}\";\n", n1.index(), n2.index()));
    }

    format!("digraph {{\n{}\n}}", dot_output)
}

fn main() {
    // Создаем граф и добавляем узлы
    let mut graph = DiGraph::new();

    // Пример создания узлов с данными и операциями
    let a = graph.add_node(Node { data: 1.0, op: None });
    let b = graph.add_node(Node { data: 2.0, op: Some(String::from("add")) });
    let c = graph.add_node(Node { data: 3.0, op: Some(String::from("mul")) });

    // Добавляем рёбра между узлами
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());

    // Экспорт графа в формат DOT
    let dot_format = draw_dot(a, &graph);
    std::fs::write("graph.dot", dot_format).expect("Не удалось сохранить файл");

    println!("Граф сохранён в формате DOT в файл 'graph.dot'");
}