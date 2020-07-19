use tbd::local_bot_runner::run_bots;
use julie::predicting_ai::PredictingAi;

fn main() {
    run_bots(PredictingAi {}, PredictingAi {});
}
