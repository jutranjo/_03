

fn check_if_symbol( character: char ) -> bool {
    !((character.is_digit(10)) || (character=='.'))
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for letter in row {
            print!("{}",letter);
        }
        println!();
    }
}

fn safe_lookup_grid_for_symbol(grid: &Vec<Vec<char>>, line_pos: usize, char_pos: usize) -> Option<char> {
    if let Some(row) = grid.get(line_pos) {
        if let Some(value) = row.get(char_pos) {
            //println!("Safe grid value is {} at {},{}",value,line_pos,char_pos);
            return Some(value.clone())
        }
    }
    None
}

fn check_if_symbol_adjacent(grid: &Vec<Vec<char>>, line_pos: usize, char_pos: usize) -> bool {
    let mut neighbour_is_symbol = false;

    //println!("Checking this element: {:?}", safe_lookup_grid_for_symbol(grid, line_pos, char_pos));

    for line_lookup in (line_pos.checked_sub(1).unwrap_or(0))..=(line_pos+1) {
        for char_lookup in (char_pos.checked_sub(1).unwrap_or(0))..=(char_pos+1) {
            if !(line_lookup==line_pos && char_lookup==char_pos) {
                if let Some(symbol_to_check) = safe_lookup_grid_for_symbol(grid, line_lookup, char_lookup){
                    //println!("Checking this NEIGHBOUR element: {:?}", symbol_to_check);
                    if !neighbour_is_symbol {
                        neighbour_is_symbol = check_if_symbol(symbol_to_check)
                    }
                }
            }
        }
    }
    
    neighbour_is_symbol
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

pub fn sum_up_gears(input: &str) -> u32 {
    let grid = create_grid(input);

    let mut line_index = 0;
    input
        .lines()
        .map(|line| {
            let mut char_index = 0;

            for ch in line.chars() {
                if ch=='*' {
                    println!("Found * at {} {}",line_index,char_index);
                } 
                char_index+=1;
            }
            line_index+=1;
            0
        }).sum()
    
}

pub fn sum_up_schematic(input: &str) -> u32 {

    let grid = create_grid(input);

    let mut line_index = 0;

    if false {
        print_grid(&grid);
    }

    input
        .lines()
        .map(|line| {
            let mut char_index = 0;
            let mut line_sum = 0;
            let mut current_number_string = String::new();

            let mut number_touches_symbol = false;

            for ch in line.chars() {
                if let Some(_digit) = ch.to_digit(10) {
                    current_number_string.push(ch);
                    if check_if_symbol_adjacent(&grid,line_index,char_index) {
                        number_touches_symbol = true;
                    }
                } else {
                    if !current_number_string.is_empty() {
                        if let Ok(parsed_number) = current_number_string.parse::<u32>() {
                            if number_touches_symbol {
                                line_sum += parsed_number;
                            }
                            
                        }
                    current_number_string.clear();
                    number_touches_symbol = false;
                    }
                }
                char_index+=1;
            }
            if !current_number_string.is_empty() {
                if let Ok(parsed_number) = current_number_string.parse::<u32>() {
                    if number_touches_symbol {
                        line_sum += parsed_number;
                    }
                }
            }

            line_index+=1;
            line_sum
        }).sum()
}

#[cfg(test)]
use std::fs;
mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn is_digit_symbol() {
        for testdigit in 0..10 {
            let testchar = char::from_digit(testdigit, 10).unwrap();
            assert_eq!(false, check_if_symbol(testchar));
        }
    }

    #[test]
    fn is_dot_symbol() {
        let testchar = '.';

        assert_eq!(false, check_if_symbol(testchar));
    }

    #[test]
    fn is_star_symbol() {
        let testchar = '*';
        assert_eq!(true, check_if_symbol(testchar));
    }

    #[test]
    fn is_all_alphanumbericsymbols_symbol() {
        let non_alpha_numeric_symbols: Vec<char> = (0..=127)
        .filter_map(|c| std::char::from_u32(c as u32))
        .filter(|&ch| !ch.is_ascii_alphanumeric() && ch != '.')
        .collect();

        

        for &symbol in &non_alpha_numeric_symbols {
            assert_eq!(true, check_if_symbol(symbol));
        }
    }

    #[test]
    fn does_bad_example_work() {
        let input = fs::read_to_string("bad_example.txt").expect("example.txt cannot be opened");
        let grid = create_grid(&input);

        let row = 0;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        
        let row = 1;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);

        
        let row = 2;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);

    }

    #[test]
    fn is_symbol_adjacent_check_for_example() {
        let input = fs::read_to_string("example.txt").expect("example.txt cannot be opened");
        let grid = create_grid(&input);

        print_grid(&grid);

        assert_eq!(check_if_symbol_adjacent(&grid, 2, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, 300, 3), false);

        let row = 0;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);

        
        let row = 1;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);

        
        let row = 2;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);
        
        
        let row = 3;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);
        
        let row = 4;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);

        let row = 5;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);

        
        let row = 6;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);
        
        let row = 7;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);

        
        let row = 8;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);

        
        let row = 9;
        assert_eq!(check_if_symbol_adjacent(&grid, row, 0), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 1), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 2), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 3), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 4), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 5), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 6), true);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 7), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 8), false);
        assert_eq!(check_if_symbol_adjacent(&grid, row, 9), false);


    }
}