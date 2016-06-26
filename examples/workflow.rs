extern crate tensorflux;

use tensorflux::{Definition, Options, Session};

const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[allow(unused_variables)]
fn main() {
    let definition = ok!(Definition::load(GRAPH_PATH)); // c = a * b
    let options = ok!(Options::new());
    let mut session = ok!(Session::new(options));
    ok!(session.extend(&definition));
}
