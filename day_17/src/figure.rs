use crate::{line::Line, pixel::Pixel};
use std::fmt::Debug;

#[derive(Clone, Eq, PartialEq)]
pub struct Figure {
    pub n: usize,
    pub m: usize,
    // From bottom left.
    pub body: Vec<Line>,
}

impl Debug for Figure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Figure [{}x{}]:", self.n, self.m)?;
        for i in 0..self.n {
            for j in 0..self.m {
                write!(f, "{}", self.body[i].get(j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Figure {
    pub fn new(s: &str) -> Figure {
        let lines: Vec<_> = s.split_whitespace().map(|line| line.as_bytes()).collect();

        let n = lines.len();
        assert!(n > 0);

        let m = lines[0].len();
        assert!(m > 0);
        assert!(lines.iter().all(|line| line.len() == m));

        let mut ret = Figure {
            n,
            m,
            body: vec![Line::new(); n],
        };

        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for j in 0..m {
                ret.body[n - i - 1].set(j, Pixel::parse(lines[i][j]));
            }
        }

        ret
    }
}
