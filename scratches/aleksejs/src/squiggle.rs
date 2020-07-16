pub fn int_to_squiggle(x: i32) -> Vec<u8> {
    let mut squiggle = Vec::new();

    if x.is_negative() {
        squiggle.push(1);
        squiggle.push(0);
    } else {
        squiggle.push(0);
        squiggle.push(1);
    }

    // NB: fails on i32::MIN
    let x = x.abs();
    let bitlength = 32 - x.leading_zeros();
    let chunks = (bitlength + 3) / 4;

    for _ in 0..chunks {
        squiggle.push(1);
    }
    squiggle.push(0);

    for i in (0..4*chunks).rev() {
        squiggle.push(((x >> i) & 1) as u8);
    }

    squiggle
}

// TODO: bound checking
pub fn squiggle_to_int(squiggle: &[u8]) -> Option<(i32, bool)> {
    let mut squiggle = squiggle.iter();

    let sign = match (squiggle.next(), squiggle.next()) {
        (Some(0), Some(1)) => 1,
        (Some(1), Some(0)) => -1,
        _ => return None
    };

    let mut counter = 0;
    let chunks = loop {
        match squiggle.next() {
            Some(0) => break counter,
            Some(1) => counter += 1,
            _ => return None
        }
    };

    let mut result: i32 = 0;
    for index in (0..4*chunks).rev() {
        match squiggle.next() {
            Some(0) => {},
            Some(1) => result |= 1 << index,
            _ => return None
        }
    }

    // squiggle must be over by now
    if squiggle.next() != None {
        return None
    }

    // check if the number would've fit in a smaller number of chunks
    let mut canonical = (chunks == 0) || (result >= (1 << (4 * (chunks - 1))));
    // check also that zero is encoded with a positive sign
    if (result == 0) && (sign != 1) {
        canonical = false;
    }

    Some((result * sign, canonical))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn i2s_examples() {
    assert_eq!(int_to_squiggle(0), vec!{0, 1, 0});
    assert_eq!(int_to_squiggle(1), vec!{0, 1, 1, 0, 0, 0, 0, 1});
    assert_eq!(int_to_squiggle(-1), vec!{1, 0, 1, 0, 0, 0, 0, 1});
    assert_eq!(int_to_squiggle(2), vec!{0, 1, 1, 0, 0, 0, 1, 0});
    assert_eq!(int_to_squiggle(-2), vec!{1, 0, 1, 0, 0, 0, 1, 0});
    assert_eq!(int_to_squiggle(16), vec!{0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0});
    assert_eq!(int_to_squiggle(-16), vec!{1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0});
    assert_eq!(int_to_squiggle(255), vec!{0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1});
    assert_eq!(int_to_squiggle(-255), vec!{1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1});
    assert_eq!(int_to_squiggle(256), vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0});
    assert_eq!(int_to_squiggle(-256), vec!{1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0});
  }

  #[test]
  fn s2i_examples() {
    assert_eq!(squiggle_to_int(&vec!{0, 1, 0}), Some((0, true)));
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 0, 1}), Some((1, true)));
    assert_eq!(squiggle_to_int(&vec!{1, 0, 1, 0, 0, 0, 0, 1}), Some((-1, true)));
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 1, 0}), Some((2, true)));
    assert_eq!(squiggle_to_int(&vec!{1, 0, 1, 0, 0, 0, 1, 0}), Some((-2, true)));
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0}), Some((16, true)));
    assert_eq!(squiggle_to_int(&vec!{1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0}), Some((-16, true)));
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1}), Some((255, true)));
    assert_eq!(squiggle_to_int(&vec!{1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1}), Some((-255, true)));
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}), Some((256, true)));
    assert_eq!(squiggle_to_int(&vec!{1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}), Some((-256, true)));
  }

  #[test]
  fn s2i_invalid() {
    // too short
    assert_eq!(squiggle_to_int(&vec!{}), None);
    assert_eq!(squiggle_to_int(&vec!{0}), None);
    assert_eq!(squiggle_to_int(&vec!{1}), None);
    // invalid sign
    assert_eq!(squiggle_to_int(&vec!{0, 0}), None);
    assert_eq!(squiggle_to_int(&vec!{1, 1}), None);
    assert_eq!(squiggle_to_int(&vec!{0, 0, 1, 0, 0, 0, 1, 0}), None);
    assert_eq!(squiggle_to_int(&vec!{1, 1, 1, 0, 0, 0, 1, 0}), None);
    // truncated bits
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 0}), None);
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0}), None);
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0}), None);
    // bits missing
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0}), None);
    // chunk count unterminated
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1}), None);
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1}), None);
    // invalid values
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 2, 0}), None);
    // too many bits
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 1, 0, 0}), None);
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0}), None);
  }

  #[test]
  fn s2i_noncanonical() {
    // zero encoded as negative
    assert_eq!(squiggle_to_int(&vec!{1, 0, 0}), Some((0, false)));
    // zero encoded as negative with extra chunks
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 0, 0, 0, 0, 0}), Some((0, false)));
    assert_eq!(squiggle_to_int(&vec!{1, 0, 1, 0, 0, 0, 0, 0}), Some((0, false)));
    // one with extra chunks
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1}), Some((1, false)));
    assert_eq!(squiggle_to_int(&vec!{0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 0, 1}), Some((1, false)));
  }

  #[quickcheck]
  fn i2s_s2i_roundtrip(x: i32) -> bool {
    let squiggle = int_to_squiggle(x);
    squiggle_to_int(&squiggle) == Some((x, true))
  }
}
