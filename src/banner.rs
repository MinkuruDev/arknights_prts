use std::collections::HashMap;
use rand::Rng;
use serde::{Serialize, Deserialize};
use super::toml_banner::TomlBanner;
use super::banner_rng::Rarity;

#[derive(Debug)]
pub(super) struct Banner{
    pub(crate) pool: HashMap<u8, Vec<String>>,
    pub(crate) rate_up: HashMap<u8, Vec<String>>,
    pub(crate) rarity: Rarity,
    pub(crate) banner_type: BannerType
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub(crate) enum BannerType {
    Standard,
    Limited
}

#[allow(dead_code)]
impl Banner {
    pub fn from_banner_type(banner_type: BannerType) -> Self{
        Self {
            pool: HashMap::new(),
            rate_up: HashMap::new(),
            rarity: Rarity::from_banner_type(banner_type),
            banner_type
        }
    }

    fn from_toml_banner(banner: &TomlBanner) -> Self{
        let mut pool = HashMap::new();
        for (k, v) in banner.pool.iter() {
            pool.insert(k.parse::<u8>().unwrap(), v.to_owned());
        };

        let mut rate_up = HashMap::new();
        for (k, v) in banner.rate_up.iter() {
            rate_up.insert(k.parse::<u8>().unwrap(), v.to_owned());
        };

        Self {
            pool, rate_up,
            banner_type: banner.banner_type,
            rarity: Rarity::from_banner_type(banner.banner_type)
        }
    }

    pub fn from_file(file_name: String) -> Self{
        Self::from_toml_banner(
            &TomlBanner::from_toml_file(file_name)
        )
    }

    pub fn set_pool(&mut self, star: u8, operators: Vec<String>) {
        self.pool.insert(star, operators);
    }

    /// Simulate Arknights headhunt ONCE
    /// Return (star_result, operator_name, is_up)
    /// - star_result: can be any number in range 3..=6
    /// - operator_name: name of the operator
    /// - is_up: the operator is rate up or not
    pub fn gacha_operator(&mut self) -> (u8, String, bool) {
        let (star_result, is_up) = self.rarity.it_gacha_time();
        let operators = 
            if is_up {self.rate_up.get(&star_result).unwrap()}
            else {self.pool.get(&star_result).unwrap()};
        (
            star_result,
            operators.get(self.rarity.rng.gen_range(0..operators.len()))
                        .unwrap().to_owned(),
            is_up
        )
    }

    pub fn gacha_10_times(&mut self) -> Vec<(u8, String, bool)> {
        let mut res = Vec::new();
        for _ in 0..10 {
            res.push(self.gacha_operator());
        }
        res
    }
}
