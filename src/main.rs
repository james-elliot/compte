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
    End
}
use crate::Computation::{End,Node};
use core::cmp::{min,max};
fn print_tree(t:Computation) {
    match t {
	Node(i,a,b,t2) => {
	    let f=OPS[i];
	    let (a,b)=(max(a,b),min(a,b));
	    println!("{}{}{}={}",a,OPSN[i],b,f(a,b).unwrap());
	    print_tree(*t2)
	},
	End => {}
    }
}

fn test(a:&Vec<i64>, goal:i64) -> Option<Computation> {
    let mut newa = Vec::with_capacity(a.len()-1);
    for i in 0..a.len() {
	for j in i+1..a.len() {
	    for k in 0..OPS.len() {
		match OPS[k](a[i],a[j]) {
		    Some (r) => {
			if r == goal {return Some(Node(k,a[i],a[j],Box::new(End)))}
			if a.len() > 2 {
			    let mut m=0;
			    newa.clear();
			    for _l in 0..newa.capacity()-1 {
				if m!=i && m!=j {newa.push(a[m])}
				m=m+1;
			    }
			    newa.push(r);
			    match test(&newa,goal) {
				Some(v) => return Some(Node(k,a[i],a[j],Box::new(v))),
				None => {}
			    }
			}
		    }
		    None => {}
		}
	    }
	}
    }
    return None
}

fn main() {
    let t = [1,2,3,4,5,6,7,8,9,10,11];
    let g = 99999;
    let res = test(&t.to_vec(),g);
    match res {
	None=>{}
	Some(t) => {
	    print_tree(t);
	}
    }
}
