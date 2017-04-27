struct Node {
    nums: Vec<u8>,
    children: Vec<Node>
}

impl Node {
    pub fn new(nums: Vec<u8>) -> Node {
        Node{nums: nums, children: vec![]}
    }

    pub fn insert(&mut self, num: u8) {
        if self.nums[self.nums.len()-1] < num {

		let mut v = vec![];
		v.extend(&self.nums);
		v.push(num);
            self.children.push(Node::new(v));

            for mut node in self.children.iter_mut() {
                node.insert(num);
            }
        }
    }

    pub fn extract(&self) -> Vec<Vec<u8>> {
		let mut v: Vec<Vec<u8>> = vec![];
		v.push(self.nums.clone());
		for node in self.children.iter() {
			v.extend(node.extract());
		}
		v
    }
}

fn main() {
    let mut node = Node::new(vec![0]);
    
    for i in 1..10 {
        node.insert(i);
    }
    
    let result  = node.extract();

	for v in result.iter().map(|x| &x[1..]) {
		if v.len() > 1 {
			println!("{:?}", v);
		}
	}
}


