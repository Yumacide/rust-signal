type Callback<'a, T> = fn(&'a T) -> ();

struct Signal<'a, T> 
    where T: Sync
{
    callbacks: Vec<Callback<'a, T>>
}

impl<'a, T> Signal<'a, T> 
    where T: Sync
{
    fn new() -> Self {
        Signal {
            callbacks: Vec::new()
        }
    }

    fn connect(&mut self, callback: Callback<'a, T>){
        self.callbacks.push(callback);
    }

    fn disconnect(&mut self, callback: Callback<'a, T>) -> Result<(), &'static str> {
        let result = self.callbacks.iter().position(|&x| x == callback);

        match result {
            Some(index) => {
                self.callbacks.remove(index);
                Ok(())
            }

            None => Err("Callback not connected to this signal")
        }
    }

    fn fire(&self, arg: &'a T)
    {
        for &callback in &self.callbacks {
            callback(arg);
        }
    } 
}

fn do_stuff(v: &Vec<String>) {
    for str in v {
        println!("{}", *str);
    }
}

fn dostuff(t: String) {}
fn main() {
    let mut stack_signal = Signal::<i32>::new();
    let some_callback = |&x| println!("{}", x);

    stack_signal.connect(some_callback);
    stack_signal.fire(&1);
    stack_signal.disconnect(some_callback).expect("");
    stack_signal.fire(&2);

    let mut heap_signal = Signal::<Vec<String>>::new();
    heap_signal.connect(do_stuff);
    heap_signal.fire(&vec![String::from("Hello, world!")]);
}
