#![allow(unused_imports)]
use std::borrow::Cow;
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
    let mut total = 0_u64;

    while let Some(line) = gets() {
        let line: Vec<char> = line.chars().collect();
        assert_eq!(line.len(), 3);
        assert_eq!(line[1], ' ');

        let opponent = Hand::parse(line[0]).unwrap();
        let desired_end = GameEnd::parse(line[2]).unwrap();

        total += match desired_end {
            GameEnd::Win => opponent.counter_to_win().score(opponent),
            GameEnd::Draw => opponent.score(opponent),
            GameEnd::Lose => opponent.counter_to_lose().score(opponent),
        };
    }

    println!("{total}");
}

#[derive(Debug, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn parse(c: char) -> Option<Hand> {
        use Hand::*;
        match c {
            'A' | 'X' => Some(Rock),
            'B' | 'Y' => Some(Paper),
            'C' | 'Z' => Some(Scissors),
            _ => None,
        }
    }

    fn score(self, o: Hand) -> u64 {
        use Hand::*;
        match self {
            Rock => match o {
                Rock => 1 + 3,
                Paper => 1 + 0,
                Scissors => 1 + 6,
            },
            Paper => match o {
                Rock => 2 + 6,
                Paper => 2 + 3,
                Scissors => 2 + 0,
            },
            Scissors => match o {
                Rock => 3 + 0,
                Paper => 3 + 6,
                Scissors => 3 + 3,
            },
        }
    }

    fn counter_to_win(self) -> Hand {
        use Hand::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn counter_to_lose(self) -> Hand {
        use Hand::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum GameEnd {
    Win,
    Draw,
    Lose,
}

impl GameEnd {
    fn parse(c: char) -> Option<GameEnd> {
        use GameEnd::*;
        match c {
            'X' => Some(Lose),
            'Y' => Some(Draw),
            'Z' => Some(Win),
            _ => None,
        }
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
