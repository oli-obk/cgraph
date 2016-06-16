extern crate cgraph;

#[test]
fn run() {
    for _ in 0..10 {
        let mir_graphviz = (b"graph testgraph {\n\n}" as &[u8]).to_owned();
        let s = String::from_utf8(cgraph::Graph::from(mir_graphviz).render_dot().unwrap()).unwrap();
        drop(s);
    }
}
