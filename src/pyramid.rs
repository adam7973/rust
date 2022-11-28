#[allow(dead_code)]
pub fn pyramid2(lines: i32, start: &str) {
    let strings = start.split(' ');
    let mut numbers: Vec<i32> = Vec::new();
    for s in strings {
        if s.chars().count() > 0 {
            numbers.push(s.parse().expect("not a number"))
        }
    }
    let mut number_lists: Vec<Vec<i32>> = vec![numbers];
    for line in 0..lines + 1 {
        number_lists.push(number_lists.get(line as usize).expect("blyat").clone());
        println!(
            "row {}: {}",
            line + 1,
            print_veci32(number_lists.get(line as usize).expect("blyat"))
        );
        let mut index_shift = 0;
        for n in 0..number_lists[line as usize].len() - 1 {
            if number_lists[line as usize][n] + number_lists[line as usize][n + 1] == line + 2 {
                number_lists[(line + 1) as usize].insert(n + index_shift + 1, line + 2);
                index_shift += 1;
            }
        }
    }
}

fn print_veci32(vec: &Vec<i32>) -> String {
    let mut line = String::new();
    line += "[";
    for int in vec {
        line.push_str(&format!("{}", int))
    }
    line += "]";
    line
}
