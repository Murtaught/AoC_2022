use crate::resources::Resources;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Blueprint {
    pub number: usize,
    pub ar: Resources,
    pub br: Resources,
    pub cr: Resources,
    pub dr: Resources,
    pub max_a: u16,
    pub max_b: u16,
    pub max_c: u16,
}

impl Blueprint {
    pub const A_INC: Resources = Resources {
        a: 1,
        b: 0,
        c: 0,
        d: 0,
    };
    pub const B_INC: Resources = Resources {
        a: 0,
        b: 1,
        c: 0,
        d: 0,
    };
    pub const C_INC: Resources = Resources {
        a: 0,
        b: 0,
        c: 1,
        d: 0,
    };
    pub const D_INC: Resources = Resources {
        a: 0,
        b: 0,
        c: 0,
        d: 1,
    };

    pub fn parse(line: &str) -> Blueprint {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r#"(?x)
                Blueprint\s+(\d+):\s+
                Each\s+ore\s+robot\s+costs\s+(\d+)\s+ore.\s+
                Each\s+clay\s+robot\s+costs\s+(\d+)\s+ore.\s+
                Each\s+obsidian\s+robot\s+costs\s+(\d+)\s+ore\s+and\s+(\d+)\s+clay.\s+
                Each\s+geode\s+robot\s+costs\s+(\d+)\s+ore\s+and\s+(\d+)\s+obsidian.
                "#
            )
            .unwrap();
        }

        let caps = RE.captures(line).unwrap();
        let number = caps[1].parse().unwrap();
        let ar = Resources {
            a: caps[2].parse().unwrap(),
            ..Resources::default()
        };
        let br = Resources {
            a: caps[3].parse().unwrap(),
            ..Resources::default()
        };
        let cr = Resources {
            a: caps[4].parse().unwrap(),
            b: caps[5].parse().unwrap(),
            ..Resources::default()
        };
        let dr = Resources {
            a: caps[6].parse().unwrap(),
            c: caps[7].parse().unwrap(),
            ..Resources::default()
        };

        Blueprint {
            number,
            ar,
            br,
            cr,
            dr,
            max_a: [ar.a, br.a, cr.a, dr.a].into_iter().max().unwrap(),
            max_b: cr.b,
            max_c: dr.c,
        }
    }
}
