#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
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

