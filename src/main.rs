use rand::{
    distributions::{Distribution, Standard},
    Rng,
    //SeedableRng,
    //StdRng
};
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dot{
    BLUE,
    GREEN,
    YELLOW,
    RED,
    WHITE, 
    BLACK,
    NotSet,
}

impl From<u32> for Dot {
    fn from(d : u32) -> Self {
        match d {
            0 => Dot::BLUE,
            1 => Dot::GREEN,
            2 => Dot::YELLOW,
            3 => Dot::RED,
            4 => Dot::WHITE,
            5 => Dot::BLACK,
            _ => Dot::NotSet,
        }
    }
}

impl From<Dot> for usize {
    fn from(d : Dot) -> Self {
        match d {
            Dot::BLUE => 0,
            Dot::GREEN => 1,
            Dot::YELLOW => 2,
            Dot::RED => 3,
            Dot::WHITE => 4,
            Dot::BLACK => 5,
            Dot::NotSet => 6,
        }
    }
}

impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dot::BLUE => write!(f, "BLUE"),
            Dot::GREEN => write!(f, "GREEN"),
            Dot::YELLOW => write!(f, "YELLOW"),
            Dot::RED => write!(f, "RED"),
            Dot::WHITE => write!(f, "WHITE"),
            Dot::BLACK => write!(f, "BLACK"),
            Dot::NotSet => write!(f, "Not Set Yet"),
        }
    }
}

impl Distribution<Dot> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dot {
        match rng.gen_range(0,6) {
            0 => Dot::BLUE,
            1 => Dot::GREEN,
            2 => Dot::YELLOW,
            3 => Dot::RED,
            4 => Dot::WHITE,
            _ => Dot::BLACK,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Clue{
    Nonexistant,
    Exist,
    Correct, 
}

impl fmt::Display for Clue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Clue::Nonexistant => write!(f, "Nonexistant"),
            Clue::Exist => write!(f, "Exist"),
            Clue::Correct => write!(f, "Correct"),
        }
    }
}

type Guess = [Dot; 4];
type Answer = [Clue; 4];
type Analysis = [[f32; 6]; 4];

fn is_all_correct(answer : &Answer) -> bool {
    for clue in answer {
        if *clue != Clue::Correct {
            return false;
        }
    }
    true
}

fn create_all_guesses(guesses : &mut Vec<Guess>) {

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut l = 0;

    while i < 6 {
        //[Dot::from(i), Dot::from(j), Dot::from(k), Dot::from(l)]
        guesses.push([Dot::from(i), Dot::from(j), Dot::from(k), Dot::from(l)]);
        l = l + 1;
        if l >= 6 {
            l = 0;
            k = k + 1;
        }
        if k >= 6 {
            k = 0;
            j = j + 1;
        }
        if j >= 6 {
            j = 0;
            i = i + 1;
        }
    }

}

fn create_guess_analysis(guesses : &Vec<Guess>) -> Analysis {
    let mut analysis : [[u32; 6]; 4] = [[0; 6]; 4];
    for guess in guesses {
        analysis[0][guess[0] as usize] += 1;
        analysis[1][guess[1] as usize] += 1;
        analysis[2][guess[2] as usize] += 1;
        analysis[3][guess[3] as usize] += 1;
    }
    let mut fa : Analysis = [[0f32; 6]; 4];
    for guess in guesses {
        fa[0][guess[0] as usize] = (analysis[0][guess[0] as usize] as f32)/(guesses.len() as f32);
        fa[1][guess[1] as usize] = (analysis[1][guess[1] as usize] as f32)/(guesses.len() as f32);
        fa[2][guess[2] as usize] = (analysis[2][guess[2] as usize] as f32)/(guesses.len() as f32);
        fa[3][guess[3] as usize] = (analysis[3][guess[3] as usize] as f32)/(guesses.len() as f32);
    }
    fa

}

fn score_guess(analysis : &Analysis, guess : &Guess) -> f32 {
    analysis[0][guess[0] as usize] +
    analysis[1][guess[1] as usize] +
    analysis[2][guess[2] as usize] +
    analysis[3][guess[3] as usize]
}

