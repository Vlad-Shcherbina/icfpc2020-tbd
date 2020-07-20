use tbd::runners::{run_in_submission, run_bots_local};
use julie::predicting_ai::PredictingAi;
use tbd::uforest::{JoinRequest, ShipParams, GameSpec, Data, GameState, Commands };

fn main() {
    if tbd::is_running_in_submission() {
        run_in_submission(PredictingAi {});
    } else {
        run_bots_local(PredictingAi {}, PredictingAi {});
    }
}
