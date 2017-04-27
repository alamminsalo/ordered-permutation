extern crate ordered_permutation;
use ordered_permutation as op;

fn main() {
    let result  = op::permutate(&vec![1,2,3,4,5]);

	for v in result.iter() {
		println!("{:?}", v);
	}
}


