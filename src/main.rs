use std::io::stdin;
use rand::Rng;

fn main() {
    let mut list_list: Vec<Vec<String>> = Vec::new();
    let mut is_looping = true;
    while is_looping {

        let mut list: Vec<String> = Vec::new();
        println!("Enter List:");
        loop {
            let mut title = String::new();
            stdin().read_line(&mut title).expect("invalid title");
            match title.trim() {
                "add" => {
                    list.truncate(10);
                    list_list.push(list);
                    println!();
                    break;
                },
                "end" => {
                    list.truncate(10);
                    list_list.push(list);
                    is_looping = false;
                    println!();
                    break;
                },
                _ => {
                    list.push(title.to_string());
                }
            }
        }
    }
    let mut weighted_list: Vec<String> = Vec::new();
    for list in list_list {
        let mut item_count = list.len();
        for item in list {
            let mut weight = item_count;
            while weight > 0 {
                weighted_list.push(item.clone());
                weight -= 1;
            }
            item_count -= 1;
        }
    }
    println!("Result:");
    println!("{}", weighted_list[rand::thread_rng().gen_range(1..weighted_list.len())]);
}