fn compare_guesses_by_analysis(analysis : &Analysis, a : &Guess, b : &Guess) -> Ordering {
    let diff = score_guess(analysis, a) - score_guess(analysis, b);
    if diff < 0.0 {
        Ordering::Less
    }else if diff > 0.0 {
        Ordering::Greater
    }else {
        Ordering::Equal
    }
}
/*
fn create_best(stats : &mut [u32; 6]) -> Dot {
    let mut max_i = 0;
    let mut curr_max = 0;

    for i in 0..6 {
        if stats[i] > curr_max {
            curr_max = stats[i];
            max_i = i;
        }
    }
    stats[max_i] = 0;
    Dot::from(max_i as u32)
}
*/
fn make_guess(possible_guesses : &mut Vec<Guess>) -> Guess {

    let l = possible_guesses.len();
    // Try to make intellegent guess
    // We see what most guesses has in each position and then guess on that
    /*
    let mut mapping : [[u32; 6]; 4] = [[0; 6]; 4];
    for i in possible_guesses {
        for j in 0..4 {
            let color = i[j] as usize;
            mapping[j][color] = mapping[j][color] + 1;
        }
    }
    let item = [
        create_best(&mut mapping[0]),
        create_best(&mut mapping[1]),
        create_best(&mut mapping[2]),
        create_best(&mut mapping[3])
    ];

    let cool = possible_guesses.iter().position(|x| *x == item);
    if cool.is_none() {
        println!("Didnt manage to find intellegent guess.");
    }else{
        let guess = possible_guesses.swap_remove(cool.unwrap());
        for i in 0..4 {
            guess_to_make[i] = guess[i];
        }
    }
    
    let mut f = First { number : [0; 6], refs : [None, None, None, None, None, None], };
    for i in 0..l {
        for j in 0..4 {
            let color = possible_guesses[i][j] as usize;
            // TODO Continue filling in this tree. Maybe make impls to do it more easily
            f.number[color] = f.number[color] + 1;
            f.refs[color] = 
                Some(Box::new(Sec {number : [0; 6], refs : [None, None, None, None, None, None], }));
        }
    }
    */

    let mut rng = rand::thread_rng();//SeedableRng::from_seed(seed);
    let r : usize = rng.gen_range(0, l);
    //let guess = possible_guesses.swap_remove(r);
    possible_guesses.swap_remove(r)
    /*
    for i in 0..4 {
        guess_to_make[i] = guess[i];
    }
    */
    //[rand::random(), rand::random(), rand::random(), rand::random()]
}

fn make_intellegent_guess(possible_guesses : &mut Vec<Guess>) -> Guess {
    // NEW VERSION
    // create guess analysis
    let analysis = create_guess_analysis(&possible_guesses);
    // Sort possible_guesses using the guess analysis
    possible_guesses.sort_unstable_by(|a, b| compare_guesses_by_analysis(&analysis, a, b));
    // choose the last guess in possible guesses as 
    let score_first = score_guess(&analysis, &possible_guesses[0]);
    let score_last = score_guess(&analysis, &possible_guesses[possible_guesses.len()-1]);
    /*
    println!("Score first: {score}", score = score_first);
    println!("Guess 1: {guess}", guess = possible_guesses[0][0]);
    println!("Guess 2: {guess}", guess = possible_guesses[0][1]);
    println!("Guess 3: {guess}", guess = possible_guesses[0][2]);
    println!("Guess 4: {guess}", guess = possible_guesses[0][3]);
    println!("Score last : {score}", score = score_second);
    println!("Guess 1: {guess}", guess = possible_guesses[possible_guesses.len()-1][0]);
    println!("Guess 2: {guess}", guess = possible_guesses[possible_guesses.len()-1][1]);
    println!("Guess 3: {guess}", guess = possible_guesses[possible_guesses.len()-1][2]);
    println!("Guess 4: {guess}", guess = possible_guesses[possible_guesses.len()-1][3]);

    for i in 0..6 {
        for j in 0..4 {
            println!("Analysis {g},{color} : {f}", g = i, color = j, f = analysis[j][i]);
        }
    }
    */
    if score_first < score_last {
        possible_guesses.pop().unwrap()
    }else {
        make_guess(possible_guesses)
    }
}

