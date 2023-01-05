use std::fs;

fn read_f(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&*format!("failed to read file at path: {}", path))
}

fn compute(input: &String) -> Vec<i32> {
    let mut max = Vec::new();
    let mut running_total = 0;
    for line in input.lines() {
        if !line.is_empty() {
            let calories = line.parse::<i32>().unwrap();
            running_total += calories;
        } else {
            if max.len() == 0 {
                max.push(running_total);
            } else if running_total > *max.first().unwrap() {
                
            }

            // if max.len() < 3 {
            //     max.push(running_total);
            // } else {
            //     for i in &mut max {
            //         if running_total > *i {
            //             *i = running_total;
            //             break;
            //         }
            //     }
            // }

            running_total = 0;
        }
    }

    max.sort_by(|a, b| b.cmp(a));

    max
}

const PATH: &str = "./src/day1_calorie_counting/input.txt";
pub(crate) fn solve() -> Vec<i32> {
    compute(&read_f(PATH))
}