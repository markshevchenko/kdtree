use std::rc::{Rc, Weak};
use std::cmp::{Ordering};

struct Node {
    value: char,
    point: [f64; 2],
    parent: Weak<Node>,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

pub struct KDTree {
    root: Option<Rc<Node>>
}

impl KDTree {
    pub fn new(items: Vec<(char, [f64; 2])>) -> Self {
        let mut items: Vec<_> = items.into_iter().map(Some).collect();

        KDTree {
            root: Self::build(&mut items, 0, Weak::new())
        }
    }

    fn build(
        items: &mut [Option<(char, [f64; 2])>],
        depth: usize,
        parent: Weak<Node>,
    ) -> Option<Rc<Node>> {
        if items.is_empty() {
            return None
        }

        let axis = depth % 2;
        let mid = items.len() / 2;
        let (left, median, right) = items.select_nth_unstable_by(
            mid,
            |a, b| a.unwrap().1[axis].partial_cmp(&b.unwrap().1[axis])
                                                                                 .unwrap_or(Ordering::Equal)
        );

        let median = median.take().unwrap();
        Some(Rc::new_cyclic(|me| Node {
            value: median.0,
            point: median.1,
            parent,
            left: Self::build(left, depth + 1, me.clone()),
            right: Self::build(right, depth + 1, me.clone())
        }))
    }

    pub fn export<'a>(&'a self) -> Vec<(&'a char, [f64; 2], Option<usize>, Option<usize>)> {
        let mut result = vec![];

        if let Some(root) = self.root.as_deref() {
            let _ = Self::fold(root, &mut result);
        }

        result
    }

    fn fold<'a>(
        node: &'a Node,
        result: &mut Vec<(&'a char, [f64; 2], Option<usize>, Option<usize>)>
    ) -> usize {
        let current_index = result.len();
        result.push((&node.value, node.point, None, None));

        if let Some(left) = node.left.as_deref() {
            result[current_index].2 = Some(Self::fold(left, result));
        }

        if let Some(right) = node.right.as_deref() {
            result[current_index].3 = Some(Self::fold(right, result));
        }

        current_index
    }
}

