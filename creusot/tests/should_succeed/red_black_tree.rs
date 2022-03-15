extern crate creusot_contracts;
use creusot_contracts::*;

#[derive(Clone, Copy)]
enum Color { Red, Black }

struct Node<K,V> {
  color: Color,
  left: Tree<K,V>,
  key: K,
  val: V,
  right: Tree<K,V>,
}

impl Color {
  fn flip(self) -> Self {
    match self {
      Color::Black => Color::Red,
      Color::Red => Color::Black,
    }
  }
}

struct Tree<K, V>(Option<Box<Node<K, V>>>);

impl<K : Ord, V> Tree<K, V> {
  fn rotate_left(&mut self) {
    if let Some(node) = self.0 {
      let right = std::mem::take(&mut node.right.0);
      if let Some(right) = right {
        std::mem::swap(&mut right.left, &mut node.right);

        std::mem::swap(self, &mut right.left);
      }
    }
  }

  fn rotate_right(&mut self) {
    if let Some(node) = self.0 {
      let left = std::mem::take(&mut node.left.0);
      if let Some(left) = left {
        std::mem::swap(&mut left.right, &mut node.left);

        std::mem::swap(self, &mut left.right);
      }
    }
  }


  fn get(&self, k: &K) -> Option<&V> {
    let mut node = self;

    while let Some(x) = node.0 {
      match x.key.cmp(k) {
        Ordering::Less => node = &x.left,
        Ordering::Equal => return Some(&x.val),
        Ordering::Greater => node = &x.right,
      }
    }

    return None;
  }

  // fn insert(&mut self, key: K, val: V) {
  //   if let None = self {
  //     *self = Some(Node {
  //       color: Color::Red,
  //       left: None,
  //       key, val,
  //       right: None
  //     });
  //     return
  //   }

  //   match key.cmp(self.key) {
  //     Less => {
  //     self.left.insert(key, val);
  //     }
  //     Equal => {self.val = val; }
  //     Greater => {
  //       self.left.insert(key, val);
  //     }
  //   }

  //   if self.right.is_red() && !self.left.is_red() {
  //     self.rotate_left();
  //   }

  //   if self.left.is_red() && !self.left.unwrap().left.is_red() {
  //     self.rotate_right();
  //   }

  //   if self.left.is_red() && self.right.is_red() {
  //     self.flip_colors();
  //   }
  // }

  // fn flip_colors(&mut self) {
  //   self.color = self.color.flip();

  //   let left = self.left.unwrap_mut();
  //   *left.color = left.color.flip();

  //   let right = self.right.unwrap_mut();
  //   *right.color = right.color.flip();
  // }

  // fn unwrap(&self) -> &Self {
  //   self.0.as_ref().unwrap()
  // }

  // fn unwrap_mut(&mut self) -> &mut Self {
  //   self.0.as_ref().unwrap()
  // }


  // fn is_red(&self) -> bool {
  //   // matches!(self.0, Some(Node { color: Color::Red, .. }))
  //   false
  // }

  // fn is_black(&self) -> bool {
  //   // matches!(self.0, Some(Node { color: Color::Black, .. }))
  //   false
  // }

}
