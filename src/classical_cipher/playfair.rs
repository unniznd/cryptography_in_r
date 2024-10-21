use std::collections::HashSet;

fn init_key_matrix(key: String) -> Vec<Vec<char>> {
    let mut key_matrix: Vec<Vec<char>> = vec![vec!['a'; 6]; 6];
    let mut seen: HashSet<char> = HashSet::new();
    let mut row: usize = 0;
    let mut col: usize = 0;

    // Fill the key matrix with characters from the key
    for chr in key.chars(){
        key_matrix[row][col] = chr;
        seen.insert(chr);
        col += 1;
        if col == 6 {
            col = 0;
            row += 1;
        }
    }

    for chr in 'a'..='z' {
        if !seen.contains(&chr) {
            key_matrix[row][col] = chr;
            col += 1;
            if col == 6 {
                col = 0;
                row += 1;
            }
        }
    }

    for chr in '0'..='9'{
        if !seen.contains(&chr) {
            key_matrix[row][col] = chr;
            col += 1;
            if col == 6 {
                col = 0;
                row += 1;
            }
        }
    }

    key_matrix
}

fn find_char_pos(key_matrix: &Vec<Vec<char>>, chr: char) -> (usize, usize) {
    for i in 0..6 {
        for j in 0..6 {
            if key_matrix[i][j] == chr {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn preprocess_plaintext(input: &str) -> String {
    let mut result: String = String::new();
    let mut prev: char = ' ';
    for chr in input.chars() {
        if chr == ' ' {
            continue;
        }
        if chr == prev {
            if chr == 'x' {
                result.push('z');
            } else {
                result.push('x');
            }
        }
        result.push(chr);
        prev = chr;
    }
    if result.len() % 2 != 0 {
        if result.chars().last().unwrap() == 'x' {
            result.push('z');
        } else {
            result.push('x');
        }
    }
    result
}

pub fn play_fair(input: String, key: String, is_encrypt: bool) -> String {
    let key_matrix: Vec<Vec<char>> = init_key_matrix(key);
    let mut result: String = String::new();
    let input: String = preprocess_plaintext(&input);
    let mut position1: (usize, usize) = (0, 0);
    let mut position2: (usize, usize);

    println!("Key Matrix:");
    for i in 0..6 {
        for j in 0..6 {
            print!("{} ", key_matrix[i][j]);
        }
        println!();
    }

    for i in 0..input.len() {
        if i % 2 == 0 {
            position1 = find_char_pos(&key_matrix, input.chars().nth(i).unwrap());
        } else {
            position2 = find_char_pos(&key_matrix, input.chars().nth(i).unwrap());
            if position1.0 == position2.0 {
                if is_encrypt {
                    result.push(key_matrix[position1.0][(position1.1 + 1) % 6]);
                    result.push(key_matrix[position2.0][(position2.1 + 1) % 6]);
                } else {
                    result.push(key_matrix[position1.0][(position1.1 + 5) % 6]);
                    result.push(key_matrix[position2.0][(position2.1 + 5) % 6]);
                }
            } else if position1.1 == position2.1 {
                if is_encrypt {
                    result.push(key_matrix[(position1.0 + 1) % 6][position1.1]);
                    result.push(key_matrix[(position2.0 + 1) % 6][position2.1]);
                } else {
                    result.push(key_matrix[(position1.0 + 5) % 6][position1.1]);
                    result.push(key_matrix[(position2.0 + 5) % 6][position2.1]);
                }
            } else {
                result.push(key_matrix[position1.0][position2.1]);
                result.push(key_matrix[position2.0][position1.1]);
            }
        }
    }

    result

    
}
