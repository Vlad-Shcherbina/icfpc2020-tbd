// Minimal player example. It can be used as a submission.
// Feel free to copy this and create your own bot.

use tbd::runners::{run_bots_local, run_in_submission};
use tbd::ai_interface::ExampleAi;

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(ExampleAi {});
    } else {
        run_bots_local(
            ExampleAi {},
            ExampleAi {},
        );
    }
}
