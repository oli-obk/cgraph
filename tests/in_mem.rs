extern crate cgraph;

#[test]
fn run() {
    for _ in 0..10 {
        let mir_graphviz = (b"graph testgraph {\n\n}" as &[u8]).to_owned();
        let s = String::from_utf8(cgraph::Graph::parse(mir_graphviz).unwrap().render_dot().unwrap()).unwrap();
        drop(s);
    }
}

#[test]
fn digraph() {
    for _ in 0..10 {
        let mir_graphviz = (b"digraph testgraph { A -> B }" as &[u8]).to_owned();
        let s = String::from_utf8(cgraph::Graph::parse(mir_graphviz).unwrap().render_dot().unwrap()).unwrap();
        drop(s);
    }
}
