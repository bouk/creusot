enum Color {
    Red,
    Black,
}

struct Node<K, V> {
    left: Option<Box<Node<K, V>>>,
    color: Color,
    key: K,
    val: V,
    right: Option<Box<Node<K, V>>>,
}

struct Tree<K, V> {
    node: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
  fn rotate_right(&mut self) {
    //     self
    //    /    \
    //   x      c
    //  / \
    // a   b
    // Rip out the left subtree
    let mut x : Box<_> = match std::mem::take(&mut self.left) {
      Some(x) => x,
      None => return,
    };
    //     self
    //         \
    //   x      c
    //  / \
    // a   b
    std::mem::swap(&mut self.left, &mut x.right);
    //        self
    //       /    \
    //   x  b      c
    //  /
    // a
    std::mem::swap(self, &mut x);
    //   self
    //  /
    // a      x
    //       / \
    //      b   c
    self.right = Some(x);
    //   self
    //  /    \
    // a      x
    //       / \
    //      b   c
  }

  fn rotate_left(&mut self) {
    let mut x : Box<_> = match std::mem::take(&mut self.right) {
      Some(x) => x,
      None => return,
    };
    std::mem::swap(&mut x.right, &mut self.left);
    std::mem::swap(self, &mut x);
    self.left = Some(x);
  }
}

impl<K: Ord, V> Tree<K, V> {
    fn insert(&mut self, k: K, v: V) {
        let mut needle = &mut self.node;

        while let Some(node) = needle {
            if k < node.key {
                needle = &mut node.left;
            } else {
                needle = &mut node.right;
            }
        }

        *needle =
            Some(Box::new(Node { left: None, key: k, val: v, right: None, color: Color::Red }));

        self.fixup();
    }

    fn fixup(&mut self) {
      let mut needle = &mut self.node;

      while let Some(node) = needle {
        if let Color::Black = node.color { break }


        if
      }
    }
}


