use std::collections::HashMap;
use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use super::toml_banner::TomlBanner;
use super::banner_rng::Rarity;

#[derive(Debug)]
pub struct Banner{
    pub pool: HashMap<u8, Vec<String>>,
    pub rate_up: HashMap<u8, Vec<String>>,
    pub rarity: Rarity,
    pub(super) banner_type: BannerType
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BannerType {
    Standard,
    Limited
}

#[allow(dead_code)]
impl Banner {
    /// Create new banner from `BannerType`
    /// `BannerType` can be one of the following:
    /// - `BannerType::Standard`
    /// - `BannerType::Limited` <br/>
    /// You have to set operator pool and operator rate up using
    /// [`set_pool`](#method.set_pool) and [`set_rate_up`](#method.set_rate_up) yourself <br/>
    /// The only diffirent is `rate_up_rate`. 70% up for 6 star operator in Limited
    /// and 50% up for 6 star operator in Standard. Both have 50% rate up rate for 5 star operator
    /// 
    /// # Example
    /// ```
    /// use prts::{Banner, BannerType};
    /// 
    /// let mut banner = Banner::from_banner_type(BannerType::Limited);
    /// ```
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

    /// Create new instance of `Banner` by read `.toml` file <br/>
    /// See
    /// [sample file](https://github.com/MinkuruDev/arknights_prts/blob/master/data/operators.toml)
    /// 
    /// # Example
    /// ```
    /// use prts::Banner;
    /// 
    /// let mut banner = Banner::from_file("./data/operators.toml".to_string());
    /// let (star, opname, is_up) = banner.gacha_operator();
    /// println!("{} {} {}", is_up, star, opname);
    /// ```
    pub fn from_file(file_name: String) -> Self{
        Self::from_toml_banner(
            &TomlBanner::from_toml_file(file_name)
        )
    }

    /// Set banner pool with specific operator bound to specific star
    /// 
    /// # Example
    /// ```
    /// use prts::{Banner, BannerType};
    /// 
    /// let mut banner = Banner::from_banner_type(BannerType::Standard);
    /// // set pool 6 star have 2 operators 
    /// banner.set_pool(6, vec!["Silver Ash".to_string(), "Angelina".to_string()]);
    /// // set pool 5 star have 2 operators 
    /// banner.set_pool(5, vec!["Andreana".to_string(), "Projekt Red".to_string()]);
    /// // set pool 4 star have 2 operators 
    /// banner.set_pool(4, vec!["Utage".to_string(), "Myrtle".to_string()]);
    /// // set pool 3 star have 2 operators 
    /// banner.set_pool(3, vec!["Lava".to_string(), "Hibicus".to_string()]);
    /// 
    /// // set rate up 6 star have 2 operator
    /// banner.set_rate_up(6, vec!["Surtr".to_string(), "Skadi".to_string()]);
    /// // set rate up 5 star have 3 operator
    /// banner.set_rate_up(5, vec!["Specter".to_string(), "Ptilopsis".to_string(), "Lappland".to_string()]);
    /// 
    /// let (star, opname, is_up) = banner.gacha_operator();
    /// println!("{} {} {}", is_up, star, opname);
    /// ```
    pub fn set_pool(&mut self, star: u8, operators: Vec<String>) {
        self.pool.insert(star, operators);
    }

    /// Set banner rate up operator with specific operator bound to specific star
    /// 
    /// # Example
    /// ```
    /// use prts::{Banner, BannerType};
    /// 
    /// let mut banner = Banner::from_banner_type(BannerType::Standard);
    /// // set pool 6 star have 2 operators 
    /// banner.set_pool(6, vec!["Silver Ash".to_string(), "Angelina".to_string()]);
    /// // set pool 5 star have 2 operators 
    /// banner.set_pool(5, vec!["Andreana".to_string(), "Projekt Red".to_string()]);
    /// // set pool 4 star have 2 operators 
    /// banner.set_pool(4, vec!["Utage".to_string(), "Myrtle".to_string()]);
    /// // set pool 3 star have 2 operators 
    /// banner.set_pool(3, vec!["Lava".to_string(), "Hibicus".to_string()]);
    /// 
    /// // set rate up 6 star have 2 operator
    /// banner.set_rate_up(6, vec!["Surtr".to_string(), "Skadi".to_string()]);
    /// // set rate up 5 star have 3 operator
    /// banner.set_rate_up(5, vec!["Specter".to_string(), "Ptilopsis".to_string(), "Lappland".to_string()]);
    /// 
    /// let (star, opname, is_up) = banner.gacha_operator();
    /// println!("{} {} {}", is_up, star, opname);
    /// ```
    pub fn set_rate_up(&mut self, star: u8, rateup_operators: Vec<String>){
        self.rate_up.insert(star, rateup_operators);
    }

