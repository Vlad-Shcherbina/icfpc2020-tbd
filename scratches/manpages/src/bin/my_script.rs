extern crate manpages;

use manpages::alien_lisp as al;

fn main() {
  println!("Converting ``ap ap div 42 ap ap add x4 ap inc 1'' to Lisp");
  println!("{:?}", al::ufo_to_lisp("ap ap div 42 ap ap add x4 ap inc 1".to_string()));
  println!("Converting ``ap ap div ap ap add x4 ap inc 1'' (broken!!!) to Lisp");
  println!("{:?}", al::ufo_to_lisp("ap ap div ap ap add x4 ap inc 1".to_string()));
  println!("Converting ``(+ 1 (truncate (/ 3 4)))'' to UFOLang");
  println!("{:?}", al::lisp_to_ufo("(+ 1 (truncate (/ 3 4)))".to_string()));
}
