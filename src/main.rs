use std::collections::HashMap;

use prts::Banner;

fn main() {
    let mut banner = Banner::from_file("./data/operators.toml".to_string());
    // println!("{:#?}", banner);
    let mut counts: HashMap<u8, u32> = HashMap::new();
    counts.insert(6, 0);
    counts.insert(5, 0);
    counts.insert(4, 0);
    counts.insert(3, 0);
    for _ in 0..100 {
        let (star, opname, is_up) = banner.gacha_operator();
        let st = counts[&star];
        counts.insert(star, st + 1);
        println!("{} {} {}", is_up, star, opname);
    }

}
