extern crate manpages;

use manpages::alien_lisp as al;

fn main() {
  println!("Converting ``ap ap div 42 ap ap add x4 ap inc 1'' to Lisp");
  println!("{:?}", al::ufo_to_lisp("ap ap div 42 ap ap add x4 ap inc 1".to_string()));
  println!("Converting ``ap ap div ap ap add x4 ap inc 1'' (broken!!!) to Lisp");
  println!("{:?}", al::ufo_to_lisp("ap ap div ap ap add x4 ap inc 1".to_string()));
}
