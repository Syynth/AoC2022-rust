pub fn find_elf_with_most_calories(list_text: &str) -> i32 {
    let lines_guess = list_text.len() / 40;
    let mut elves = Vec::<i32>::with_capacity(lines_guess);
    elves.push(0);

    let (_, largest_value) = list_text.lines().fold((0, 0), |(index, max), line| {
        if !line.is_empty() {
            if let Ok(calories) = line.parse::<i32>() {
                elves[index] += calories;
            }
            return (index, elves[index].max(max));
        }
        elves.push(0);
        (index + 1, max)
    });

    largest_value
}

pub fn find_top_n_elves_total_calories(list_text: &str, n: usize) -> i32 {
    let lines_guess = list_text.len() / 40;
    let mut elves = Vec::<i32>::with_capacity(lines_guess);
    elves.push(0);

    list_text.lines().fold(0, |index, line| {
        if !line.is_empty() {
            if let Ok(calories) = line.parse::<i32>() {
                elves[index] += calories;
            }
            return index;
        }
        elves.push(0);
        index + 1
    });
    elves.sort_unstable();
    elves.iter().rev().take(n).sum()
}
