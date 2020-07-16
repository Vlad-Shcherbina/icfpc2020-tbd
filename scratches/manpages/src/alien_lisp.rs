use std::collections::HashMap;

pub fn known_operators() -> [&'static str; 7] {
  [
    "ap inc",
    "ap dec",
    "ap ap add",
    "ap ap mul",
    "ap ap div",
    "ap ap eq",
    "ap ap lt",
    //"ap gtl",
  ]
}

pub fn lisp_equivalents() -> [&'static str; 7] {
  [
    "(+ 1",
    "(- 1",
    "(+",
    "(*",
    "(truncate (/",
    "(=",
    "(<",
  ]
}

pub fn known_operators_with_arity_and_lisp_equivalent() -> HashMap<String, (usize, String)> {
  fn arity(x : String) -> usize {
    let ap_splitted : Vec<&str> = x.split("ap").collect();
    ap_splitted.len() - 1
  }
  let mut hm = HashMap::new();
  let lisp = &lisp_equivalents();
  let ko = &known_operators();
  for i in 0..ko.len() {
    hm.insert(ko[i].to_string(), (arity(ko[i].to_string()), lisp[i].to_string()));
  }
  hm
}

pub fn ufo_to_lisp(str_ufo_expr : String) -> Option<String> {
  fn find_leftmost_operator(here : String) -> Option<(String, String)> {
    for op in &known_operators() {
      //println!("op is {:?} ; Here is {:?}", here1, op, here);
      //let heresplit : Vec<&str> = here.split(op).collect();
      //println!("Here when splitted: {:?}", heresplit);
      if here.split(op).next().unwrap().is_empty() {
        //println!("Returning: {:?}, {:?}", op.to_string(), here.clone().replacen(op, "", 1).trim().to_string());
        return Some((op.to_string(), here.clone().replacen(op, "", 1).trim().to_string()));
      }
    }
    None
  }

  fn find_leftmost_argument(here : String) -> Option<(String, String)> {
    let mut splitted = here.split(" ");
    let first_maybe = splitted.next();
    match first_maybe {
      None => None,
      Some(first) => {
        //println!("FIRST IS {:?}", &first);
        match first {
          "" => None,
          "f" => Some(("#f".to_string(), here.clone().replacen(first, "", 1).trim().to_string())),
          "t" => Some(("#t".to_string(), here.clone().replacen(first, "", 1).trim().to_string())),
          _   => Some((first.to_string(), here.clone().replacen(first, "", 1).trim().to_string()))
        }
      },
    }
  }

  fn deop(rest : String, ops : &HashMap<String, (usize, String)>, acc : String, searching_for_n_terms : usize) -> Option<(String, String)> {
    //println!("Searching for {:?} terms in {:?} with accumulator of {:?}", searching_for_n_terms, rest, acc);
    // Recursion termination
    if searching_for_n_terms <= 0 {
      if rest.is_empty() {
        //println!("Returning {:?}", rest);
        return Some((acc, rest));
      } else {
        return None;
      }
    }

    // Searching for the next term
    match find_leftmost_operator(rest.clone()) {
      None => match find_leftmost_argument(rest.clone()) {
        None =>
          return None,
        Some((argument_found, rest1)) => {
          //println!(" Found {:?} instead!", argument_found);
          // Pad found argument with a space on the left if accumulator wasn't empty
          let acc1 = if acc.is_empty() { argument_found } else { format!("{} {}", acc, argument_found) };
          //println!("New accumulator state {:?}", acc1);
          // Return found argument for operator search to continue
          return Some((acc1, rest1));
        },
      },
      Some((operator_found, rest1)) => {
        //println!(" Found {:?}", operator_found);
        //println!("Ops hashmap is {:?}; searching for {:?}", ops, operator_found);
        let (arity, lisp) = ops.get(&operator_found).unwrap();
        // Split recursion, searching for the appropriate amount of terms
        let mut args = "".to_string();
        let mut rest_current = rest1;
        for i in 0..*arity {
          //println!("On iteration #{:?} :: args {:?} :: rest_current {:?}", i + 1, args, rest_current);
          if rest_current.is_empty() {
            return None;
          }
          let new_args_maybe = deop(rest_current, ops, "".to_string(), *arity - i);
          match new_args_maybe {
            None => return None,
            Some((new_args, new_rest)) => {
              args = if args.is_empty() { new_args } else { args + " " + &new_args };
              rest_current = new_rest;
            },
          }
        }
        // For each opening paren in the Lisp equivalent, close it
        let closing_time = ")".repeat(lisp.matches("(").count());
        return Some((
          format!("{} {}{}", lisp, args, closing_time),
          rest_current
        ))
      }
    }

  }
  //println!("Running ufo_to_lisp with {}", str_ufo_expr);
  match deop(str_ufo_expr, &known_operators_with_arity_and_lisp_equivalent(), "".to_string(), 1) {
    Some( (resu, _) ) => Some(resu),
    None => None,
  }
}

