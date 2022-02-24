struct Dimension {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimension;
}

struct ConveyorBelt<T: Convey> {
    pub items: Vec<T>,
}

// generic implementation:
impl<T: Convey> ConveyorBelt<T> {
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "abc".to_string()
        }
    }
}

impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }
    fn dimensions(&self) -> Dimension {
        Dimension {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

fn main() {
    let mut belt = ConveyorBelt {
        items: vec![]
    };
    belt.add(CarPart::default());
}

