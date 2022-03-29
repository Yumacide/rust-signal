#[cfg(test)]
mod tests;

type Callback<'a, T> = fn(&'a T) -> ();

pub struct Signal<'a, T> {
    callbacks: Vec<Callback<'a, T>>,
}

impl<'a, T> Signal<'a, T> {
    pub fn new() -> Self {
        Signal {
            callbacks: Vec::new(),
        }
    }

    pub fn connect(&mut self, callback: Callback<'a, T>) {
        self.callbacks.push(callback);
    }

    pub fn disconnect(&mut self, callback: Callback<'a, T>) -> Result<(), &'static str> {
        let result = self.callbacks.iter().position(|&x| x == callback);

        match result {
            Some(index) => {
                self.callbacks.remove(index);
                Ok(())
            }

            None => Err("Callback not connected to this signal"),
        }
    }

    pub fn fire(&self, arg: &'a T) {
        for &callback in &self.callbacks {
            callback(arg);
        }
    }
}
