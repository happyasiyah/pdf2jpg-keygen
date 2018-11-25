extern crate rand;
extern crate promptly;

use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use promptly::prompt;

static CHAR_LIST: [&str; 10] = ["Z", "S", "Q", "J", "W", "L", "D", "T", "M", "X"];

#[derive(PartialEq)]
enum Edition {
    //Single License 1 PC
    SG,
    //Personal License 3 PCs
    PS,
    //Home License 10 PCs
    HM,
    //Team License (> 10 PCs)
    TM,
    //Enterprise License (>100 PCs)
    EP,
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Edition::SG => write!(f, "SG"),
            Edition::PS => write!(f, "PS"),
            Edition::HM => write!(f, "HM"),
            Edition::TM => write!(f, "TM"),
            Edition::EP => write!(f, "EP"),
        }
    }
}


fn num_to_string(num: u32) -> String {
    let mut n = num;
    let mut digits = Vec::new();
    let mut num_vec = Vec::new();

    if num < 1000000 && num >= 11 {
        if num < 100 {
            num_vec.push("ZZZZ")
        } else if num < 1000 {
            num_vec.push("ZZZ")
        } else if num < 10000 {
            num_vec.push("ZZ")
        }else if num <100000 {
            num_vec.push("Z")
        }

        while n > 0 {
            digits.push(CHAR_LIST[(n % 10) as usize]);
            n /= 10;
        }
        digits.reverse();
        num_vec.append(digits.as_mut())
    } else {
        num_vec.push("MMMMQQ")
    }

    num_vec.join("").to_string()
}


fn get_rand(list: &[char]) -> char {
    let mut rng = thread_rng();
    let choice = *list.choose(&mut rng).unwrap();
    choice
}


fn keygen(edition: Edition, no_of_pcs: u32) -> String {
    let mut key = String::new();
    let mut chr: char;
    key.push_str("NGIP-");
	key.push_str(&edition.to_string());

    chr = get_rand(&['Z', 'S', 'Q']);
    key.push(chr);
 	match chr {
        'Z' | 'S' => key.push(get_rand(&['Z', 'S', 'Q', 'J', 'W', 'L', 'D', 'T', 'M', 'X'])),
        'Q' => key.push(get_rand(&['Z', 'S', 'Q', 'J'])),
        _ => println!("error!")
    }
    key.push('-');

    chr = get_rand(&['Z', 'S', 'Q', 'J']);
    key.push(chr);
    match chr {
        'Z' => key.push(get_rand(&['S', 'Q', 'J', 'W', 'L', 'D', 'T', 'M', 'X'])),
        'S' | 'Q' => key.push(get_rand(&['Z', 'S', 'Q', 'J', 'W', 'L', 'D', 'T', 'M', 'X'])),
        'J' => key.push(get_rand(&['Z', 'S'])),
        _ => println!("error!")
    }

    chr = get_rand(&['Z', 'S']);
    key.push(chr);
    match chr {
        'Z' => key.push(get_rand(&['S', 'Q', 'J', 'W', 'L', 'D', 'T', 'M', 'X'])),
        'S' => key.push(get_rand(&['S', 'Q', 'J'])),
        _ => println!("error!")
    }

    key.push('-');
    key.push(get_rand(&['Z', 'S', 'Q', 'J', 'W', 'L']));
    key.push(get_rand(&['Z', 'S', 'Q', 'J', 'W', 'L', 'D', 'T', 'M', 'X']));
    key.push(get_rand(&['Z', 'S', 'Q', 'J', 'W', 'L']));
    key.push(get_rand(&['Z', 'S', 'Q', 'J', 'W', 'L', 'D', 'T', 'M', 'X']));

    match edition {
        Edition::TM | Edition::EP => {
            key.push('-');
            key.push_str(&num_to_string(no_of_pcs))
        }
        _ => ()
    }
    key
}

fn main() {
    let i: u32;
    println!("License Generator for PDF to JPG");
    i = prompt("License for Number of PCs (1-999999)");

    if i <= 1 {
        println!("License: {}", keygen(Edition::SG, i));
    } else if i < 4 {
        println!("License: {}", keygen(Edition::PS, i));
    } else if i < 11 {
        println!("License: {}", keygen(Edition::HM, i));
    } else if i < 101 {
        println!("License: {}", keygen(Edition::TM, i));
    } else {
        println!("License: {}", keygen(Edition::EP, i));
    }
}