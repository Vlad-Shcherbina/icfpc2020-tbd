#[derive(PartialEq, Clone)]
pub enum Data {
    Nil,
    Number(i64),
    Cons(Box<Data>, Box<Data>)
}

impl From<i64> for Data {
    fn from(x: i64) -> Self {
        Data::Number(x)
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pretty_string())
    }
}

impl Data {
    pub fn make_cons(head: impl Into<Data>, tail: impl Into<Data>) -> Data {
        Data::Cons(Box::new(head.into()), Box::new(tail.into()))
    }

    pub fn make_list1(x: impl Into<Data>) -> Data {
        Data::make_cons(x, Data::Nil)
    }

    pub fn make_list2(x: impl Into<Data>, y: impl Into<Data>) -> Data {
        Data::make_cons(x, Data::make_list1(y))
    }

    pub fn make_list3(x: impl Into<Data>, y: impl Into<Data>, z: impl Into<Data>) -> Data {
        Data::make_cons(x, Data::make_list2(y, z))
    }

    pub fn to_string(&self) -> String {
        match self {
            Data::Nil => String::from("nil"),
            Data::Number(x) => {
                format!("{}", x)
            }
            Data::Cons(head, tail) => {
                format!("({}, {})", head.to_string(), tail.to_string())
            }
        }
    }

    pub fn to_pretty_string(&self) -> String {
        if self.is_list() {
            let mut elements = Vec::new();
            let mut t = self;
            while let Data::Cons(head, tail) = t {
                elements.push(head.to_pretty_string());
                t = tail.as_ref();
            }
            format!("[{}]", elements.join(", "))
        } else {
            self.to_string()
        }
    }

    pub fn is_list(&self) -> bool {
        let mut t = self;
        while let Data::Cons(head, tail) = t {
            t = tail.as_ref();
        }
        if let Data::Nil = t { true } else { false }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        // super hujovo
        if s == "nil" {
            Some(Data::Nil)
        } else if s.starts_with("+") || s.starts_with("-") {
            let value: i64 = s.parse().ok()?;
            Some(Data::Number(value))
        } else if s.starts_with("(") && s.ends_with(")") {
            let s = &s[1..s.len()-1];

            // find comma
            let mut parens = 0;
            let mut comma_pos = None;
            for (i, ch) in s.char_indices() {
                if (ch == ',') && (parens == 0) {
                    comma_pos = Some(i);
                    break;
                } else if ch == '(' {
                    parens += 1;
                } else if ch == ')' {
                    parens -= 1;
                }
            }

            let comma_pos = comma_pos?;
            let head = Data::from_str(&s[..comma_pos])?;
            let tail = Data::from_str(&s[comma_pos+2..])?;
            Some(Data::Cons(Box::new(head), Box::new(tail)))
        } else {
            None
        }
    }

    pub fn into_vec(self) -> Vec<Self> {
        let mut res: Vec<Self> = Vec::new();
        let mut val = self;
        loop {
            match val {
                Self::Nil => break,
                Self::Cons(car, cdr) => {
                    res.push(*car);
                    val = *cdr;
                }
                _ => panic!("{:?}", val)
            }
        }
        res
    }

    pub fn try_as_number(&self) -> Option<i64> {
        match self {
            Self::Number(x) => Some(*x),
            _ => None,
        }
    }
}

pub fn bytes_to_squiggle(bytes: &[u8]) -> Option<Vec<u8>> {
    if !bytes.iter().all(|&x| (x == b'0') || (x == b'1')) {
        return None
    }
    Some(bytes.iter().map(|&x| match x {
        b'0' => 0,
        b'1' => 1,
        _ => unreachable!()
    }).collect())
}

fn modulate_int_into(x: u64, squiggle: &mut Vec<u8>) {
    let bitlength = 64 - x.leading_zeros();
    let chunks = (bitlength + 3) / 4;

    for _ in 0..chunks {
        squiggle.push(1);
    }
    squiggle.push(0);

    for i in (0..4*chunks).rev() {
        squiggle.push(((x >> i) & 1) as u8);
    }
}

pub fn modulate_into(data: Data, squiggle: &mut Vec<u8>) {
    match data {
        Data::Nil => {
            squiggle.push(0);
            squiggle.push(0);
        },
        Data::Number(x) => {
            if x >= 0 {
                squiggle.push(0);
                squiggle.push(1);
            } else {
                squiggle.push(1);
                squiggle.push(0);
            }

            modulate_int_into(x.abs() as u64, squiggle);
        },
        Data::Cons(head, tail) => {
            squiggle.push(1);
            squiggle.push(1);
            modulate_into(*head, squiggle);
            modulate_into(*tail, squiggle);
        }
    }
}

pub fn modulate(data: Data) -> Vec<u8> {
    let mut squiggle = Vec::new();
    modulate_into(data, &mut squiggle);
    squiggle
}

pub fn demodulate<'a, I>(mut squiggle: I) -> Option<(Data, I)>
where I: Iterator<Item = &'a u8>,
{
    match (squiggle.next()?, squiggle.next()?) {
        (0, 1) => demodulate_int(squiggle, 1),
        (1, 0) => demodulate_int(squiggle, -1),
        (0, 0) => Some((Data::Nil, squiggle)),
        (1, 1) => {
            let (head, rest) = demodulate(squiggle)?;
            let (tail, rest) = demodulate(rest)?;
            Some((Data::Cons(Box::new(head), Box::new(tail)), rest))
        },
        _ => None
    }
}

