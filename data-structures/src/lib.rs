#[derive(Debug)]
pub struct Node<T: Ord> {
    pub value: T,
    pub right: Option<Box<Node<T>>>,
    pub left: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => {
                    self.left = Some(Box::new(Node {
                        value,
                        right: None,
                        left: None,
                    }))
                }
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => {
                    self.right = Some(Box::new(Node {
                        value,
                        right: None,
                        left: None,
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tree() {
        let cypress = Node {
            value: "cypress",
            right: None,
            left: None,
        };
        let oak = Node {
            value: "oak",
            right: None,
            left: None,
        };
        let elm = Node {
            value: "elm",
            right: Some(Box::new(cypress)),
            left: Some(Box::new(oak)),
        };
        assert_eq!(elm.left.unwrap().value, "oak");
        assert_eq!(elm.right.unwrap().value, "cypress");
    }

    #[test]
    fn it_inserts_a_lower_value_to_the_left() {
        let mut root = Node {
            value: 1,
            right: None,
            left: None,
        };
        root.insert(0);
        assert_eq!(root.left.unwrap().value, 0);
    }

    #[test]
    fn it_inserts_a_higher_value_to_the_right() {
        let mut root = Node {
            value: 1,
            right: None,
            left: None,
        };
        root.insert(2);
        assert_eq!(root.right.unwrap().value, 2);
    }
}
