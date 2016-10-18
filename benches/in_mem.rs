#![feature(test)]

extern crate cgraph;
extern crate test;
use test::Bencher;

#[bench]
fn run(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mir_graphviz = (br###"
digraph testgraph {
    A -> B -> C
    D [label="foo"]
    B -> D [label="bar"]
}
"### as &[u8]).to_owned();
        let s = String::from_utf8(cgraph::Graph::parse(mir_graphviz).unwrap().render_dot().unwrap()).unwrap();
        test::black_box(s);
    });
}