fn demodulate_int<'a, I>(mut squiggle: I, sign: i64) -> Option<(Data, I)>
where I: Iterator<Item = &'a u8>,
{
    let mut counter = 0;
    let chunks = loop {
        match squiggle.next()? {
            0 => break counter,
            1 => counter += 1,
            _ => return None
        }
    };

    let mut result: i64 = 0;
    for index in (0..4*chunks).rev() {
        match squiggle.next()? {
            0 => {},
            1 => result |= 1 << index,
            _ => return None
        }
    }

    /*
    // check if the number would've fit in a smaller number of chunks
    let mut canonical = (chunks == 0) || (result >= (1 << (4 * (chunks - 1))));
    // check also that zero is encoded with a positive sign
    if (result == 0) && (sign != 1) {
        canonical = false;
    }
    */

    Some((Data::Number(result * sign), squiggle))
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Data::*;

    #[test]
    fn i2s_examples() {
        assert_eq!(modulate(Number(0)), vec!{0, 1, 0});
        assert_eq!(modulate(Number(1)), vec!{0, 1, 1, 0, 0, 0, 0, 1});
        assert_eq!(modulate(Number(-1)), vec!{1, 0, 1, 0, 0, 0, 0, 1});
        assert_eq!(modulate(Number(2)), vec!{0, 1, 1, 0, 0, 0, 1, 0});
        assert_eq!(modulate(Number(-2)), vec!{1, 0, 1, 0, 0, 0, 1, 0});
        assert_eq!(modulate(Number(16)), vec!{0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0});
        assert_eq!(modulate(Number(-16)), vec!{1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0});
        assert_eq!(modulate(Number(255)), vec!{0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1});
        assert_eq!(modulate(Number(-255)), vec!{1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1});
        assert_eq!(modulate(Number(256)), vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0});
        assert_eq!(modulate(Number(-256)), vec!{1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0});
    }

    #[test]
    fn s2i_examples() {
        assert_eq!(demodulate((vec!{0, 1, 0}).iter()).unwrap().0, Number(0));
        assert_eq!(demodulate((vec!{0, 1, 1, 0, 0, 0, 0, 1}).iter()).unwrap().0, Number(1));
        assert_eq!(demodulate((vec!{1, 0, 1, 0, 0, 0, 0, 1}).iter()).unwrap().0, Number(-1));
        assert_eq!(demodulate((vec!{0, 1, 1, 0, 0, 0, 1, 0}).iter()).unwrap().0, Number(2));
        assert_eq!(demodulate((vec!{1, 0, 1, 0, 0, 0, 1, 0}).iter()).unwrap().0, Number(-2));
        assert_eq!(demodulate((vec!{0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0}).iter()).unwrap().0, Number(16));
        assert_eq!(demodulate((vec!{1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0}).iter()).unwrap().0, Number(-16));
        assert_eq!(demodulate((vec!{0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1}).iter()).unwrap().0, Number(255));
        assert_eq!(demodulate((vec!{1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1}).iter()).unwrap().0, Number(-255));
        assert_eq!(demodulate((vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}).iter()).unwrap().0, Number(256));
        assert_eq!(demodulate((vec!{1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}).iter()).unwrap().0, Number(-256));
    }

    #[test]
    fn s2i_invalid() {
        // too short
        assert!(demodulate((vec!{}).iter()).is_none());
        assert!(demodulate((vec!{0}).iter()).is_none());
        assert!(demodulate((vec!{1}).iter()).is_none());
        // truncated bits
        assert!(demodulate((vec!{0, 1, 1, 0, 0, 0, 0}).iter()).is_none());
        assert!(demodulate((vec!{0, 1, 1, 0, 0, 0}).iter()).is_none());
        assert!(demodulate((vec!{0, 1, 1, 0, 0}).iter()).is_none());
        // bits missing
        assert!(demodulate((vec!{0, 1, 1, 0}).iter()).is_none());
        // chunk count unterminated
        assert!(demodulate((vec!{0, 1, 1, 1}).iter()).is_none());
        assert!(demodulate((vec!{0, 1, 1}).iter()).is_none());
        // chunk count missing
        assert!(demodulate((vec!{0, 1}).iter()).is_none());
        // invalid values
        assert!(demodulate((vec!{0, 1, 1, 0, 0, 0, 2, 0}).iter()).is_none());
    }

    #[test]
    fn nil() {
        assert_eq!(modulate(Nil), vec!{0, 0});
        assert_eq!(demodulate((vec!{0, 0}).iter()).unwrap().0, Nil);
    }

    #[test]
    fn lists() {
        assert_eq!(modulate(Cons(Box::new(Nil), Box::new(Nil))), vec!{1, 1, 0, 0, 0, 0});
        assert_eq!(demodulate((vec!{1, 1, 0, 0, 0, 0}).iter()).unwrap().0, Cons(Box::new(Nil), Box::new(Nil)));
        assert_eq!(
            modulate(Cons(
                Box::new(Number(0)),
                Box::new(Nil)
            )),
            vec!{1, 1, 0, 1, 0, 0, 0}
        );
        assert_eq!(
            demodulate((vec!{1, 1, 0, 1, 0, 0, 0}).iter()).unwrap().0,
            Cons(
                Box::new(Number(0)),
                Box::new(Nil)
            )
        );
    }
}
