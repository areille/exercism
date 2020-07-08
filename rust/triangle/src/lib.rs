pub struct Triangle(u64, u64, u64);

fn is_degenerate(sides: [u64; 3]) -> bool {
    sides.contains(&0)
        || sides[0] + sides[1] < sides[2]
        || sides[1] + sides[2] < sides[0]
        || sides[2] + sides[0] < sides[1]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if is_degenerate(sides) {
            return None;
        }
        Some(Triangle(sides[0], sides[1], sides[2]))
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.0 == self.2
    }
}
