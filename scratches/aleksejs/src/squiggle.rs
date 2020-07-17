#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus,
    Minus
}

#[derive(Debug, PartialEq)]
pub enum Data {
    Nil,
    Number(Sign, u32),
    Cons(Box<Data>, Box<Data>)
}

fn modulate_int_into(x: u32, squiggle: &mut Vec<u8>) {
    let bitlength = 32 - x.leading_zeros();
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
        Data::Number(sign, x) => {
            match sign {
                Sign::Plus => {
                    squiggle.push(0);
                    squiggle.push(1);
                },
                Sign::Minus => {
                    squiggle.push(1);
                    squiggle.push(0);
                }
            }

            modulate_int_into(x, squiggle);
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
        (0, 1) => demodulate_int(squiggle, Sign::Plus),
        (1, 0) => demodulate_int(squiggle, Sign::Minus),
        (0, 0) => Some((Data::Nil, squiggle)),
        (1, 1) => {
            let (head, rest) = demodulate(squiggle)?;
            let (tail, rest) = demodulate(rest)?;
            Some((Data::Cons(Box::new(head), Box::new(tail)), rest))
        },
        _ => None
    }
}

fn demodulate_int<'a, I>(mut squiggle: I, sign: Sign) -> Option<(Data, I)>
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

    let mut result: u32 = 0;
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

    Some((Data::Number(sign, result), squiggle))
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use super::Data::*;
    use super::Sign::*;

    #[test]
    fn i2s_examples() {
        assert_eq!(modulate(Number(Plus, 0)), vec!{0, 1, 0});
        assert_eq!(modulate(Number(Plus, 1)), vec!{0, 1, 1, 0, 0, 0, 0, 1});
        assert_eq!(modulate(Number(Minus, 1)), vec!{1, 0, 1, 0, 0, 0, 0, 1});
        assert_eq!(modulate(Number(Plus, 2)), vec!{0, 1, 1, 0, 0, 0, 1, 0});
        assert_eq!(modulate(Number(Minus, 2)), vec!{1, 0, 1, 0, 0, 0, 1, 0});
        assert_eq!(modulate(Number(Plus, 16)), vec!{0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0});
        assert_eq!(modulate(Number(Minus, 16)), vec!{1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0});
        assert_eq!(modulate(Number(Plus, 255)), vec!{0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1});
        assert_eq!(modulate(Number(Minus, 255)), vec!{1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1});
        assert_eq!(modulate(Number(Plus, 256)), vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0});
        assert_eq!(modulate(Number(Minus, 256)), vec!{1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0});
    }

    #[test]
    fn s2i_examples() -> Result<(), std::option::NoneError> {
        assert_eq!(demodulate((vec!{0, 1, 0}).iter())?.0, Number(Plus, 0));
        assert_eq!(demodulate((vec!{0, 1, 1, 0, 0, 0, 0, 1}).iter())?.0, Number(Plus, 1));
        assert_eq!(demodulate((vec!{1, 0, 1, 0, 0, 0, 0, 1}).iter())?.0, Number(Minus, 1));
        assert_eq!(demodulate((vec!{0, 1, 1, 0, 0, 0, 1, 0}).iter())?.0, Number(Plus, 2));
        assert_eq!(demodulate((vec!{1, 0, 1, 0, 0, 0, 1, 0}).iter())?.0, Number(Minus, 2));
        assert_eq!(demodulate((vec!{0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0}).iter())?.0, Number(Plus, 16));
        assert_eq!(demodulate((vec!{1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0}).iter())?.0, Number(Minus, 16));
        assert_eq!(demodulate((vec!{0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1}).iter())?.0, Number(Plus, 255));
        assert_eq!(demodulate((vec!{1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1}).iter())?.0, Number(Minus, 255));
        assert_eq!(demodulate((vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}).iter())?.0, Number(Plus, 256));
        assert_eq!(demodulate((vec!{1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}).iter())?.0, Number(Minus, 256));
        Ok(())
    }

    // #[test]
    // fn s2i_invalid() {
    //     // too short
    //     assert_eq!(squiggle_to_int(&vec!{}), None);
    //     assert_eq!(squiggle_to_int(&vec!{0}), None);
    //     assert_eq!(squiggle_to_int(&vec!{1}), None);
    //     // invalid sign
    //     assert_eq!(squiggle_to_int(&vec!{0, 0}), None);
    //     assert_eq!(squiggle_to_int(&vec!{1, 1}), None);
    //     assert_eq!(squiggle_to_int(&vec!{0, 0, 1, 0, 0, 0, 1, 0}), None);
    //     assert_eq!(squiggle_to_int(&vec!{1, 1, 1, 0, 0, 0, 1, 0}), None);
    //     // truncated bits
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 0}), None);
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0}), None);
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0}), None);
    //     // bits missing
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0}), None);
    //     // chunk count unterminated
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1}), None);
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1}), None);
    //     // invalid values
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 2, 0}), None);
    //     // too many bits
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 1, 0, 0}), None);
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0}), None);
    // }

    // #[test]
    // fn s2i_noncanonical() {
    //     // zero encoded as negative
    //     assert_eq!(squiggle_to_int(&vec!{1, 0, 0}), Some((0, false)));
    //     // zero encoded as negative with extra chunks
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 0, 0}), Some((0, false)));
    //     assert_eq!(squiggle_to_int(&vec!{1, 0, 1, 0, 0, 0, 0, 0}), Some((0, false)));
    //     // one with extra chunks
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1}), Some((1, false)));
    //     assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 0, 1}), Some((1, false)));
    // }

    // #[quickcheck]
    // fn i2s_s2i_roundtrip(x: i32) -> bool {
    //     let squiggle = int_to_squiggle(x);
    //     squiggle_to_int(&squiggle) == Some((x, true))
    // }
}
*/
