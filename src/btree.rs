#[derive(Default, Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Default, Debug)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl From<Node> for Option<Box<Node>> {
    fn from(value: Node) -> Self {
        Some(Box::new(value))
    }
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, val: i32) {
        match &mut self.root {
            None => self.root = Node::new(val).into(),
            Some(node) => Self::recursive_insert(node, val),
        }
    }

    fn insert_iterative(&mut self, val: i32) {
        if self.root.is_none() {
            self.root = Node::new(val).into();
            return;
        }

        let mut q: Vec<&mut Box<Node>> = Vec::new();
        q.push(self.root.as_mut().unwrap());

        while let Some(node) = q.pop() {
            if node.val > val {
                let left = &mut node.left;
                match left {
                    None => *left = Node::new(val).into(),
                    Some(n) => q.push(n),
                }
                continue;
            }
            let right = &mut node.right;
            match right {
                None => *right = Node::new(val).into(),
                Some(n) => q.push(n),
            }
            continue;
        }
    }

    fn recursive_insert(node: &mut Node, val: i32) {
        if node.val > val {
            match &mut node.left {
                None => node.left = Node::new(val).into(),
                Some(left) => Self::recursive_insert(left, val),
            }
            return;
        }
        match &mut node.right {
            None => node.right = Node::new(val).into(),
            Some(right) => Self::recursive_insert(right, val),
        }
        return;
    }
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert_iterative(3);
        tree.insert(2);
        tree.insert(4);
        tree.insert_iterative(6);
        tree.insert(0);

        println!("{:?}", tree);
    }
}
