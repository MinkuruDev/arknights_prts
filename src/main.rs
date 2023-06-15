use prts::Banner;

fn main() {
    let mut banner = Banner::from_file("./data/operators.toml".to_string());
    let res = banner.gacha_10_times();
    for (star, name, is_up) in &res {
        println!("{} {} {}", is_up, star, name);
    }
    banner.set_dokutah_info(90, 0);
    let res = banner.gacha_10_times();
    for (star, name, is_up) in &res {
        println!("{} {} {}", is_up, star, name);
    }
}