fn give_answer(guess : &Guess, correct : &Guess, answer : &mut Answer) {
    let mut corr_has_been_used : [bool; 4] = [false; 4];
    let mut guess_has_been_used : [bool; 4] = [false; 4];
    
    let mut mapping : [usize; 4] = [0, 1, 2, 3];
    let mut rng = rand::thread_rng();//SeedableRng::from_seed(seed);
    for _i in 0..8 {
        let d1 : u32 = rng.gen_range(0,4);//rand::random()%4;
        let rand1 : usize = d1 as usize; // FIXME

        let d2 : u32 = rng.gen_range(0,4);//rand::random()%4;
        let rand2 : usize = d2 as usize;

        let tmp = mapping[rand1];
        mapping[rand1] = mapping[rand2];
        mapping[rand2] = tmp;
    }

    // Check correct
    for i in 0..4 {
        if correct[i] == guess[i] {
            answer[mapping[i]] = Clue::Correct;
            corr_has_been_used[i] = true;
            guess_has_been_used[i] = true;
        }

    }

    // Check Exist
    for i in 0..4 {
        for j in 0..4 {
            if correct[i] == guess[j] && !corr_has_been_used[i] && !guess_has_been_used[j] {
                answer[mapping[i]] = Clue::Exist;
                corr_has_been_used[i] = true;
                guess_has_been_used[j] = true;
            }
        }
    }
}

fn count_dot_colors(dots : &Guess, count : &mut [u32; 6]) {
    for color in 0..6 {
        count[color] = dots
            .iter()
            .filter(|x| **x as usize == color)
            .count() as u32;
    }
}

fn is_guess_possible(answer : &Answer, made_guess : &Guess, new_guess : &Guess) -> bool {

    // See if guesses are the same in as many places as we had correct
    let num_corrects = answer.iter().filter(|x| **x == Clue::Correct).count();
    let mut num_same = 0;
    for i in 0..4 {
        if made_guess[i] == new_guess[i] {
            num_same = num_same + 1;
        }
    }
    if num_corrects != num_same {
        return false;
    }

    
    // See if the guess has as many dots differing as there are nonexistant clues
    let num_nonexists = answer.iter().filter(|x| **x == Clue::Nonexistant).count();
    //let num_exists = answer.iter().filter(|x| **x == Clue::Exist).count();

    let mut colors_new_guess : [u32; 6] = [0; 6];
    count_dot_colors(new_guess, &mut colors_new_guess);
    let mut colors_made_guess : [u32; 6]  = [0; 6];
    count_dot_colors(made_guess, &mut colors_made_guess);

    let mut added_colors = 0;
    let mut sub_colors = 0;
    for i in 0..6 {
        let diff = (colors_new_guess[i] as i32)-(colors_made_guess[i] as i32);
        if diff > 0 {
            added_colors = added_colors + (diff as usize);
        }else if diff < 0 {
            sub_colors = sub_colors + (diff.abs() as usize);
        }
    }
    
    if added_colors != sub_colors && num_nonexists != added_colors && num_nonexists != sub_colors {
        return false;
    }
    
    /* THIS SHOULD WORK... But I cant find the error... At the same time... it should not add much
    if sub_colors != 4-(num_exists+num_corrects) {
        return false;
    }
    */

    // Otherwise return true
    true
}

