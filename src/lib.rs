use std::fs;

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

fn check_if_symbol_adjacent(grid: &Vec<Vec<char>>, char_pos: usize, line_pos: usize) -> bool {
    //Check all 9 neighbours
    
    check_if_symbol(grid[line_pos][char_pos])
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    grid
}

pub fn sum_up_schematic(input: &str) -> u32 {

    let grid = create_grid(input);
    print_grid(&grid);

    let line_pos = 1;
    let char_pos = 3;
    let xy_pos_letter = grid[line_pos][char_pos];
    println!("character at {},{} is {},[ is it a symbol?{} ][is it adjacent to symbol? {}]",
            line_pos,
            char_pos,
            xy_pos_letter,
            check_if_symbol(xy_pos_letter),
            check_if_symbol_adjacent(&grid, char_pos, line_pos));


    return 0;

}

#[cfg(test)]

mod tests {
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
    fn is_symbol_adjacent() {
        let input = fs::read_to_string("example.txt").expect("example.txt cannot be opened");
        let grid = create_grid(&input);

        let linepos = 3;
        let charpos = 3;
        assert_eq!(check_if_symbol_adjacent(&grid, linepos, charpos), true)
    }
}