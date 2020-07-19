// Minimal player example. It can be used as a submission.
// Feel free to copy this and mess around.

use tbd::uforest::*;

fn main() {
    let client = Client::from_submission_argv();

    loop {
        eprintln!("*** new game ***");

        let gr = client.join(JoinRequest { mystery: Data::Nil });
        dbg!(&gr);
        // assert_eq!(success, 1);
        if gr.stage == Stage::Finished {
            eprintln!("the game ended prematurely");
            continue;
        }
        assert_eq!(gr.stage, Stage::NotStarted);

        let gr = client.start(ShipParams {
            fuel: 1,
            number2: 1,
            number3: 1,
            number4: 1,
        });
        dbg!(&gr);
        assert_eq!(gr.stage, Stage::Started);

        loop {
            let gr = client.commands(Commands(vec![]));
            dbg!(&gr);
            match gr.stage {
                Stage::NotStarted => panic!(),
                Stage::Started => {},
                Stage::Finished => break,
            }
        }

        // try to send another request just to see what happens
        let gr = client.commands(Commands(vec![]));
        dbg!(&gr);
    }
}