/*
pub fn lisp_to_ufo(str_lisp_expr : String) -> Option<String> {
  None
}
*/

#[cfg(test)]
mod tests {
  use super::*;
  use quickcheck::{Arbitrary, Gen};

  fn ufo_to_lisp_(x : String) -> String {
    ufo_to_lisp(x).unwrap()
  }

  /*
  fn lisp_to_ufo_(x : String) -> String {
    lisp_to_ufo(x).unwrap()
  }
  */

  #[derive(Debug, Clone)]
  struct UFOStringWrapper(String);
  impl Arbitrary for UFOStringWrapper {
    fn arbitrary<G : Gen>(g : &mut G) -> UFOStringWrapper {
      // I don't know how to make a shrinkable generator sadly, so here's some values for you
      let values = &[
        "t", "f", "-1", "0", "1", "x0", "x1",
        "ap ap add ap ap add 1 2 333",
        "ap ap add ap ap add 1 x1 1",
        "ap ap eq ap ap lt 1 2 ap ap lt ap ap div 1 2 ap dec 2",
        "ap inc 4",
        "ap inc ap ap div 3 4",
        "ap ap div 42 ap ap add x4 ap inc 1",
      ];
      let i = usize::arbitrary(g) % values.len();
      UFOStringWrapper(values[i].to_string())
    }
  }

  /*
  #[quickcheck]
  fn to_comp_from_is_id(UFOStringWrapper(str_ufo_exp) : UFOStringWrapper) -> bool {
    let str_lisp_expr = ufo_to_lisp_(str_ufo_exp.clone());
    ( str_ufo_exp == lisp_to_ufo_(ufo_to_lisp_(str_ufo_exp.clone())) ) &&
    ( str_lisp_expr == ufo_to_lisp_(lisp_to_ufo_(str_lisp_expr.clone())) )
  }
  */

  // 1,2,3
  #[quickcheck]
  fn ints_are_ints(x : i32) -> bool {
      ufo_to_lisp_(format!("{}", x)) == format!("{}", x)
  }

  // 4
  // We don't test definitional equality because Lisp is no Idris

  // 5,6
  #[quickcheck]
  fn inc_dec_are_plus_minus_one(x : i32) -> bool {
    ( ufo_to_lisp_(format!("ap inc {}", x)) == format!("(+ 1 {})", x) ) &&
    ( ufo_to_lisp_(format!("ap dec {}", x)) == format!("(- 1 {})", x) )
  }

  // 7
  #[quickcheck]
  fn add_is_plus(x : i32, y : i32) -> bool {
    ufo_to_lisp_(format!("ap ap add {} {}", x, y)) == format!("(+ {} {})", x, y)
  }

  // 8
  #[quickcheck]
  fn variables_are_variables(var_id : i32) -> bool {
    ufo_to_lisp_(format!("x{}", var_id)) == format!("x{}", var_id)
  }

  // 9
  #[quickcheck]
  fn mul_is_asterisk(x : i32, y : i32) -> bool {
    ufo_to_lisp_(format!("ap ap mul {} {}", x, y)) == format!("(* {} {})", x, y)
  }

  // 10
  #[quickcheck]
  fn div_is_truncate_slash(x : i32, y : i32) -> bool {
    ufo_to_lisp_(format!("ap ap div {} {}", x, y)) == format!("(truncate (/ {} {}))", x, y)
  }

  // 11
  #[quickcheck]
  fn eq_is_equals_and_t_and_f_are_sharp_t_and_sharp_f(UFOStringWrapper(str_ufo_exp) : UFOStringWrapper) -> bool {
    let str_lisp_expr = ufo_to_lisp_(str_ufo_exp.clone());
    let ufolang_source = format!("ap ap eq {} {}", str_ufo_exp, str_ufo_exp);
    let lisp_source = ufo_to_lisp_(ufolang_source.clone());
    let lisp_goal = format!("(= {} {})", &str_lisp_expr, &str_lisp_expr);

    //println!("UFO {:?} LISP SOURCE {:?} LISP GOAL {:?}", &ufolang_source, &lisp_source, &lisp_goal);

    ( lisp_source == lisp_goal ) &&
    ( ufo_to_lisp_("t".to_string()) == "#t" && ufo_to_lisp_("f".to_string()) == "#f" )
  }

  // 12
  #[quickcheck]
  fn lt_is_opening_pointy_bracket(x : i32, y : i32) -> bool {
    ufo_to_lisp_(format!("ap ap lt {} {}", x, y)) == format!("(< {} {})", x, y)
  }

}
