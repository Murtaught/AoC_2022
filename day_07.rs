#![allow(unused_imports)]
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io::{stdin, stdout, BufRead, Read, Write};
use std::mem::swap;
use std::num;
use std::ops::{BitOr, Deref, Range, RangeInclusive, Rem};
use std::str::FromStr;

fn main() {
    let mut fs = HashMap::<Vec<String>, u64>::new();
    let mut cur = Vec::<String>::new();
    let mut after_ls = false;

    while let Some(line) = gets() {
        if line.starts_with('$') {
            // A command.
            if let Some(target) = line.strip_prefix("$ cd ") {
                eprint!("$ cd {target:?}  => ");
                if target == ".." {
                    assert!(!cur.is_empty());
                    cur.pop();
                } else if target == "/" {
                    cur.clear();
                } else {
                    assert!(!target.is_empty());
                    cur.push(target.to_string());
                }
                eprintln!("{cur:?}");
                after_ls = false;
            } else {
                assert_eq!(line, "$ ls");
                after_ls = true;
            }
        } else {
            assert!(after_ls);

            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(\d+) ([\w\.]+)$").unwrap();
            }
            
            if let Some(caps) = RE.captures(&line) {
                let size = caps[1].parse::<u64>().unwrap();
                let file = &caps[2];
                eprintln!("file {file:?}, size {size}");
    
                let mut it = cur.clone();
                loop {
                    if let Some(dir_size) = fs.get_mut(&it) {
                        *dir_size += size;
                    } else {
                        fs.insert(it.clone(), size);
                    }
                    
                    if it.is_empty() {
                        break;
                    }

                    it.pop();
                }
            }
        }
    }

    const DISK_SIZE: u64 = 70_000_000;
    const UPDATE_SIZE: u64 = 30_000_000;
    let total_used = fs[&vec![]];
    
    let total_free = DISK_SIZE - total_used;
    let need_to_free = UPDATE_SIZE - total_free;

    println!("total used: {total_used}");
    println!("total free: {total_free}");
    println!("need to free: {need_to_free}");

    let ans = fs.values().filter(|&&x| x >= need_to_free).min().unwrap();
    println!("ans: {ans}");
}

// ------- ✂ -------

fn gets() -> Option<String> {
    let mut line = String::new();
    let count = stdin().read_line(&mut line).expect("Failed to read line");
    if count > 0 {
        Some(line.chomp())
    } else {
        None
    }
}

trait StringExt {
    fn chomp(self) -> String;
}

impl StringExt for String {
    fn chomp(mut self) -> String {
        loop {
            match self.chars().last() {
                Some(c) if c.is_whitespace() => self.pop(),
                _ => break,
            };
        }
        self
    }
}

trait StrExt {
    fn to_i(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_f64(&self) -> f64;
    fn to_usize(&self) -> usize;

    fn parse_2<F>(&self) -> (F, F)
    where
        F: FromStr,
        <F as FromStr>::Err: Debug;

    fn parse_3<F>(&self) -> (F, F, F)
    where
        F: FromStr,
        <F as FromStr>::Err: Debug;

    fn parse_vec<T>(&self, capacity: usize) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug;
}

impl<S> StrExt for S
where
    S: Deref<Target = str>,
{
    fn to_i(&self) -> i32 {
        self.parse().unwrap()
    }
    fn to_i64(&self) -> i64 {
        self.parse().unwrap()
    }
    fn to_f64(&self) -> f64 {
        self.parse().unwrap()
    }
    fn to_usize(&self) -> usize {
        self.parse().unwrap()
    }

    fn parse_2<T>(&self) -> (T, T)
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut it = self.split_whitespace();
        let fst: T = it.next().unwrap().parse().unwrap();
        let snd: T = it.next().unwrap().parse().unwrap();
        (fst, snd)
    }

    fn parse_3<T>(&self) -> (T, T, T)
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut it = self.split_whitespace();
        let fst: T = it.next().unwrap().parse().unwrap();
        let snd: T = it.next().unwrap().parse().unwrap();
        let thd: T = it.next().unwrap().parse().unwrap();
        (fst, snd, thd)
    }

    fn parse_vec<T>(&self, capacity: usize) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut vec = Vec::with_capacity(capacity);
        for word in self.split_whitespace() {
            vec.push(word.parse().unwrap());
        }
        vec
    }
}

trait IsOdd {
    fn is_odd(&self) -> bool;

    fn is_even(&self) -> bool {
        !self.is_odd()
    }
}

impl<T> IsOdd for T
where
    T: Copy + PartialEq + From<u8> + Rem<Output = T>,
{
    fn is_odd(&self) -> bool {
        *self % T::from(2) == T::from(1)
    }
}

trait CountMap<K> {
    fn empty_count_map() -> Self;
    fn increment(&mut self, key: K);
    fn decrement(&mut self, key: &K) -> bool;
}

impl<K: Hash + Eq> CountMap<K> for HashMap<K, usize> {
    fn empty_count_map() -> Self {
        HashMap::new()
    }

    fn increment(&mut self, key: K) {
        *self.entry(key).or_insert(0) += 1;
    }

    fn decrement(&mut self, key: &K) -> bool {
        if let Some(value) = self.get_mut(key) {
            *value -= 1;

            if *value == 0 {
                self.remove(key);
            }

            return true;
        }

        false
    }
}

impl<K: Ord + Eq> CountMap<K> for BTreeMap<K, usize> {
    fn empty_count_map() -> Self {
        BTreeMap::new()
    }

    fn increment(&mut self, key: K) {
        *self.entry(key).or_insert(0) += 1;
    }

    fn decrement(&mut self, key: &K) -> bool {
        if let Some(value) = self.get_mut(key) {
            *value -= 1;

            if *value == 0 {
                self.remove(key);
            }

            return true;
        }

        false
    }
}

trait Count<M>
where
    M: CountMap<Self::Item>,
{
    type Item;

    fn into_count_map(self) -> M;
}

impl<I, M> Count<M> for I
where
    I: Iterator,
    M: CountMap<I::Item>,
{
    type Item = I::Item;

    fn into_count_map(self) -> M {
        self.fold(M::empty_count_map(), |mut acc, x| {
            acc.increment(x);
            acc
        })
    }
}
