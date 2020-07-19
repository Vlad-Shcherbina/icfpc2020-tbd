#[derive(Debug)]
pub enum Tree<T> {
    Leaf(T),
    App(Box<Tree<T>>, Box<Tree<T>>),
}

#[derive(PartialEq, Clone)]
pub enum VecTree<T> {
    Leaf(T),
    Branch(Vec<VecTree<T>>),
    List(Vec<VecTree<T>>)
}

impl<T> Tree<T> {
    // We assume that application token is represented as None.
    pub fn parse(tokens: &mut dyn Iterator<Item=Option<T>>) -> Self {
        match tokens.next().unwrap() {
            Some(t) => Tree::Leaf(t),
            None => Tree::App(
                Box::new(Tree::parse(tokens)),
                Box::new(Tree::parse(tokens))),
        }
    }

    pub fn parse_many(tokens: &mut dyn Iterator<Item=Option<T>>) -> Vec<Self> {
        let mut tokens = tokens.peekable();
        let mut result = Vec::new();
        while  tokens.peek().is_some() {
            result.push(Tree::parse(&mut tokens));
        }
        result
    }

    pub fn to_haskell(&self, render_token: &dyn Fn(&T) -> String, parenthesize: bool) -> String {
        let mut t = self;
        let mut seq: Vec<&Self> = Vec::new();
        while let Tree::App(left, right) = t {
            t = left;
            seq.push(right);
        }
        seq.push(t);
        seq.reverse();
        let mut result = String::new();

        for (i, q) in seq.into_iter().enumerate() {
            if i > 0 {
                result.push(' ');
            }
            match q {
                Tree::Leaf(token) => result.push_str(&render_token(token)),
                Tree::App(_, _) => result.push_str(&q.to_haskell(render_token, true)),
            }
        }
        if parenthesize {
            result = format!("({})", result);
        }
        result
    }
}

impl VecTree<&str> {
    pub fn find_lists(self) -> Self {
        match self {
            VecTree::Leaf(_) => self,
            VecTree::Branch(branch) => {
                let mut flag = true;
                let mut correct_list = true;
                let mut list: Vec<VecTree<&str>> = Vec::new();

                let mut current_branch = &branch;
                while flag {
                    if current_branch.len() != 3 {
                        correct_list = false;
                        break;
                    }
                    let head = &current_branch[1];
                    match &current_branch[2] {
                        VecTree::Leaf("nil") => {
                            list.push(head.clone().find_lists());
                            flag = false
                        }
                        VecTree::Branch(new_branch) => {
                            list.push(head.clone().find_lists());
                            current_branch = new_branch; 
                        }
                        _ => {
                            correct_list = false;
                            break;
                        }
                    }
                }

                if correct_list {
                    VecTree::List(list)
                } else {
                    VecTree::Branch(branch)
                }
            },
            _ => self // dummy
        }
    }

    pub fn to_string(&self, parenthesize: bool) -> String {
        match self {
            VecTree::Leaf(token) => token.to_string(),
            VecTree::List(vector) => {
                let mut result = String::new();
                for (i, q) in vector.into_iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&q.to_string(true))
                }
                result = format!("[{}]", result);
                result
            }
            VecTree::Branch(vector) => {
                if vector.len() == 2 && VecTree::Leaf("neg") == vector[0] {
                    match vector[1] {
                        VecTree::Leaf(number) => return format!("-{}", number),
                        _ => {}
                    }
                }
                let mut result = String::new();
                for (i, q) in vector.into_iter().enumerate() {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push_str(&q.to_string(true))
                }
                if parenthesize {
                    result = format!("({})", result)
                }
                result
            }
        }
    }
}

impl Tree<&str> {
    fn to_vec_tree(&self) -> VecTree<&str> {
        let mut t = self;
        let mut seq: Vec<&Self> = Vec::new();
        while let Tree::App(left, right) = t {
            t = left;
            seq.push(right);
        }
        seq.push(t);
        seq.reverse();

        let mut result = Vec::<VecTree<&str>>::new();

        for q in seq.into_iter() {
            match q {
                Tree::Leaf(token) => result.push(VecTree::Leaf(token)),
                Tree::App(_, _) => result.push(q.to_vec_tree())
            }
        }

        VecTree::Branch(result)
    }

    pub fn better_to_haskell(&self) -> String {
        self.to_vec_tree().find_lists().to_string(false)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_to_hs() {
        let app = None;

        let tokens = vec![app, app, Some("f"), Some("x"), Some("y")];
        let tree = Tree::parse(&mut tokens.into_iter());
        let hs = tree.to_haskell(&|t| t.to_string(), false);
        assert_eq!(hs, "f x y");

        let tokens = vec![app, Some("f"), app, Some("x"), Some("y")];
        let tree = Tree::parse(&mut tokens.into_iter());
        let hs = tree.to_haskell(&|t| t.to_string(), false);
        assert_eq!(hs, "f (x y)");
    }
}
