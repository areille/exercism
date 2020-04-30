pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn binomial_coeff(top: u32, bottom: u32) -> u32 {
    factorial(top) / (factorial(bottom) * factorial(top - bottom))
}
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for i in 0..row_count {
            let mut vec: Vec<u32> = Vec::new();
            for j in 0..=i {
                vec.push(binomial_coeff(i, j));
            }
            rows.push(vec);
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
