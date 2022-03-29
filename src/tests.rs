use crate::Signal;

fn callback(v: &Vec<String>) {
    for str in v {
        println!("{}", *str);
    }
}

#[test]
fn stack_signal() {
    let mut stack_signal = Signal::<i32>::new();
    let callback_closure = |&x| println!("{}", x);

    stack_signal.connect(callback_closure);
    stack_signal.fire(&1);
    stack_signal.disconnect(callback_closure).expect("");
    stack_signal.fire(&2);
}

#[test]
fn heap_signal() {
    let mut heap_signal = Signal::<Vec<String>>::new();

    heap_signal.connect(callback);
    heap_signal.fire(&vec![String::from("Hello, world!")]);
}
