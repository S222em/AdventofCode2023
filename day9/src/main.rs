use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<_> = file.split("\n").collect();

    let mut sum: isize = 0;

    for line in lines {
        let history: Vec<isize> = line.split_whitespace().map(|str| str.parse().unwrap()).collect();

        sum += previous(&history);
    }

    println!("{}", sum);
}

fn previous(history: &[isize]) -> isize {
    if history.iter().all(|&item| item == 0) {
        return 0;
    }

    let differences = get_differences(history);
    let difference = previous(&differences);

    history.first().unwrap() - difference
}

fn get_differences(history: &[isize]) -> Vec<isize> {
    history.windows(2).map(|window| window[1] - window[0]).collect()
}