fn main() {

    //println!("{c}", c = Dot::BLACK as u32);
    //println!("{c}", c = Dot::from(6));

    // Configuration
    let cmd_display = false;
    const NUM_GAMES : usize = 10000;
    let try_to_be_intellegent = true;
    const NUM_TURNS : usize = 15;

    // Statistics
    let mut turn_won_on : [u32; NUM_TURNS] = [0; NUM_TURNS]; 
    let mut ran_out_of_guesses = 0;
    let mut ran_out_of_time = 0;

    for _n in 0..NUM_GAMES {
        
        // Init
        let mut guess_board : [Guess; NUM_TURNS] = [[Dot::NotSet; 4]; NUM_TURNS];
        let mut answer_board : [Answer; NUM_TURNS] = [[Clue::Nonexistant; 4]; NUM_TURNS];
        let mut current_guess = 0;
        let mut correct_found = false;
        
        
        // Create Correct order
        let correct_order: Guess = [rand::random(), rand::random(), rand::random(), rand::random()];

        // Init guess state
        //let mut guess_state : [[f32; 6]; 4] = [[1.0/6.0; 6]; 4];
        //let mut clue_state : [[f32; 4]; 4] = [[1.0/6.0; 6]; 4];
        let mut possible_guesses : Vec<Guess> = Vec::with_capacity(1296);
        create_all_guesses(&mut possible_guesses);


        if cmd_display {
            println!("=== GAME START ===");
            println!("Number of possible guesses: {guesses}", guesses = possible_guesses.len());
        }
            
        // Start guessing
        while current_guess < NUM_TURNS && !correct_found && possible_guesses.len() > 0 {

            if current_guess > 0 && try_to_be_intellegent {
                guess_board[current_guess] = make_intellegent_guess(&mut possible_guesses);
            }else{
                guess_board[current_guess] = make_guess(&mut possible_guesses);
            }

            let curr_guess = &guess_board[current_guess];
            let mut curr_answer = &mut answer_board[current_guess];

            give_answer(&curr_guess, &correct_order, &mut curr_answer);

            possible_guesses = possible_guesses
                .into_iter()
                .filter(|x| is_guess_possible(&curr_answer, &curr_guess, &x))
                .collect();

            // DISPLAY
            if cmd_display {
                println!("\n --- Turn {turn} --- \n", turn = current_guess + 1);

                println!("Guess 1: {guess}", guess = guess_board[current_guess][0]);
                println!("Guess 2: {guess}", guess = guess_board[current_guess][1]);
                println!("Guess 3: {guess}", guess = guess_board[current_guess][2]);
                println!("Guess 4: {guess}", guess = guess_board[current_guess][3]);

                println!(" ");

                println!("Clues 1: {clue}", clue = answer_board[current_guess][0]);
                println!("Clues 2: {clue}", clue = answer_board[current_guess][1]);
                println!("Clues 3: {clue}", clue = answer_board[current_guess][2]);
                println!("Clues 4: {clue}", clue = answer_board[current_guess][3]);

                println!(" ");

                println!("Correct 1: {correct}", correct = correct_order[0]);
                println!("Correct 2: {correct}", correct = correct_order[1]);
                println!("Correct 3: {correct}", correct = correct_order[2]);
                println!("Correct 4: {correct}", correct = correct_order[3]);

                println!(" ");
                
                println!("Number of possible guesses: {guesses}", guesses = possible_guesses.len());
            }

            // CHECK FOUND

            correct_found = is_all_correct(&answer_board[current_guess]);

            current_guess = current_guess + 1;
        }

        if correct_found {
            if cmd_display {
                println!("\n Congratulation! Correct found!\n");
            }
            turn_won_on[current_guess-1] = turn_won_on[current_guess-1] + 1;
        }else if possible_guesses.len() == 0 {
            if cmd_display {
                println!("\n No more guesses...\n");
            }
            ran_out_of_guesses = ran_out_of_guesses + 1;
        }else{
            if cmd_display {
                println!("\n Didn't find correct in time...\n");
            }
            ran_out_of_time = ran_out_of_time + 1;
        }
    }
    println!("=== STATS ===");
    println!("Times ran out of guesses: {r}", r = ran_out_of_guesses);
    println!("Times ran out of time: {r} \n", r = ran_out_of_time);
    for i in 0..NUM_TURNS {
        println!("Times finished on {index}: \t {r}", index = i+1, r = turn_won_on[i]);
    }

}
