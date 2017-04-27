// check that results are unique
#[cfg(test)]
#[test]
fn test_unique() {
	let a = permutate(&vec![1,2,3,4,5]);

	let mut b: Vec<Vec<u8>> = vec![];

	for v in a.iter() {
		assert!(!b.contains(&v));
		b.push(v.clone());
	}

	// ok
}

pub fn permutate(source: &Vec<u8>) -> Vec<Vec<u8>> {
	let mut root = Node::new(vec![]);
	for val in source.iter() {
		root.insert(*val);
	}
	
	root.extract()
}

struct Node {
    nums: Vec<u8>,
    children: Vec<Node>
}

impl Node {
    fn new(nums: Vec<u8>) -> Node {
        Node{nums: nums, children: vec![]}
    }

    fn insert(&mut self, num: u8) {
        if self.nums.len() == 0 || self.nums[self.nums.len()-1] < num {

		let mut v = vec![];
		v.extend(&self.nums);
		v.push(num);
            self.children.push(Node::new(v));

            for mut node in self.children.iter_mut() {
                node.insert(num);
            }
        }
    }

    fn extract(&self) -> Vec<Vec<u8>> {
		let mut v: Vec<Vec<u8>> = vec![];
		if self.nums.len() > 0 {
			v.push(self.nums.clone());
		}
		for node in self.children.iter() {
			v.extend(node.extract());
		}
		v
    }
}

