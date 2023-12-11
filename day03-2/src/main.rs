use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("src/puzzle_input.txt").unwrap();

    let values = parse_values(&puzzle_input);
    let symbols = parse_symbols(&puzzle_input);
    let filtered = symbols.iter().filter(|(row, id)| {
        values
            .iter()
            .filter(|value| value.is_next_to(*row, *id))
            .count()
            == 2
    });

    let mut sum = 0;
    for (row, id) in filtered {
        let mut gear_neighbors = values.iter().filter(|x| x.is_next_to(*row, *id));

        sum += gear_neighbors.next().unwrap().value * gear_neighbors.next().unwrap().value
    }

    println!("{sum}")
}

#[derive(Debug)]
pub struct Value {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

impl Value {
    pub fn increase_value(&mut self, value_to_append: u32) {
        self.end += 1;
        self.value = 10 * self.value + value_to_append
    }

    pub fn is_next_to(&self, row: usize, id: usize) -> bool {
        (row + 1 >= self.row && row <= self.row + 1) && (id + 1 >= self.start && id <= self.end + 1)
    }
}

pub fn parse_values(str: &str) -> Vec<Value> {
    let mut value_list = Vec::new();
    for (row, row_str) in str.lines().enumerate() {
        let digits = row_str
            .chars()
            .enumerate()
            .filter(|(_, char)| char.is_ascii_digit());
        let a: Vec<(usize, char)> = digits.collect();
        value_list.append(&mut create_values_row(a, row));
    }
    value_list
}

pub fn create_values_row(values_row: Vec<(usize, char)>, row: usize) -> Vec<Value> {
    let mut return_list: Vec<Value> = Vec::new();

    for (id, value_) in values_row {
        let value_ = format!("{value_}").parse().unwrap();
        match return_list.last_mut() {
            Some(value) if value.end + 1 == id => value.increase_value(value_),
            _ => return_list.push(Value {
                value: value_,
                row,
                start: id,
                end: id,
            }),
        }
    }

    return_list
}

pub fn parse_symbols(str: &str) -> Vec<(usize, usize)> {
    let mut id_list = Vec::new();
    for (row, row_str) in str.lines().enumerate() {
        let mut ids_row = row_str
            .chars()
            .enumerate()
            .filter(|(_, char)| char == &'*')
            .map(|(id, _)| (row, id))
            .collect();
        id_list.append(&mut ids_row)
    }
    id_list
}
