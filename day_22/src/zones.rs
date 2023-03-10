use crate::map::{Map, Tile};
use std::io::Write;

#[allow(dead_code)]
pub fn zones_diagram(map: &Map) -> String {
    let square_len = map.ss;
    assert_eq!(map.n % square_len, 0);
    assert_eq!(map.m % square_len, 0);

    let mut buf = Vec::new();
    let n = map.n / square_len;
    let m = map.m / square_len;

    for i in 0..n {
        for j in 0..m {
            match map.d[i * square_len][j * square_len] {
                Tile::Void => write!(buf, ".").unwrap(),
                Tile::Floor | Tile::Wall => {
                    let zone_index = i * m + j;
                    write!(buf, "{:X}", zone_index).unwrap();
                }
            }
        }

        writeln!(buf).unwrap();
    }

    String::from_utf8(buf).unwrap()
}
