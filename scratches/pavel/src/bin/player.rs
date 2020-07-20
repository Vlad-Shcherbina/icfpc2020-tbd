// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::uforest::*;
use tbd::ai_interface::Ai;
use pavel::simple_ai;
use tbd::uforest::Command::Accelerate;
use tbd::runners::{run_in_submission, run_bots_local};
use pavel::simple_ai::OrbitBot;


fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(OrbitBot {});
    } else {
        run_bots_local(OrbitBot {}, OrbitBot {});
    }
}
