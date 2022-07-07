use std::io;
use rand::Rng;

fn main() {
    let mut list: Vec<String> = vec![];
    
    loop {
        let mut title = String::new();
        io::stdin().read_line(&mut title).expect("invalid title");
        if title.trim().to_string() == "end" {
            break;
        }
        list.push(title.trim().to_string());
    }
    let mut list_size: u16 = list.len() as u16;
    let mut weighted_list: Vec<String> = vec![];
    for title in list {
        let mut i = list_size;
        loop {
            weighted_list.push(title.to_string());
            i -= 1;
            if i == 0 {
                break;
            }
        }
        list_size -= 1;
    }
    println!("{}", weighted_list[rand::thread_rng().gen_range(1..weighted_list.len())]);
}
