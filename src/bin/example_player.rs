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

        let gr = client.start(InitialShipParams {
            number1: 0,
            number2: 0,
            number3: 0,
            number4: 0,
        });
        dbg!(&gr);
        assert_eq!(gr.stage, Stage::Started);

        loop {
            let gr = client.commands(Commands { mystery: Data::Nil });
            dbg!(&gr);
            match gr.stage {
                Stage::NotStarted => panic!(),
                Stage::Started => {},
                Stage::Finished => break,
            }
        }

        // try to send another request just to see what happens
        let gr = client.commands(Commands { mystery: Data::Nil });
        dbg!(&gr);
    }
}
