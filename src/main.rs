/// Simple driver class that generates the probability distributions and the
/// best combination for combinations of length 1, 2, or 3.
use cant_stop_analysis;

fn main() {
    let (names, probabilities) =
        cant_stop_analysis::calculate_probabilities_of_reroll_for_all_combinations(1);
    for (name, prob) in names.iter().zip(probabilities.iter()) {
        println!("{}: {}", name, prob);
    }
    cant_stop_analysis::find_best_probability(probabilities, names);
    println!();
    let (names, probabilities) =
        cant_stop_analysis::calculate_probabilities_of_reroll_for_all_combinations(2);
    for (name, prob) in names.iter().zip(probabilities.iter()) {
        println!("{}: {}", name, prob);
    }
    cant_stop_analysis::find_best_probability(probabilities, names);
    println!();
    let (names, probabilities) =
        cant_stop_analysis::calculate_probabilities_of_reroll_for_all_combinations(3);
    for (name, prob) in names.iter().zip(probabilities.iter()) {
        println!("{}: {}", name, prob);
    }
    cant_stop_analysis::find_best_probability(probabilities, names);
}
