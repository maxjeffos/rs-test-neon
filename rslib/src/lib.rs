use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    node_id: usize,
}

impl Node {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
        }
    }
}

pub fn get_graph() -> Graph {
    Graph {
        nodes: vec![Node::new(0), Node::new(1), Node::new(2)],
        edges: vec![Edge::new(0, 1), Edge::new(1, 2)],
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
