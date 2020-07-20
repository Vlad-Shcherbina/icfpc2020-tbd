use earthdok::simple_ai;
use tbd::runners::{run_in_submission, run_bots_local};

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(simple_ai::SimpleAi {});
    } else {
        run_bots_local(simple_ai::SimpleAi {}, simple_ai::SimpleAi {});
    }
}
