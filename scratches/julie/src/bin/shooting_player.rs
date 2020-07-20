use tbd::runners::{run_in_submission, run_bots_local};
use julie::shooting_bot::ShootingAI;

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(ShootingAI {});
    } else {
        run_bots_local(ShootingAI {}, ShootingAI {});
    }
}
