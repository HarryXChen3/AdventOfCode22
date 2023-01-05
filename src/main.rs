pub mod day1_calorie_counting;

fn main() {
    let calories = day1_calorie_counting::solve::solve();
    for i in &calories {
        println!("{}", i);
    }

    println!("1. {}", calories.get(0).unwrap());
    println!("2. {}", calories.iter().sum::<i32>())
}
