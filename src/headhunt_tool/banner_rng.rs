use std::collections::HashMap;
use rand::{thread_rng, Rng};

use crate::BannerType;

#[derive(Debug)]
pub struct Rarity{
    pub(crate) rarity: HashMap<u8, f32>,
    pub(crate) rate_up_rate: HashMap<u8, f32>,
    pub non_six_star_count: u8,
    pub guarantee_five_star: i8,
}

#[allow(dead_code)]
impl Rarity {
    pub fn from_banner_type(banner_type: BannerType) -> Self {
        let mut rarity = HashMap::new();
        rarity.insert(6, 0.02);
        rarity.insert(5, 0.08);
        rarity.insert(4, 0.50);
        let non_six_star_count = 0;
        let guarantee_five_star = 10;
        let mut rate_up_rate = HashMap::new();
        rate_up_rate.insert(5, 0.5);

        match banner_type {
            BannerType::Limited => rate_up_rate.insert(6, 0.70),
            BannerType::Standard => rate_up_rate.insert(6, 0.50)
        };

        Self{
            rarity, rate_up_rate, 
            non_six_star_count, guarantee_five_star
        }
    }

    pub fn it_gacha_time(&mut self) -> (u8, bool) {
        let mut res: f32 = thread_rng().gen_range(0.0..1.0);
        let mut extra_rate = 
            if self.non_six_star_count <= 50 {0.0}
            else {0.02 * ((self.non_six_star_count - 50) as f32)};
        let mut star_result = 0;
        for star in (3..=6).rev() {
            let rate = extra_rate + self.rarity.get(&star).unwrap_or(&1.0).to_owned();
            extra_rate = 0.0;
            if res < rate {
                star_result = star;
                break;
            }
            res -= rate;
        }

        self.non_six_star_count = 
            if star_result == 6 {0}
            else {self.non_six_star_count + 1};

        self.guarantee_five_star -= 1;
        if(self.guarantee_five_star == 0) && (star_result < 5){
            star_result = 5;
        }

        let mut is_up = false;
        if star_result >= 5 {
            self.guarantee_five_star = -1;
            is_up = if thread_rng().gen_range(0.0..1.0) < 
                        self.rate_up_rate.get(&star_result).unwrap_or(&0.0).to_owned()
                    {true} else {false}
        }

        (star_result, is_up)
    }
}
