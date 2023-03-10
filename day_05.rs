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
    let stack_count = gets().unwrap().to_usize();
    let mut stacks: Vec<String> = std::iter::repeat(())
        .take(stack_count)
        .map(|_| gets().unwrap())
        .collect();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    while let Some(line) = gets() {
        let caps = RE.captures(&line).unwrap();
        let amount = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap() - 1;
        let to = caps[3].parse::<usize>().unwrap() - 1;

        let (from, to) = get_mutable_refs(&mut stacks, from, to);

        let n = from.len();
        to.push_str(&from[(n - amount)..]);
        from.replace_range((n - amount).., "");
    }

    for stack in &stacks {
        print!("{}", stack.chars().last().unwrap());
    }
    println!();
}

fn get_mutable_refs<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    if i > j {
        let (b, a) = get_mutable_refs(slice, j, i);
        (a, b)
    } else {
        assert!(i < j);
        let (s1, s2) = slice.split_at_mut(j);
        (&mut s1[i], &mut s2[0])
    }
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
