use std::cmp::Ordering;
use std::io;

/// A token for stopping adding Divisor-Word combinations and calculating FizzBuzz
const STOP_TOKEN: &str = "N";
/// An error reminding the expected input format
const ERR_BAD_FORMAT: &str =
    "Make sure you input a number and a string separated by comma (e.g 3,Fizz)";
/// An error message used for panicking the program if invalid input is provided at stdin().read_line
const ERR_INVALID_INPUT: &str = "Invalid input!";

/// A struct to represent a Divisor-Word combination (e.g 3,Fizz)
#[derive(Eq)]
struct DivisorWord {
    divisor: u32,
    word: String,
}

impl Ord for DivisorWord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.divisor.cmp(&other.divisor)
    }
}

impl PartialOrd for DivisorWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DivisorWord {
    fn eq(&self, other: &Self) -> bool {
        self.divisor == other.divisor
    }
}

/// Prints a given text, reads a line from standard input and removes whitespace characters
/// # Arguments
/// * `text` - A string literal to be printed
fn read_line(text: &str) -> String {
    println!("{}", text);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(ERR_INVALID_INPUT);
    input.retain(|c| !c.is_whitespace());
    input
}

/// Prints the program FizzBuzz given a list of Divisor-Word combinations and a sample size
/// # Arguments
/// * `divisor_word_list` - A vector of DivisorWord struct
/// * `sample_size` - The last number to test each divisor combination against (inclusive), starting on 1
fn fizz_buzz(divisor_word_list: &Vec<DivisorWord>, sample_size: u32) {
    for num in 1..sample_size + 1 {
        let mut word_list = String::new();
        for DivisorWord { divisor, word } in divisor_word_list {
            if num % divisor == 0 {
                word_list.push_str(&word);
            }
        }
        if word_list == "" {
            word_list.push_str(&num.to_string());
        }
        println!("{}", word_list);
    }
}

/// Given a vector of strings, an index and a boolean flag, it marks the flag as false and returns "" if &vector[index] is not found. Otherwise, it returns its value.
/// # Arguments
/// * `vector` - A vector of string literals
/// * `index` - An index for accessing a position of vector
/// * `is_valid` - A bool flag
fn vec_get_flag<'word>(vector: &Vec<&'word str>, index: usize, is_valid: &mut bool) -> &'word str {
    match vector.get(index) {
        Some(value) => value,
        None => {
            *is_valid = false;
            ""
        }
    }
}

/// Given a vector of strings, an index, a boolean flag and a callback function that also validates, it marks the flag as false and returns "" if &vector[index] is not found.
/// Otherwise, it returns its value and calls callback.
/// # Arguments
/// * `vector` - A vector of string literals
/// * `index` - An index for accessing a position of vector
/// * `is_valid` - A bool flag
/// * `callback` - A function with two parameters, a string literal to be validated/processed and a boolean flag to be called if &vector[index] was found
fn vec_get_flag_w_callback<T>(
    vector: &Vec<&str>,
    index: usize,
    is_valid: &mut bool,
    callback: fn(&str, &mut bool) -> T,
) -> T {
    callback(vec_get_flag(vector, index, is_valid), is_valid)
}

/// Given a string and a boolean flag, it marks the flag as false and returns 0 if number.parse::<u32>() fails. Otherwise, returns divisor.parse::<u32>()
fn validate_number(number: &str, is_valid: &mut bool) -> u32 {
    match number.parse() {
        Ok(value) => value,
        Err(_) => {
            *is_valid = false;
            0
        }
    }
}

/// Gets user input and calculates FizzBuzz
fn main() {
    println!("Welcome to FizzBuzz!");
    let sample_size: u32 = read_line("Input sample size: (100 by default)")
        .parse()
        .unwrap_or(100);
    println!("This is the sample size you chose {}", sample_size);
    let mut divisor_word_list: Vec<DivisorWord> = Vec::new();
    let mut count = 0;
    loop {
        let input = read_line("Input a divisor-word combination separated by comma (e.g 3,Fizz)");
        // Check if the user wants to stop adding combinations and calculate FizzBuzz
        if input.to_uppercase() == STOP_TOKEN {
            break;
        }

        let mut is_valid = true;
        let result_vector: Vec<&str> = input.split(",").collect();

        let DivisorWord { divisor, word } = DivisorWord {
            divisor: vec_get_flag_w_callback(&result_vector, 0, &mut is_valid, validate_number),
            word: vec_get_flag(&result_vector, 1, &mut is_valid).to_string(),
        };
        // Show an error if the user provided values were not valid
        if !is_valid {
            println!("{}", ERR_BAD_FORMAT);
            continue;
        }

        println!("You have added {} for {}", word, divisor);
        divisor_word_list.push(DivisorWord { divisor, word });
        // Remind the user the token to stop adding combinations and calculate FizzBuzz
        if count % 3 == 1 {
            println!("IMPORTANT: Input '{}' to calculate FizzBuzz", STOP_TOKEN);
        }
        count += 1;
    }
    // Sanitize list
    divisor_word_list.sort();
    divisor_word_list.dedup();
    println!("Result of FizzBuzz:");
    fizz_buzz(&divisor_word_list, sample_size);
}