    /// Simulate Arknights headhunt once <br/>
    /// It have the exact same mechanic in game (or i think that have :)))) ) <br/>
    /// Mechanism: <br/>
    /// In a single headhunt, you have the following change to get an operator:
    /// - 2% to get 6 star operator
    /// - 8% to get 5 star operator
    /// - 50% to get 4 star operator
    /// - 40% to get 3 star operator <br/>
    /// In the first 10 headhunt in a banner, if you don't get a 5 star or higher star operator
    /// the 10th headhunt guaranteed a 5 star operator <br/>
    /// If for 50 headhunt, you don't get a 6 star operator, from the 51st headhunt will increase change 
    /// to get six star operator by 2% (change to get 6 star operator in the 51st headhunt is 4%, 
    /// 52nd is 6%, ...) <br/>
    /// If the result is 5 star operator or 6 star operator, you will have change to get the rate up operator
    /// - 50% to get 5 star rate up operator
    /// - 70% to get 6 star rate up operator in Limited banner
    /// - 50% to get 6 star rate up operator in Standard banner
    /// 
    /// Function returns: (star_result, operator_name, is_up)
    /// - star_result: can be any number in range 3..=6
    /// - operator_name: name of the operator
    /// - is_up: the operator is rate up or not
    /// 
    /// # Example
    /// ```
    /// use prts::Banner;
    /// 
    /// let mut banner = Banner::from_file("./data/operators.toml".to_string());
    /// let (star, opname, is_up) = banner.gacha_operator();
    /// println!("{} {} {}", is_up, star, opname);
    /// ```
    /// 
    /// # Panics
    /// May Panik if operator pool or operator rate up is not set
    /// 
    pub fn gacha_operator(&mut self) -> (u8, String, bool) {
        let (star_result, is_up) = self.rarity.it_gacha_time();
        let operators = 
            if is_up {self.rate_up.get(&star_result).unwrap()}
            else {self.pool.get(&star_result).unwrap()};
        (
            star_result,
            operators.get(thread_rng().gen_range(0..operators.len()))
                        .unwrap().to_owned(),
            is_up
        )
    }

    /// Simulate Arknights headhunt 10 times <br/>
    /// 
    /// # Example
    /// ```
    /// use prts::Banner;
    /// 
    /// let mut banner = Banner::from_file("./data/operators.toml".to_string());
    /// let res = banner.gacha_10_times();
    /// for (star, name, is_up) in &res {
    ///     println!("{} {} {}", is_up, star, name);
    /// }
    /// ```
    /// 
    /// # Panics
    /// May Panik if operator pool or operator rate up is not set
    pub fn gacha_10_times(&mut self) -> Vec<(u8, String, bool)> {
        let mut res = Vec::new();
        for _ in 0..10 {
            res.push(self.gacha_operator());
        }
        res
    }

    /// set `non_six_star_count` and `guarantee_five_star` of `Banner`
    /// can be useful when have many doctor, but need to create only 1 banner
    /// 
    /// # Example
    /// ```
    /// use prts::Banner;
    /// 
    /// let mut banner = Banner::from_file("./data/operators.toml".to_string());
    /// let res = banner.gacha_10_times();
    /// for (star, name, is_up) in &res {
    ///     println!("{} {} {}", is_up, star, name);
    /// }
    /// let old_dokutah_infor = (banner.rarity.non_six_star_count, banner.rarity.guarantee_five_star);
    /// banner.set_dokutah_info(90, 0);
    /// let res = banner.gacha_10_times();
    /// for (star, name, is_up) in &res {
    ///     println!("{} {} {}", is_up, star, name);
    /// }
    /// ```
    pub fn set_dokutah_info(&mut self, non_six_star_count: u8, guarantee_five_star: i8) {
        self.rarity.non_six_star_count = non_six_star_count;
        self.rarity.guarantee_five_star = guarantee_five_star;
    }
}
