extern crate creusot_contracts;
use creusot_contracts::*;

#[derive(Clone, Copy)]
enum Color {
    Red,
    Black,
}

enum Tree<K, V> {
    Node { color: Color, left: Box<Tree<K, V>>, key: K, val: V, right: Box<Tree<K, V>> },
    Leaf,
}

impl Color {
    fn flip(self) -> Self {
        match self {
            Color::Black => Color::Red,
            Color::Red => Color::Black,
        }
    }
}

impl<K: Ord, V> Tree<K, V> {
    fn rotate_left(&mut self) {
        if let Tree::Node { right, .. } = self {
            let mut child : Self = std::mem::replace(&mut *right, Tree::Leaf);

            if let Tree::Node { left, .. } = &mut child {
                std::mem::swap(right, left );
                std::mem::swap(self, &mut child);
                **left = child;
            }
        }
    }

    // fn rotate_right(&mut self) {
    //     if let Some(node) = self.0 {
    //         let left = std::mem::take(&mut node.left.0);
    //         if let Some(left) = left {
    //             std::mem::swap(&mut left.right, &mut node.left);

    //             std::mem::swap(self, &mut left.right);
    //         }
    //     }
    // }

    // fn get(&self, k: &K) -> Option<&V> {
    //     let mut node = self;

    //     while let Some(x) = node.0 {
    //         match x.key.cmp(k) {
    //             Ordering::Less => node = &x.left,
    //             Ordering::Equal => return Some(&x.val),
    //             Ordering::Greater => node = &x.right,
    //         }
    //     }

    //     return None;
    // }

    // fn insert(&mut self, key: K, val: V) {
    //     if let None = self.0 {
    //         *self = Tree(Some(Box::new(Node {
    //             color: Color::Red,
    //             left: Tree(None),
    //             key,
    //             val,
    //             right: Tree(None),
    //         })));
    //         return;
    //     }
    //     let node = self.unwrap_mut();
    //     match key.cmp(&node.key) {
    //         Ordering::Less => {
    //             node.left.insert(key, val);
    //         }
    //         Ordering::Equal => {
    //             node.val = val;
    //         }
    //         Ordering::Greater => {
    //             node.left.insert(key, val);
    //         }
    //     }

    //     if node.right.is_red() && !node.left.is_red() {
    //         self.rotate_left();
    //     }

    //     //   if self.left.is_red() && !self.left.unwrap().left.is_red() {
    //     //     self.rotate_right();
    //     //   }

    //     //   if self.left.is_red() && self.right.is_red() {
    //     //     self.flip_colors();
    //     //   }
    // }

    // fn flip_colors(&mut self) {
    //     let node = self.unwrap_mut();
    //     node.color = node.color.flip();

    //     let left = node.left.unwrap_mut();
    //     left.color = left.color.flip();

    //     let right = node.right.unwrap_mut();
    //     right.color = right.color.flip();
    // }

    // fn unwrap(&self) -> &Node<K, V> {
    //     self.0.as_ref().unwrap()
    // }

    // fn unwrap_mut(&mut self) -> &mut Node<K, V> {
    //     self.0.as_mut().unwrap()
    // }

    // fn is_red(&self) -> bool {
    //     match self.0 {
    //         None => false,
    //         Some(n) => matches!(n.color, Color::Red),
    //     }
    // }

    // fn is_black(&self) -> bool {
    //     match self.0 {
    //         None => true,
    //         Some(n) => matches!(n.color, Color::Black),
    //     }
    // }
}
