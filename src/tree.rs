#[derive(Debug)]
pub enum Tree<T> {
    Leaf(T),
    App(Box<Tree<T>>, Box<Tree<T>>),
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
