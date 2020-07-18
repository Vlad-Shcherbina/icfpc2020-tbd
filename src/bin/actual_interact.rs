use tbd::ufolang::Protocol;
use tbd::ufolang::Value::*;
use std::rc::Rc;

fn main1() {
    // let protocol = Protocol::load_galaxy();

    // statelessdraw
    let protocol = Protocol::from_snippet("\
    main = ap ap c ap ap b b ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c ap ap b cons ap ap c cons nil nil
    ");

    // let internal_state = Rc::new(Nil);
    // let data_in = Rc::new(Pair(Rc::new(Number(0)), Rc::new(Number(0))));
    // protocol.invoke(internal_state, data_in);
}

fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(main1).unwrap()
        .join().unwrap();
}
