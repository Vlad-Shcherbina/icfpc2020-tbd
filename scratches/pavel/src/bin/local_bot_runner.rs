use tbd::local_bot_runner::run_bots;
use pavel::simple_ai;

fn main() {
    run_bots(simple_ai::OrbitBot {}, simple_ai::OrbitBot {});
}