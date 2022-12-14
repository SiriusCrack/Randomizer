use std::io;
use rand::Rng;
use ureq::get;

fn main() {
    let list = populate_list();
    let list = weight_list(list);
    println!("{}", random_item_from_list(list));
}

fn populate_list() -> Vec<String> {
    let mut list = Vec::new();
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).expect("input should be readable");
        if buffer.trim().len() == 0 { break; }
        list.push(buffer.trim().to_owned());
        buffer.clear();
    }
    list
}

fn weight_list(list: Vec<String>) -> Vec<String> {
    let mut weighted_list: Vec<String> = Vec::new();
    for (i, item) in list.iter().enumerate() {
        for _ in 0..(list.len()-i) {
            weighted_list.push(item.to_owned())
        }
    }
    weighted_list
}

fn random_item_from_list(list: Vec<String>) -> String {
    let result;
    let mut url = "https://www.random.org/integers/?num=1&min=0&col=1&base=10&format=plain&rnd=new&max=".to_string();
    url.push_str(&list.len().to_string()[..]);
    match get(&url).call() {
        Ok(resp) => {
            result = resp.into_string().unwrap().trim().parse().unwrap();
        }
        Err(_) => {
            println!("Random.org request failed. Generating result locally...");
            result = rand::thread_rng().gen_range(0..list.len());
        }
    }
    list[result].to_owned()
}