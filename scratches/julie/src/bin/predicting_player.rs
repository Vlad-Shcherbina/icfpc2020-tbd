use tbd::runners::{run_in_submission, run_bots_local};
use julie::predicting_ai::PredictingAi;

fn main() {
    if tbd::is_running_in_submission() {
        let upgrades = Data::make_list2( 103652820, 192496425430);
        JoinRequest { upgrades }
    } else {
        run_bots_local(PredictingAi {}, PredictingAi {});
    }
}
