// Classic Trie without space optimization.

use super::Char;

pub struct Trie<T> {
    root: Node<T>,
}

enum Link<T> {
    Node(Box<Node<T>>),
    Leaf(Box<Leaf<T>>),
}
impl<T> Link<T> {
    const EMPTY: Option<Link<T>> = None;
}

struct Leaf<T>(T);

struct Node<T> {
    children: [Option<Link<T>>; Char::ALPHABET],
    value: Option<T>,
}

impl<T> Trie<T> {
    pub fn new() -> Trie<T> {
        Trie {
            root: Node {
                children: [Link::EMPTY; Char::ALPHABET],
                value: None,
            },
        }
    }

    pub fn find(&self, key: &[Char]) -> Option<&T> {
        let mut curr = &self.root;
        for c in &key[..key.len() - 1] {
            match &curr.children[c.index()] {
                Some(Link::Node(child)) => curr = child.as_ref(),
                Some(Link::Leaf(_)) => return None,
                None => return None,
            }
        }
        match &curr.children[key[key.len() - 1].index()] {
            Some(Link::Node(child)) => child.value.as_ref(),
            Some(Link::Leaf(child)) => Some(&child.0),
            None => return None,
        }
    }

    pub fn insert(&mut self, key: &[Char], value: T) {
        let mut curr = &mut self.root;
        for c in &key[..key.len() - 1] {
            let child = match curr.children[c.index()].take() {
                Some(Link::Node(child)) => child,
                Some(Link::Leaf(child)) => Box::new(Node {
                    children: [Link::EMPTY; Char::ALPHABET],
                    value: Some(child.0), // keep old value of leaf
                }),
                None => Box::new(Node {
                    children: [Link::EMPTY; Char::ALPHABET],
                    value: None,
                }),
            };

            curr.children[c.index()] = Some(Link::Node(child));
            match curr.children[c.index()].as_mut() {
                Some(Link::Node(child)) => curr = child,
                _ => unreachable!(),
            }
        }

        match curr.children[key[key.len() - 1].index()].as_mut() {
            Some(Link::Node(child)) => child.value = Some(value), // replace
            Some(Link::Leaf(child)) => child.0 = value,           // replace
            None => {
                curr.children[key[key.len() - 1].index()] = Some(Link::Leaf(Box::new(Leaf(value))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Chars;
    use super::Trie;

    #[test]
    fn test_trie() {
        struct TestCase<'a> {
            key: &'a str,
            value: i32,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                key: "in",
                value: 1,
            },
            TestCase {
                key: "inside",
                value: 2,
            },
            TestCase {
                key: "hello",
                value: 3,
            },
            TestCase {
                key: "he",
                value: 4,
            },
            TestCase {
                key: "her",
                value: 5,
            },
        ];

        let mut trie: Trie<i32> = Trie::new();
        for i in 0..cases.len() {
            trie.insert(
                Chars::try_from(cases[i].key).unwrap().as_ref(),
                cases[i].value,
            );
            for j in 0..=i {
                let v = trie.find(Chars::try_from(cases[j].key).unwrap().as_ref());
                assert_eq!(v, Some(&cases[j].value));
            }
        }
    }
}
