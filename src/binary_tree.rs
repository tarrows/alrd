pub struct Node<T> {
    pub val: T,
    pub l: Option<Box<Node<T>>>,
    pub r: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Copy> Node<T> {
    pub fn root(val: T) -> Self {
        Node { val: val, l: None, r: None }
    }

    pub fn insert(&mut self, x: T) {
        if self.val == x {
            return
        }

        let child = if x < self.val {
            &mut self.l
        } else {
            &mut self.r
        };

        match child {
            Some(n) => n.insert(x),
            None => {
                let node = Node { val: x, l: None, r: None };
                let boxed_node = Some(Box::new(node));
                *child = boxed_node;
            }
        }
    }

    pub fn find(&self, x: T) -> Option<T> {
        if x < self.val {
            match &self.l {
                Some(n) => n.find(x),
                None => None,
            }
        } else if x > self.val {
            match &self.r {
                Some(n) => n.find(x),
                None => None,
            }
        } else {
            Some(x)
        }
    }

    pub fn min(&self) -> T {
        match &(self.l) {
            Some(n) => n.min(),
            None => self.val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_int() {
        let mut root = Node::root(2);
        root.insert(1);
        root.insert(2);
        root.insert(3);

        assert!(match root.l {
            Some(child) => child.val == 1,
            None => false,
        });
        assert!(match root.r {
            Some(child) => child.val == 3,
            None => false,
        }); 
    }

    #[test]
    fn test_tree_str() {
        let mut root = Node::root("egg");
        root.insert("banana");
        root.insert("chocolate");
        root.insert("apple");
        root.insert("lemon");

        assert!(match root.l {
            Some(child) => child.val == "banana" && match child.l {
                Some(grandchild) => grandchild.val == "apple",
                None => false,
            } && match child.r {
                Some(grandchild) => grandchild.val == "chocolate",
                None => false,
            },
            None => false,
        })
    }

    #[test]
    fn test_find_str() {
        let mut tree = Node::root("egg");
        tree.insert("banana");
        tree.insert("chocolate");
        tree.insert("apple");
        tree.insert("lemon");

        let x = tree.find("apple");
        assert!(x == Some("apple"));
        let y = tree.find("orange");
        assert!(y == None);
    }

    #[test]
    fn test_min_int() {
        let mut tree = Node::root(5);
        tree.insert(8);
        tree.insert(3);
        tree.insert(1);
        tree.insert(4);
        tree.insert(2);

        let x = tree.min();
        assert!(x == 1);
    }
}
