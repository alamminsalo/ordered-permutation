extern crate ordered_permutation;
use ordered_permutation as op;

fn main() {
    let mut node = op::Node::new(vec![0]);
    
    for i in 1..7 {
        node.insert(i);
    }
    
    let result  = node.extract();

	for v in result.iter().map(|x| &x[1..]) {
		if v.len() > 1 {
			println!("{:?}", v);
		}
	}
}


