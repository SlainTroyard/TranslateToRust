use std::collections::{BTreeSet, HashMap, HashSet};
use std::cmp::{max, min};
use std::io::{self, BufRead};

const BLOCK_SIZE: usize = 512;

#[derive(Debug, Clone)]
struct Query {
    lx: usize,
    rx: usize,
    ly: usize,
    ry: usize,
    area: i64,
}

impl Query {
    fn new(lx: usize, rx: usize, ly: usize, ry: usize, area: i64) -> Self {
        Self { lx, rx, ly, ry, area }
    }
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.lx == other.lx &&
        self.rx == other.rx &&
        self.ly == other.ly &&