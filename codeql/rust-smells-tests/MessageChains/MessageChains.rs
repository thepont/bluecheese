struct Root;
struct Child;
struct State;

impl Root {
    fn get_child(&self) -> Child { Child }
}

impl Child {
    fn get_state(&self) -> State { State }
}

impl State {
    fn get_value(&self) -> i32 { 0 }
}

fn long_chain(root: &Root) -> i32 {
    root.get_child().get_state().get_value()
}

fn short_chain(root: &Root) -> State {
    root.get_child().get_state()
}
