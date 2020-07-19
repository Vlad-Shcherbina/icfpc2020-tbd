use tbd::local_bot_runner::run_bots;
use earthdok::simple_ai;

fn main() {
   run_bots(simple_ai::SimpleAi {}, simple_ai::SimpleAi {});
}