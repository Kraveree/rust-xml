//use std::iter; 
//struct Node {
//    children: Vec<Node>,
//    data: usize,
//}

//impl Node {

//    pub fn new() -> Node {
//        Node {
//            children: vec!(),
//            data: 0
//        }
//    }

//    pub fn expand(&mut self) {
//        self.children = vec!(Node::new(), Node::new());
//    }

//    pub fn is_leaf(&self) -> bool {
//        self.children.len() == 0
//    }

//    fn expand_leaf_and_inc(&mut self) {
//        if self.is_leaf() {
//            self.expand();
//        } else {
//            let index = 0;
//            self.children[index].expand_leaf_and_inc();
//        }
//        self.data += 1
//    }
//}

//pub fn main() {
//    let mut root = Node::new();
//    for _ in 0..10 {
//        root.expand_leaf_and_inc();
//    }
//}



trait Hierarchical<T> {
	fn new(data: T) -> Self;
	fn get_data(&self) -> &T;
	fn add_child(&mut self, data: T) -> &mut Self;
	fn get_child(&mut self, index: usize) -> &mut Self;
	fn get_child_save(&mut self, index: usize) -> Option<&mut Self>;
	fn get_child_data(&mut self, index: usize) -> &T;
	fn get_count(&self) -> usize;
}


#[derive(Debug, PartialEq)]
struct Node<T> {
	children: Vec<Node<T>>,
	data: T,
}



#[derive(Debug, PartialEq)]
struct Item {
	index: usize,
}



impl<T> IntoIterator for Node<T> {
	type Item = Node<T>;
	type IntoIter = NodeIntoIterator<T>;

	fn into_iter(self) -> Self::IntoIter {
		NodeIntoIterator::new(&self)
	}

}


impl<T> Iterator for NodeIntoIterator<T> {
	type Item = Node<T>;
	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}


struct NodeIntoIterator<T> {
	nodelist: Vec<Node<T>>,
}


impl<T> NodeIntoIterator<T> {
	fn new(node: &Node<T>) -> NodeIntoIterator<T> {
		NodeIntoIterator {
			nodelist: vec![],
		}
	}
}



impl<T> Hierarchical<T> for Node<T> {
	fn new(data: T) -> Self {
		Node {
			children: vec!(),
			data: data,
		}
	}

	fn add_child(&mut self, data: T) -> &mut Self {
		let child = Self::new(data);
		self.children.push(child);
		let count: usize;
		{
			count = self.get_count()-1; 
		}
		&mut self.children[count]
	}

	fn get_data(&self) -> &T {
		&self.data
	}

	fn get_child(&mut self, index: usize) -> &mut Self {
		&mut self.children[index]
	}

	fn get_child_data(&mut self, index: usize) -> &T {
		&self.children[index].get_data()
	}

	fn get_child_save(&mut self, index: usize) -> Option<&mut Self> {
		if index < self.children.len() {
			Some(&mut self.children[index])
		}
		else {
			None
		}
	}
	    
	fn get_count(&self) -> usize {
		self.children.len()
	}
}



#[cfg(test)]
mod tests {
	use super::Hierarchical;
	use super::Node;
	use super::Item;

	#[test]
	fn new_node_test() {
		let item = Item { index: 0 };
		let hierarchy = Node::new(item);
		assert_eq!(hierarchy.get_data(), &Item{ index: 0 });
	}

	#[test]
	fn add_child_test() {
		let item = Item { index: 0 };
		assert_eq!(Node::new(item).add_child(Item { index: 1 } ).get_data(), &Item{ index: 1 });
	}

	#[test]
	fn add_children_test() {
		let root  = Item { index: 0 };
//		let item1 = Item { index: 1 };
//		let item2 = Item { index: 2 };
//		let item3 = Item { index: 3 };
		let mut hierarchy = Node::new(root);
//		let mut child1 = hierarchy.add_child(item1);
//		let mut child2 = child1.add_child(item2);
//		let mut child3 = child2.add_child(item3);
		assert_eq!(hierarchy.get_data(), &Item{ index: 0 });
		hierarchy.add_child(Item{ index: 1 });
		assert_eq!(hierarchy.get_count(), 1);
		assert_eq!(hierarchy.get_child_save(0).unwrap().get_data(), &Item{ index: 1 } );
		assert_eq!(hierarchy.get_child(0).get_data(), &Item{ index: 1 } );
		assert_eq!(hierarchy.get_child_data(0), &Item{ index: 1 } );
		hierarchy.get_child(0).add_child( Item{ index: 2 } );
		assert_eq!(hierarchy.get_child_save(0).unwrap().get_child_save(0).unwrap().get_data(), &Item{ index: 2 });
		assert_eq!(hierarchy.get_child(0).get_child(0).get_data(), &Item{ index: 2 });
		assert_eq!(hierarchy.get_child(0).get_child_data(0), &Item{ index: 2 });
	}
}
