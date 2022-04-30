// Usage: compte 1 2 3 4 5 6 256
// There can be any number of arguments, the last one is always the number to find

fn add(a:i64,b:i64) -> Option<i64> {
    return Some(a+b)
}

fn sub(a:i64,b:i64) -> Option<i64> {
    if a>b {return Some(a-b)}
    else if a<b {return Some(b-a)}
    else {return None}
}

fn mul(a:i64,b:i64) -> Option<i64> {
    if b!=1 && a!=1 {return Some(a*b)}
    else {return None}
}

fn div(a:i64,b:i64) -> Option<i64> {
    if a > b {
	if b != 1 && a%b == 0 {return Some(a/b)}
	else {return None}
    }
    else {
	if a != 1 && b%a == 0 {return Some(b/a)}
	else {return None}
    }
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
use core::cmp::{min,max};

fn print_tree(t:Computation) {
    match t {
	Node(i,a,b,t2) => {
	    let (a,b)=(max(a,b),min(a,b));
	    println!("{}{}{}={}",a,OPSN[i],b,OPS[i](a,b).unwrap());
	    print_tree(*t2)
	},
	End|Nothing => {}
    }
}

fn test(a:&Vec<i64>, goal:i64) -> Computation {
    let mut newa = Vec::with_capacity(a.len()-1);
    for i in 0..a.len() {
	for j in i+1..a.len() {
	    for k in 0..OPS.len() {
		match OPS[k](a[i],a[j]) {
		    Some (r) => {
			if r == goal {return Node(k,a[i],a[j],Box::new(End))}
			if a.len() > 2 {
			    newa.clear();
			    for m in 0..a.len() {if m!=i && m!=j {newa.push(a[m])}}
			    newa.push(r);
			    let res = test(&newa,goal);
			    match  res {
				Node(..) => return Node(k,a[i],a[j],Box::new(res)),
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

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let g = args[args.len()-1].parse().unwrap();
    let mut t = Vec::with_capacity(args.len()-2);
    for i in 1..args.len()-1 {t.push(args[i].parse().unwrap())}
    print_tree(test(&t,g));
}
