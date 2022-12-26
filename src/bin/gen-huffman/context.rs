use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Context {
    root: Node,
}

impl Context {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn add(&mut self, sym: usize, code: &mut VecDeque<bool>) {
        self.root.add(sym, code);
    }

    pub fn to_decode(&mut self) -> String {
        // id transition nodes
        let mut id = 0;
        self.root.set_id(&mut id, &mut vec![]);

        // record decoding states by transitions
        self.root.build_transition_table(&self.root);

        // persist transition states
        let mut out = String::from("pub const DECODE_TABLE: [[(usize, u8); 16]; 256] = [\n");
        self.root.put_str(&mut out);
        out.push_str("];\n");

        out
    }
}

#[derive(Default)]
struct Node {
    id: Option<usize>,
    sym: Option<usize>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    accept: bool,
    transitions: RefCell<Vec<(Option<usize>, Option<usize>, bool)>>,
}

impl Node {
    fn new() -> Self {
        Self::default()
    }

    // add nodes to prefix tree
    fn add(&mut self, sym: usize, code: &mut VecDeque<bool>) {
        if code.is_empty() {
            self.sym = Some(sym);
            return;
        }

        let bit = code.pop_front().unwrap();

        let node = if bit {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new()));
            }
            &mut self.right
        } else {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new()));
            }
            &mut self.left
        };

        if let Some(ref mut node) = node {
            node.add(sym, code);
        }
    }

    // identify transition nodes
    fn set_id(&mut self, id: &mut usize, prefix: &mut Vec<bool>) {
        if self.sym.is_some() {
            return;
        }

        if prefix.len() < 8 && prefix.iter().all(|b| *b) {
            self.accept = true;
        }

        self.id = Some(*id);
        *id += 1;

        if let Some(ref mut node) = self.left {
            prefix.push(false);
            node.set_id(id, prefix);
            prefix.pop();
        }

        if let Some(ref mut node) = self.right {
            prefix.push(true);
            node.set_id(id, prefix);
            prefix.pop();
        }
    }

    fn build_transition_table(&self, root: &Node) {
        self.do_build_transition_table(root, self, None, 4);

        if let Some(ref node) = self.left {
            node.build_transition_table(root);
        }

        if let Some(ref node) = self.right {
            node.build_transition_table(root);
        }
    }

    // record decoding states by 4 transitions
    fn do_build_transition_table(
        &self,
        root: &Node,
        start: &Node,
        sym: Option<usize>,
        steps_left: usize,
    ) {
        if steps_left == 0 {
            let (id, sym) = match sym {
                Some(256) => (None, None),
                _ => (Some(self.id.unwrap_or(0)), sym),
            };

            start.transitions.borrow_mut().push((id, sym, self.accept));

            return;
        }

        let next = if self.sym.is_none() { self } else { root };

        for node in &[next.left.as_ref().unwrap(), next.right.as_ref().unwrap()] {
            let sym = if let Some(s) = node.sym { Some(s) } else { sym };
            node.do_build_transition_table(root, start, sym, steps_left - 1);
        }
    }

    fn put_str(&self, out: &mut String) {
        const ACCEPTED: usize = 1 << 12;
        const SYM: usize = 1 << 13;
        const ERROR: usize = 1 << 14;

        if self.sym.is_some() {
            return;
        }

        out.push_str(&format!("    // {}\n", self.id.unwrap()));
        out.push_str("    [\n");

        for trans in self.transitions.borrow().iter() {
            let (id, sym, accept) = trans;

            let mut flags = 0;

            let id = if let Some(id) = id {
                *id
            } else {
                flags |= ERROR;
                0
            };

            let sym = if let Some(sym) = sym {
                flags |= SYM;
                *sym
            } else {
                0
            };

            if *accept {
                flags |= ACCEPTED
            }

            out.push_str(&format!("        (0x{:04x}, {}),\n", flags | id, sym));
        }

        out.push_str("    ],\n");

        self.left.as_ref().unwrap().put_str(out);
        self.right.as_ref().unwrap().put_str(out);
    }
}
