type Domino = (u8, u8);

fn flip(domino: Domino) -> Domino {
    (domino.1, domino.0)
}

fn solve_from(chain: &mut [Domino], from: usize) -> bool {
    let value = chain[from - 1].1;
    if from >= chain.len() {
        chain[0].0 == value
    } else {
        for i in from..chain.len() {
            if chain[i].0 == value || chain[i].1 == value {
                chain.swap(from, i);
                if chain[from].0 != value {
                    chain[from] = flip(chain[from]);
                }
                if solve_from(chain, from + 1) {
                    return true;
                }
            }
        }
        false
    }
}

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    if input.is_empty() {
        Some(vec![])
    } else {
        let mut chain = input.to_owned();
        if solve_from(&mut chain, 1) {
            Some(chain)
        } else {
            None
        }
    }
}
