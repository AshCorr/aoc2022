pub fn day(input: String) -> (String, String) {
    let data = input.split("\n");
    let mut elves: Vec<usize> = Vec::new();

    let mut calories: usize = 0;

    for data_line in data {
        if data_line.len() == 0 {
            elves.push(calories);
            calories = 0;
            continue;
        }

        calories += data_line
            .parse::<usize>()
            .expect(&format!("found non number in data: {}", &data_line))
    }

    elves.sort_by(|a, b| b.cmp(a));

    return (
        elves[0].to_string(),
        elves[0..3].iter().sum::<usize>().to_string(),
    );
}
