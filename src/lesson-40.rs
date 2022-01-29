struct Temperature {
    degrees_f: f64
}

impl Temperature {
    fn freezing() -> Self {
        Self {degrees_f: 32.0}
    }

    fn show_temp(&self) {
        println!();
    }
}
