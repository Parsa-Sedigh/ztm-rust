struct Temperature {
    degrees_f: f64
}

impl Temperature {
    fn freezing() -> Self {
        Self {degrees_f: 32.0}
    }

    fn boiling() -> Self {
     Self {degrees_f: 212.0}
    }

    fn show_temp(&self) {
        println!("{:?}", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature {degrees_f: 99.9};
    hot.show_temp();

    let cold = Temperature::freezing(); // this call will return a new Temperature with degrees_f of 32.0
    cold.show_temp();
}
