// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::uforest::*;
use tbd::ai_interface::{Ai, ExampleAi};

fn main() {
    tbd::runners::run_in_submission(ExampleAi {} );
}
