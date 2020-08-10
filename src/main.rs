use std::cmp::Ordering;
use std::io;

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

fn read_line(text: &str) -> String {
    println!("{}", text);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input!");
    input.retain(|c| !c.is_whitespace());
    input
}

fn fizz_buzz(lcm_word_list: &Vec<DivisorWord>, sample_size: u32) {
    for i in 1..sample_size + 1 {
        let mut check_lcm = String::new();
        for lcm_word in lcm_word_list {
            if i % lcm_word.divisor == 0 {
                check_lcm.push_str(&lcm_word.word);
            }
        }
        if check_lcm == "" {
            check_lcm.push_str(&i.to_string());
        }
        println!("{}", check_lcm);
    }
}

fn main() {
    println!("Welcome to FizzBuzz!");

    let sample_size: u32 = match read_line("Input sample size: (100 by default)") {
        input if input.parse::<u32>().is_ok() => input.parse().unwrap(),
        _ => 100,
    };

    println!("This is the sample size you chose {}", sample_size);
    let mut lcm_word_map: Vec<DivisorWord> = Vec::new();
    let mut done = false;
    let mut count = 0;
    while !done {
        let input =
            read_line("Input a divisor and a word combination separated by comma (e.g 3,Fizz):");
        done = input.to_uppercase() == "N";
        if !done {
            let result_vector: Vec<&str> = input.split(",").collect();
            let data = DivisorWord {
                divisor: (&result_vector)[0]
                    .parse()
                    .expect("Make sure you input a number and a string separated by comma"),
                word: (&result_vector)[1].to_string(),
            };
            println!("You have added {} for {}", data.word, data.divisor);
            if count % 3 == 1 {
                println!("IMPORTANT: Type 'n' at any time to stop adding cominations");
            }
            lcm_word_map.push(data);
        }
        count += 1;
    }

    // Sanitize list
    lcm_word_map.sort();
    lcm_word_map.dedup();
    println!("Result of FizzBuzz:");
    fizz_buzz(&lcm_word_map, sample_size);
}
