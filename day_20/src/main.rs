fn main() {
    let content = std::fs::read_to_string("input2").unwrap();
    let xs: Vec<(i64, usize)> = content
        .lines()
        .map(|line| line.parse().unwrap())
        .zip(0..)
        .collect();

    println!("ans (p1): {}", solve_p1(&xs));
    println!("ans (p2): {}", solve_p2(&xs));
}

fn solve_p1(xs: &[(i64, usize)]) -> i64 {
    let mut mixer = Mixer(xs.to_owned());

    for &(_, j) in xs {
        mixer.move_elem(j);
    }

    let a = mixer.after_zero(1000);
    let b = mixer.after_zero(2000);
    let c = mixer.after_zero(3000);

    a + b + c
}

fn solve_p2(xs: &[(i64, usize)]) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;

    let mut vec = xs.to_owned();
    for (x, _) in &mut vec {
        (*x) *= DECRYPTION_KEY;
    }

    let mut mixer = Mixer(vec);

    for _ in 0..10 {
        for &(_, j) in xs {
            mixer.move_elem(j);
        }
    }

    let a = mixer.after_zero(1000);
    let b = mixer.after_zero(2000);
    let c = mixer.after_zero(3000);
    a + b + c
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mixer(pub Vec<(i64, usize)>);

impl Mixer {
    pub fn move_elem(&mut self, j: usize) {
        let index_a = self.index_of(j);
        let x = self.0[index_a].0;
        let index_b = self.cycle_index(index_a, x);

        let elem = self.0.remove(index_a);
        self.0.insert(index_b, elem);
    }

    pub fn after_zero(&self, addendum: i64) -> i64 {
        let zero_index = self
            .0
            .iter()
            .enumerate()
            .find(|(_, (x, _))| *x == 0)
            .map(|(i, _)| i)
            .unwrap();

        let n = self.0.len() as i64;
        let index = (zero_index as i64 + addendum) % n;

        self.0[index as usize].0
    }

    fn index_of(&self, index: usize) -> usize {
        self.0
            .iter()
            .enumerate()
            .find(|(_, (_, j))| *j == index)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn cycle_index(&self, index: usize, addendum: i64) -> usize {
        let n = self.0.len() as i64;
        let mut index = index as i64 + addendum;

        if index < 0 {
            let mult = (-index + n - 2) / (n - 1);
            index += mult * (n - 1);
        }

        if index >= n {
            let mult = index / (n - 1);
            index -= mult * (n - 1);
        }

        assert!(0 <= index);
        assert!(index < n);

        index as usize
    }
}
