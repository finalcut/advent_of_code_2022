use aoc_util::{get_seed_data};
use dict::{Dict, DictIface};

fn main() -> std::io::Result<()> {

    let rounds = get_seed_data().expect("Could not load values");

    // define values
    let mut play_value = Dict::<u16>::new();
    play_value.add("Rock".to_string(), 1);
    play_value.add("Paper".to_string(), 2);
    play_value.add("Scissors".to_string(), 3);

    let mut result_value = Dict::<u16>::new();
    result_value.add("W".to_string(), 6);
    result_value.add("L".to_string(), 0);
    result_value.add("D".to_string(), 3);

    // key beats value
    let mut get_what_losses_to_the_played_option_of = Dict::<String>::new();
    get_what_losses_to_the_played_option_of.add("Paper".to_string(), "Rock".to_string());
    get_what_losses_to_the_played_option_of.add("Scissors".to_string(), "Paper".to_string());
    get_what_losses_to_the_played_option_of.add("Rock".to_string(), "Scissors".to_string());

    // value beats key
    let mut get_what_beats_the_played_option_of = Dict::<String>::new();
    get_what_beats_the_played_option_of.add("Paper".to_string(), "Scissors".to_string());
    get_what_beats_the_played_option_of.add("Scissors".to_string(), "Rock".to_string());
    get_what_beats_the_played_option_of.add("Rock".to_string(), "Paper".to_string());

    // keep track of strategy guide
    let mut opponent_legend = Dict::<String>::new();
    // add an element 1 indexed by the key "key"
    opponent_legend.add("A".to_string(), "Rock".to_string());
    opponent_legend.add("B".to_string(), "Paper".to_string());
    opponent_legend.add("C".to_string(), "Scissors".to_string());

    let mut my_legend = Dict::<String>::new();
    my_legend.add("X".to_string(), "Rock".to_string());
    my_legend.add("Y".to_string(), "Paper".to_string());
    my_legend.add("Z".to_string(), "Scissors".to_string());

    let mut my_legend2 = Dict::<String>::new();
    my_legend2.add("X".to_string(), "L".to_string());
    my_legend2.add("Y".to_string(), "D".to_string());
    my_legend2.add("Z".to_string(), "Z".to_string());

    // results:
    let mut total_score_round1: u16 = 0;
    let mut total_score_round2: u16 = 0;

    // we can iterate just like an array
    for round in rounds {
        // convert the round into something I know how to use
        let split = round.split(" ");
        let r = split.collect::<Vec<&str>>();

        let o_play = opponent_legend.get(&r[0]).unwrap();
        let my_play = my_legend.get(&r[1]).unwrap();

        // ROUND 1 LOGIC
        // a DRAW
        let mut my_play_value_round1 = *play_value.get(&my_play).unwrap();
        if my_play.eq(o_play) {
            my_play_value_round1 += result_value.get("D").unwrap();
        //  println!( "{} - {}", "D".to_string() , my_play_value_round1);
        } else {
            // A WIN
            let result = get_what_losses_to_the_played_option_of
                .get(&my_play)
                .unwrap();
            let i_win: bool = result.eq(o_play);

            //   println!( "O: {} - R: {} - Win?: {}", o_play, result, i_win );

            if i_win {
                my_play_value_round1 += result_value.get("W").unwrap();
            //  println!( "{} - {}", "W".to_string(), my_play_value_round1 );
            } else {
                // a loss
                my_play_value_round1 += result_value.get("L").unwrap();
                //  println!( "{} - {}", "L".to_string(), my_play_value_round1);
            }
        }
        total_score_round1 += my_play_value_round1;

        // ROUND 2 LOGIC

        let desired_result = my_legend2.get(&r[1]).unwrap();
        let mut my_play_value_round2 = 0;
        // println!( "O: {} - Desired Result: {}", o_play, desired_result );

        // a DRAW
        if desired_result.eq("D") {
            let my_actual_play = o_play;
            my_play_value_round2 += *play_value.get(&my_actual_play).unwrap();
            my_play_value_round2 += result_value.get("D").unwrap();
            // println!( "DRAW O: {} - R: {} - Round Value?: {}", o_play, my_actual_play, my_play_value_round2 );
        } else {
            if desired_result.eq("L") {
                // A LOSS
                // get the winning play
                let my_actual_play = get_what_losses_to_the_played_option_of
                    .get(&o_play)
                    .unwrap();
                my_play_value_round2 += *play_value.get(&my_actual_play).unwrap();
                my_play_value_round2 += result_value.get("L").unwrap();
                // println!( "LOSS O: {} - R: {} - Round Value?: {}", o_play, my_actual_play, my_play_value_round2 );
            } else {
                // A WIN
                let my_actual_play = get_what_beats_the_played_option_of.get(&o_play).unwrap();
                my_play_value_round2 += *play_value.get(&my_actual_play).unwrap();
                my_play_value_round2 += result_value.get("W").unwrap();
                // println!( "LOSS O: {} - R: {} - Round Value?: {}", o_play, my_actual_play, my_play_value_round2 );
            }
        }

        total_score_round2 += my_play_value_round2;
    }
    println!("Total 1: {}", total_score_round1);
    println!("Total 2: {}", total_score_round2);
    Ok(())
}
