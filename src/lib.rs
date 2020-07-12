/// A set of functions designed to generate probability distributions for the Can't Stop game.
use itertools::join;

const COMBINATIONS: f64 = 1296.0;

/// Simple function that finds the max probability in the vector and returns
/// it attached the name it is associated with.
pub fn find_best_probability(probabilities: Vec<f64>, names: Vec<String>) {
    let mut max: f64 = 0.0;
    let mut location: usize = 0;
    for i in 0..probabilities.len() {
        if probabilities[i] > max {
            location = i;
            max = probabilities[i];
        }
    }
    println!(
        "The largest probability combination is {}, with a probability of {}.",
        names[location], max
    );
}

/// Given a number of combinations to work with this function will generate a
/// list of probabilities for each combination of size num_of_combinations.
pub fn calculate_probabilities_of_reroll_for_all_combinations(
    num_of_combinations: usize,
) -> (Vec<String>, Vec<f64>) {
    let mut probabilities: Vec<f64> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut rolls: Vec<usize> = Vec::new();

    calculate_probabilities_of_reroll_for_all_combinations_helper(
        num_of_combinations,
        &mut probabilities,
        &mut names,
        &mut rolls,
    );
    (names, probabilities)
}

fn calculate_probabilities_of_reroll_for_all_combinations_helper(
    num_of_combinations: usize,
    probabilities: &mut Vec<f64>,
    names: &mut Vec<String>,
    rolls: &mut Vec<usize>,
) {
    if num_of_combinations == 0 {
        names.push(join(rolls.iter(), ", "));
        probabilities.push(calculate_probability_of_reroll(rolls.to_vec()));
    } else {
        let last_roll: usize = match rolls.last() {
            Some(last_roll) => *last_roll,
            None => 1,
        };

        for roll in last_roll + 1..13 {
            rolls.push(roll);
            calculate_probabilities_of_reroll_for_all_combinations_helper(
                num_of_combinations - 1,
                probabilities,
                names,
                rolls,
            );
        }
    }
    rolls.pop();
}

fn calculate_probability_of_reroll(nums: Vec<usize>) -> f64 {
    let mut sum: usize = 0;
    let mut lock: usize = 0;

    for d_1 in 1..7 {
        for d_2 in 1..7 {
            for d_3 in 1..7 {
                for d_4 in 1..7 {
                    for num in &nums {
                        sum += add_one_if_unlocked_and_sums_to_number(&mut lock, d_1 + d_2, *num);
                        sum += add_one_if_unlocked_and_sums_to_number(&mut lock, d_1 + d_3, *num);
                        sum += add_one_if_unlocked_and_sums_to_number(&mut lock, d_1 + d_4, *num);
                        sum += add_one_if_unlocked_and_sums_to_number(&mut lock, d_2 + d_3, *num);
                        sum += add_one_if_unlocked_and_sums_to_number(&mut lock, d_2 + d_4, *num);
                        sum += add_one_if_unlocked_and_sums_to_number(&mut lock, d_3 + d_4, *num);
                    }
                    lock = 0;
                }
            }
        }
    }

    let prob: f64 = sum as f64 / COMBINATIONS;
    prob
}

fn add_one_if_unlocked_and_sums_to_number(lock: &mut usize, sum: usize, num: usize) -> usize {
    if sum == num && *lock == 0 {
        *lock += 1;
        return 1;
    }
    0
}
