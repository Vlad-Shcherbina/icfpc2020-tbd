// Minimal player example. It can be used as a submission.
// Feel free to copy this and mess around.

use tbd::uforest::*;

fn main() {
    let client = Client::from_submission_argv();

    let gr = client.join(JoinRequest { mystery: Data::Nil });
    dbg!(&gr);
    if gr.stage == Stage::Finished {
        return;
    }

    let gr = client.start(ShipParams {
        fuel: 1,
        number2: 1,
        number3: 1,
        number4: 1,
    });
    dbg!(&gr);
    if gr.stage == Stage::Finished {
        return;
    }

    loop {
        let gr = client.commands(Commands(vec![]));
        dbg!(&gr);
        if gr.stage == Stage::Finished {
            return;
        }
    }
}
