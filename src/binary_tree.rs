pub struct Node {
    pub val: i32,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
}

impl Node {
    pub fn root(val: i32) -> Self {
        Node { val: val, l: None, r: None }
    }

    pub fn insert(&mut self, x: i32) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_values() {
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
}