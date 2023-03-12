pub struct Meters(f32);

impl Meters {
    pub fn new(m: f32) -> Self {
        Meters(m)
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn to_feet(self) -> Feet {
        Feet::new(self.0 / 0.3048)
    }
}

pub struct Feet(f32);

impl Feet {
    pub fn new(f: f32) -> Self {
        Feet(f)
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn to_meters(self) -> Meters {
        Meters::new(self.0 * 0.3048)
    }
}
