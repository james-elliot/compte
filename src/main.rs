// This is NOT a very efficient code.
// Efficiency requires the use of hash tables
// See my paper: http://arxiv.org/abs/1502.05450 

fn add(a:i64,b:i64) -> Option<i64> {
    return Some(a+b)
}

fn sub(a:i64,b:i64) -> Option<i64> {
    let res = a-b;
    if res != b && res!=0 {return Some(res)}
    else {return None}
}

fn mul(a:i64,b:i64) -> Option<i64> {
    if b!=1 {return Some(a*b)}
    else {return None}
}

fn div(a:i64,b:i64) -> Option<i64> {
    if b != 1 && a%b == 0 {
	let res = a/b;
	if res != b {return Some(res)}
	else {return None}
    }
    else {return None}
}

const OPSN:[&str;4] = ["+","-","*","/"];
const OPS: [fn(i64, i64) -> Option<i64>; 4] =
    [add as fn(i64, i64) -> Option<i64>,
     sub as fn(i64, i64) -> Option<i64>,
     mul as fn(i64, i64) -> Option<i64>,
     div as fn(i64, i64) -> Option<i64>];

#[derive(Debug,Clone)]
enum Computation {
    Node(usize,i64,i64,Box<Computation>),
    End,
    Nothing
}

use crate::Computation::{End,Node,Nothing};

fn print_tree(t:Computation) {
    match t {
	Node(i,a,b,t2) => {
	    println!("{}{}{}={}",a,OPSN[i],b,OPS[i](a,b).unwrap());
	    print_tree(*t2)
	},
	End|Nothing => {}
    }
}

use core::cmp::{min,max};
fn test(a:&Vec<i64>, goal:i64) -> Computation {
    let mut newa = Vec::with_capacity(a.len()-1);
    for i in 0..a.len() {
	for j in i+1..a.len() {
	    let (v1,v2)=(max(a[i],a[j]),min(a[i],a[j]));
	    for k in 0..OPS.len() {
		match OPS[k](v1,v2) {
		    Some (r) => {
			if r == goal {return Node(k,v1,v2,Box::new(End))}
			if a.len() > 2 {
			    newa.clear();
			    for m in 0..a.len() {if m!=i && m!=j {newa.push(a[m])}}
			    newa.push(r);
			    let res = test(&newa,goal);
			    match res {
				Node(..) => return Node(k,v1,v2,Box::new(res)),
				Nothing => {},
				End => panic!("Never!!!")
			    }
			}
		    }
		    None => {}
		}
	    }
	}
    }
    return Nothing
}

fn all(a:&Vec<i64>,tab:&mut[bool]){
    let mut newa = Vec::with_capacity(a.len()-1);
    for i in 0..a.len() {
	for j in i+1..a.len() {
	    let (v1,v2)=(max(a[i],a[j]),min(a[i],a[j]));
	    for k in 0..OPS.len() {
		match OPS[k](v1,v2) {
		    Some (r) => {
			if (r as usize) < tab.len() {tab[r as usize]=true};
			if a.len() > 2 {
			    newa.clear();
			    for m in 0..a.len() {
				if m!=i && m!=j {newa.push(a[m])}
			    }
			    newa.push(r);
			    all(&newa,tab);
			    }
		    }
		    None => {}
		}
	    }
	}
    }
}

use std::env;

// Usage: cargo run --release 1 2 3 4 5 6 256
// There can be any number of arguments
// the last one is always the number to find
#[allow(dead_code)]
fn solve_one() {
    let args: Vec<String> = env::args().collect();
    let g = args[args.len()-1].parse().unwrap();
    let mut t = Vec::with_capacity(args.len()-2);
    for i in 1..args.len()-1 {t.push(args[i].parse().unwrap())}
    print_tree(test(&t,g));
}

#[allow(dead_code)]
fn solve_all() {
    let args: Vec<String> = env::args().collect();
    let mut t = Vec::with_capacity(args.len()-1);
    for i in 1..args.len() {t.push(args[i].parse().unwrap())}
    let mut tab = [false;1000];
    all(&t,&mut tab);
    let mut n = 0;
    for i in 100..tab.len() {
	if tab[i] {n=n+1}
    }
    println!("{}",n);
}

// Generates the classical set of numbers
fn gen_numbers() -> Vec<i64> {
    let mut t = Vec::new();
    for i in 1..11 {t.push(i);t.push(i)}
    t.push(25);
    t.push(50);
    t.push(75);
    t.push(100);
    return t;
}

//Usage: cargo run --release 6 100 1000
use std::collections::HashSet;
fn solve_classical() {
    let args: Vec<String> = env::args().collect();
    let size = args[1].parse().unwrap();
    let inf  = args[2].parse().unwrap();
    let sup  = args[3].parse().unwrap();
    let nums = gen_numbers();
    let mut ind = Vec::with_capacity(size);
    let mut nbset = HashSet::new();
    for i in 0..size {
	ind.push(i);
    }
    let (mut cnt,mut cnt2) = (0,0);
    loop {
	let mut t = Vec::with_capacity(size);
	for i in 0..size {t.push(nums[ind[i]])}
	if !nbset.contains(&t) {
	    nbset.insert(t.clone());
	    let mut tab = vec![false;sup];
	    all(&t,&mut tab);
	    let mut n = 0;
	    for i in inf..tab.len() {
		if tab[i] {n=n+1}
	    }
	    if n == sup-inf {
		cnt2=cnt2+1;
		println!("t={:?} n={}",t,n)
	    }
	    cnt=cnt+1;
	}
	for i in (0..size).rev() {
	    ind[i]=ind[i]+1;
	    if ind[i] < nums.len()-(size-1-i) {break}
	    ind[i]=0;
	    if i==0 {
		println!("cnt={} cnt2={}",cnt,cnt2);
		return
	    }
	}
	for i in 1..size {
	    if ind[i]<=ind[i-1] {ind[i]=ind[i-1]+1}
	}
    }
}

fn main() {
    // solve_one();
    // solve_all();
    solve_classical();
}
