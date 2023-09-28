extern crate core;

use std::collections::HashMap;
use crate::module_bank::{BankManager, StuffType};
use crate::module_card::CardPool;
use crate::module_game::Game;

///定义了一种可以高度通用的银行容器，可以实现物资交换
mod module_bank {
    use std::collections::HashMap;
    use std::fmt;
    use std::fmt::Display;
    use super::module_card::*;
    use rand::prelude::*;

    /// 东西的类型
    #[derive(Hash)]
    #[derive(Eq, PartialEq)]
    #[derive(Clone)]
    pub enum StuffType {
        GeneralType(&'static str,i32),
        CardType(Card),
    }

    /// 我的银行
    #[derive(Clone)]
    pub struct MyBank {
        basket:HashMap<StuffType, i32>,
    }

    /// 银行管理特性
    pub trait BankManager<T,U>{

        /// 接口：将任意数量单位、任意类型的东西从某一家银行传给另一家银行
        fn stuff_transfer(&mut self, what:&T, how_many:U, destination:&mut Self) -> Result<String, String>;

        /// 接口：将任意数量单位、任意类型的东西加入篮子
        fn stuff_in(&mut self, what:&T, how_many:U) -> Result<String, String>;

        /// 接口：将任意数量单位、任意类型的东西拿出篮子
        fn stuff_out(&mut self, what:&T, how_many:U) -> Result<String, String>;
    }

    /// 给东西实现Display特性
    impl Display for StuffType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                StuffType::GeneralType(string, value) => write!(f, "面值{0}$的{1}", value, string),
                StuffType::CardType(card) => write!(f, "{0}", card),
            }
        }
    }
    /// 给获取东西价值
    impl StuffType {
        pub fn get_value(&self) -> i32 {
            return match self {
                StuffType::GeneralType(.., value) => *value,
                StuffType::CardType(..) => 0,
            }
        }
    }

    /// 银行实现特性
    impl BankManager<StuffType,i32> for MyBank {
        /// 接口：将任意数量单位、任意类型的东西从某一家银行传给另一家银行
        fn stuff_transfer (&mut self, what:&StuffType, how_many:i32, destination:&mut Self) -> Result<String, String> {
            return match self.stuff_out(what, how_many) {
                Ok(T) => {
                    match destination.stuff_in(what, how_many) {
                        Ok(T1) => Ok(T + &T1),
                        Err(E) => {
                            match self.stuff_in(what, how_many) {
                                Ok(T1) => Err(E + &T1 + "The stuff succeeds to get back!"),
                                Err(E1) => Err(E + &E1 + "The stuff fails to get back!"),
                            }
                        }
                    }
                },
                Err(E) => Err(E)
            }
        }

        /// 接口：将任意数量单位、任意类型的东西加入篮子
        fn stuff_in (&mut self, what:&StuffType, how_many:i32) -> Result<String, String> {
            let basket = &mut (self.basket);
            if how_many > 0 {
                if let Some(now_many) = basket.insert((*what).clone(), how_many) {
                    let temp_many = now_many + how_many;
                    if temp_many > 0 {
                        let no_use = basket.insert((*what).clone(), temp_many);
                        Ok(format!("Succeed to do stuff_in by inserting {0} {1} to basket that contains {2} {1}!", how_many, what, now_many))
                    } else if temp_many == 0 {
                        let no_use = basket.remove(what);
                        Ok(format!("Succeed to do stuff_in by inserting {0} {1} to basket that contains {2} {1}!", how_many, what, now_many))
                    } else {
                        let no_use = basket.insert((*what).clone(), now_many);
                        Err(format!("Fail to do stuff_in because inserting {0} {1} to basket that contains {2} {1} is invalid!", how_many, what, now_many))
                    }
                } else {
                    Ok(format!("Succeed to do stuff_in by inserting {0} {1} to basket that contains no {1}!", how_many, what))
                }
            } else if how_many == 0 {
                Ok(format!("There's nothing to do because {0} {1} is being inserted!", how_many, what))
            } else {
                Err(format!("Fail to do stuff_in because inserting {0} {1} to basket is invalid!", how_many, what))
            }
        }

        /// 接口：将任意数量单位、任意类型的东西拿出篮子
        fn stuff_out (&mut self, what:&StuffType, how_many:i32) -> Result<String, String> {
            let basket = &mut (self.basket);
            if how_many > 0 {
                if let Some(now_many) = basket.insert((*what).clone(), how_many) {
                    let temp_many = now_many - how_many;
                    if temp_many > 0 {
                        let no_use = basket.insert((*what).clone(), temp_many);
                        Ok(format!("Succeed to do stuff_out by removing {0} {1} from basket that contains {2} {1}!", how_many, what, now_many))
                    } else if temp_many == 0 {
                        let no_use = basket.remove(what);
                        Ok(format!("Succeed to do stuff_out by removing {0} {1} from basket that contains {2} {1}!", how_many, what, now_many))
                    } else {
                        let no_use = basket.insert((*what).clone(), now_many);
                        Err(format!("Fail to do stuff_out because removing {0} {1} from basket that contains {2} {1} is invalid!", how_many, what, now_many))
                    }
                } else {
                    let no_use = basket.remove(what);
                    Err(format!("Fail to do stuff_out because removing {0} {1} from basket that contains no {1} is invalid!", how_many, what))
                }
            } else if how_many == 0 {
                Ok(format!("There's nothing to do because {0} {1} is being removed!", how_many, what))
            } else {
                Err(format!("Fail to do stuff_out because removing {0} {1} to basket is invalid!", how_many, what))
            }
        }
    }

    /// 银行实现特性
    impl BankManager<Vec<(&StuffType, i32)>,bool> for MyBank {
        /// 接口：将一批东西从某一家银行传给另一家银行，并确认如果中途失败是否回溯
        fn stuff_transfer (&mut self, what:&Vec<(&StuffType,i32)>, how_many:bool, destination:&mut Self) -> Result<String, String> {
            return match self.stuff_out(what, how_many) {
                Ok(T) => {
                    match destination.stuff_in(what, how_many) {
                        Ok(T1) => Ok(T+&T1),
                        Err(E) => {
                            match self.stuff_in(what, false) {
                                Ok(T1) => Err(E + &T1),
                                Err(E1) => Err(E + &E1),
                            }
                        }
                    }
                },
                Err(E) => Err(E)
            }
        }

        /// 接口：将一批东西加入篮子，并确认如果中途失败是否回溯
        fn stuff_in (&mut self, what:&Vec<(&StuffType,i32)>, how_many:bool) -> Result<String, String> {
            return if how_many {
                let mut stack: Vec<(&StuffType, i32)> = Vec::new();
                let mut t_stack = String::new();
                for stuff in what.iter() {
                    match self.stuff_in(stuff.0, stuff.1) {
                        Err(E) => {
                            return match self.stuff_out(&stack, false) {
                                Err(E1) => Err(t_stack + &E + &E1 + "The stuff fails to send back!"),
                                Ok(T) => Err(t_stack + &E + &T + "The stuff succeeds to send back!")
                            }
                        }
                        Ok(T) => {
                            stack.push(*stuff);
                            t_stack.push_str(&(T));
                        }
                    }
                }
                Ok(t_stack)
            } else {
                let mut t_stack = String::new();
                for stuff in what.iter() {
                    match self.stuff_in(stuff.0, stuff.1) {
                        Err(E) => return Err(t_stack + &E + "The stuff in is reserved!"),
                        Ok(T) => {
                            t_stack.push_str(&(T));
                        }
                    }
                }
                Ok(t_stack)
            }
        }

        /// 接口：将一批东西拿出篮子，并确认如果中途失败是否回溯
        fn stuff_out (&mut self, what:&Vec<(&StuffType,i32)>, how_many:bool) -> Result<String, String> {
            return if how_many {
                let mut stack: Vec<(&StuffType, i32)> = Vec::new();
                let mut t_stack = String::new();
                for stuff in what.iter() {
                    match self.stuff_out(stuff.0, stuff.1) {
                        Err(E) => {
                            return match self.stuff_in(&stack, false) {
                                Err(E1) => Err(t_stack + &E + &E1 + "The stuff fails to get back!"),
                                Ok(T) => Err(t_stack + &E + &T + "The stuff succeeds to get back!")
                            }
                        }
                        Ok(T) => {
                            stack.push(*stuff);
                            t_stack.push_str(&(T));
                        }
                    }
                }
                Ok(t_stack)
            } else {
                let mut t_stack = String::new();
                for stuff in what.iter() {
                    match self.stuff_out(stuff.0, stuff.1) {
                        Err(E) => return Err(t_stack + &E + "The stuff out is lost!"),
                        Ok(T) => {
                            t_stack.push_str(&(T));
                        }
                    }
                }
                Ok(t_stack)
            }
        }
    }

    /// 银行构造函数
    impl MyBank {
        pub fn new() -> MyBank {
            let bank = MyBank {
                basket:HashMap::new(),
            };
            return bank;
        }
        pub fn read_random_item(&self) -> &StuffType {
            let basket_vec:Vec<(&StuffType, i32)> = self.get_basket_vec();
            let length = basket_vec.len();
            let mut rng = thread_rng();
            let rand_num = rng.gen_range(0..length);
            return basket_vec[rand_num].0
        }
        pub fn get_basket_vec(&self) -> Vec<(&StuffType, i32)> {
            let basket_vec_temp:Vec<(&StuffType, &i32)> = self.basket.iter().collect();
            let mut basket_vec:Vec<(&StuffType, i32)> = Vec::new();
            for i in basket_vec_temp.iter() {
                basket_vec.push(((*i).0, *((*i).1)));
            }
            return basket_vec;
        }
        pub fn stuff_clear(&mut self) -> Result<String, String> {
            self.basket = HashMap::new();
            Ok("Succeed to do stuff_clear!".to_string())
        }
        pub fn get_values_of_bank(&self) -> i32 {
            let basket_vec = self.get_basket_vec();
            let mut values = 0;
            for item in basket_vec {
                values += item.0.get_value() * item.1;
            }
            return values;
        }
        /// 根据价值，将自己银行的东西用最大面额组合起来
        pub fn collect_stuff(&mut self, value:i32) -> Result<Vec<(StuffType,i32)>, String> {
            let mut sort_basket = self.get_basket_vec();
            let fun = |a:&(&StuffType,i32),b:&(&StuffType,i32)| {
                let x1 = a.0.get_value();
                let x2 = b.0.get_value();
                x2.cmp(&x1)
            };
            // 获取按照面值排序的篮子东西
            sort_basket.sort_by(fun);


            // 计算所有东西的个数
            let sort_basket_clone = sort_basket.clone();
            let mut total_num:usize = 0;
            for item in sort_basket_clone.iter() {
                total_num += item.1 as usize;
            }
            // 把排序的篮子东西按照各种类个数展开
            let mut sort_basket_full: Vec<&StuffType> = Vec::new();
            let mut index = 0;
            let mut cnt = sort_basket_clone[index].1;
            for epoch in 0..total_num {
                sort_basket_full.push(sort_basket_clone[index].0);
                cnt -= 1;
                if cnt <= 0 {
                    index += 1;
                    if index < sort_basket_clone.len() {
                        cnt = sort_basket_clone[index].1;
                    }
                }
            }
            // 跑所有的组合
            let sort_basket_full_clone = sort_basket_full.clone();
            let mut basket_now: Vec<&StuffType> = Vec::new();
            let get_vec_value = |a: &Vec<&StuffType>| -> i32 {
                let mut value_total = 0;
                for item in a.iter() {
                    value_total += item.get_value();
                }
                return value_total;
            };
            let mut value_now = 0;
            'outer:for epoch in 0..total_num {
                basket_now.clear();
                let mut index = epoch;
                basket_now.push(sort_basket_full_clone[index]);
                'inner:loop {
                    // 计算当前总价值
                    value_now = get_vec_value(&basket_now.clone());
                    if value_now > value {
                        // 价值大了，去掉一个
                        basket_now.pop();
                        if basket_now.len() == 0 {
                            // 去掉的就是第一个，那么直接开始下一代
                            break 'inner;
                        } else {
                            index += 1;
                            if index >= total_num {
                                // 后面没法再补充了，那么直接开始下一代
                                break 'inner;
                            } else {
                                // 补充下一个，开始下一次循环
                                basket_now.push(sort_basket_full_clone[index]);
                                continue 'inner;
                            }
                        }
                    } else if value_now < value {
                        // 价值小了，继续加下一个
                        index += 1;
                        if index >= total_num {
                            // 后面没法再补充了，那么直接开始下一代
                            break;
                        } else {
                            // 补充下一个，开始下一次循环
                            basket_now.push(sort_basket_full_clone[index]);
                            continue;
                        }
                    } else {
                        // 刚好满足需求
                        break 'outer;
                    }
                }
            }
            return if value_now == value {
                let mut bank_temp = MyBank::new();
                let basket_now_clone = basket_now.clone();
                for stuff in basket_now_clone.iter() {
                    let no_use = bank_temp.stuff_in(*stuff, 1);
                }
                let bank_temp_clone = bank_temp.clone();
                let result = bank_temp_clone.get_basket_vec();
                let mut result_processed:Vec<(StuffType,i32)> = Vec::new();
                for item in result.iter() {
                    result_processed.push(((item.0).clone(), item.1));
                }
                Ok(result_processed)
            } else {
                Err(format!("Fail to collect stuffs with value of {0}", value))
            }
        }
    }

    /// 给银行实现Display特性
    impl Display for MyBank {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "【")?;
            let mut cnt = 0;
            for p in self.basket.iter() {
                if cnt==0 {
                    write!(f,"{0}个{1}", p.1, p.0)?;
                    cnt += 1;
                } else {
                    write!(f,"，{0}个{1}", p.1, p.0)?; // p是元组
                    cnt += 1;
                }
            }
            if cnt >= 1 {
                write!(f,"】")
            } else {
                write!(f,"空】")
            }
        }
    }
}

mod module_player {
    use std::cmp::Ordering;
    use super::module_bank::*;
    use super::module_card::*;
    use std::fmt;
    use std::fmt::Display;
    use rand::prelude::*;

    /// 玩家
    #[derive(Clone)]
    pub struct Player {
        pub name:String,
        pub role:Role,
        pub owned_bank:MyBank,
        pub bet_bank:MyBank,
        pub cards_bank:MyBank,
    }

    /// 玩家的方法
    impl Player {
        /// 下注
        pub fn place_a_bet(&mut self, bet:&Vec<(&StuffType,i32)>) -> Result<String, String> {
            self.owned_bank.stuff_transfer(bet,true,&mut (self.bet_bank))
        }
        /// 下注指定的value
        pub fn place_a_bet_with_value(&mut self, value:i32) -> Result<String, String> {
            let mut te_stack = String::new();
            // 把bet拿回来
            match self.get_bets_back() {
                Ok(T) => te_stack.push_str(&T),
                Err(E) => te_stack.push_str(&E),
            }
            let mut owned_bank_clone = self.owned_bank.clone();
            match owned_bank_clone.collect_stuff(value) {
                Ok(result) => {
                    let mut result_process = Vec::new();
                    for item in result.iter() {
                        result_process.push((&(item.0), item.1));
                    }
                    match self.place_a_bet(&result_process) {
                        Ok(T) => {
                            te_stack.push_str(&T);
                            self.role = Role::PlaceBet;
                            return Ok(te_stack);
                        },
                        Err(E) => {
                            te_stack.push_str(&E);
                        },
                    }
                }
                Err(E) => {
                    te_stack.push_str(&E);
                }
            }
            self.role = Role::GiveUp;
            return Err(te_stack);
        }

        /// 根据上一者下注value自动跟注、加注或放弃的决策
        pub fn place_a_bet_with_last_value(&mut self, last_player_value:i32, max_bet_value:i32, min_value_unit:i32) -> Result<String, String> {
            let mut te_stack = String::new();
            let mut rng = thread_rng();
            let rand_num = rng.gen_range(1..=100);
            // 有20%概率放弃
            if rand_num <= 20 {
                self.role = Role::GiveUp;
                return Err(te_stack);
            }
            for times in 0..20 {
                let mut rng = thread_rng();
                let rand_num = rng.gen_range(1..=100);
                let mut value = last_player_value;
                let mut d_value = 0;
                if rand_num <= 50 {
                    // 有50%概率跟注
                    d_value = 0;
                } else {
                    let mut rng = thread_rng();
                    let rand_num = rng.gen_range(1..=100);
                    if rand_num <= 70 {
                        // 有70%概率加注10%
                        d_value = max_bet_value/10;
                    } else {
                        let mut rng = thread_rng();
                        let rand_num = rng.gen_range(2..=5);
                        d_value = max_bet_value/10*rand_num;
                    }
                }
                if d_value%min_value_unit != 0 {
                    d_value = d_value + (min_value_unit - d_value%min_value_unit);
                }
                if value + d_value > max_bet_value {
                    continue;
                }
                let self_clone = self.clone();
                let bet_bank_backup = self_clone.bet_bank.get_basket_vec();
                match self.place_a_bet_with_value(value+d_value) {
                    Ok(T) => {
                        te_stack.push_str(&T);
                        self.role = Role::PlaceBet;
                        return Ok(te_stack);
                    }
                    Err(E) => {
                        self.place_a_bet(&bet_bank_backup);
                        te_stack.push_str(&E);
                    }
                }
            }
            self.role = Role::GiveUp;
            return Err(te_stack);
        }

        /// 下注指定的物品，并检查是否等于value
        pub fn place_a_bet_and_check_value(&mut self, bet:&Vec<(&StuffType,i32)>, order:Ordering ,value:i32) -> Result<String, String> {
            let mut te_stack = String::new();
            // 把bet拿回来
            match self.get_bets_back() {
                Ok(T) => te_stack.push_str(&T),
                Err(E) => te_stack.push_str(&E),
            }
            match self.place_a_bet(bet) {
                Ok(T) => {
                    // 如果可以下注
                    te_stack.push_str(&T);
                    let num = self.bet_bank.get_values_of_bank();
                    match order{
                        Ordering::Equal if num == value => {
                            // 只有符合要求才可以
                            self.role = Role::PlaceBet;
                            return Ok(te_stack);
                        }
                        Ordering::Greater if num >= value => {
                            // 只有符合要求才可以
                            self.role = Role::PlaceBet;
                            return Ok(te_stack);
                        }
                        Ordering::Less if num <= value => {
                            // 只有符合要求才可以
                            self.role = Role::PlaceBet;
                            return Ok(te_stack);
                        }
                        _ => {
                            // 不符合要求就忽略
                            match self.get_bets_back() {
                                Ok(T) => te_stack.push_str(&T),
                                Err(E) => te_stack.push_str(&E),
                            }
                            return Err(te_stack + &format!("Please place a bet meeting the requirement of {0}!",self.role));
                        }
                    }
                },
                Err(E) => {
                    // 如果无法下注
                    te_stack.push_str(&E);
                    return Err(te_stack + &"Please place a bet that you are able to offer!".to_string());
                },
            }
        }
        /// 设置初始资金
        pub fn initial_my_owned_bank(&mut self, initial:&Vec<(&StuffType, i32)>) -> Result<String, String> {
            self.owned_bank.stuff_in(initial,true)
        }
        /// 清除我的卡
        pub fn clear_my_cards(&mut self) -> Result<String, String> {
            self.cards_bank.stuff_clear()
        }
        /// 清除我的下注
        pub fn clear_my_bet_bank(&mut self) -> Result<String, String> {
            self.bet_bank.stuff_clear()
        }
        /// 玩家把下注的钱拿回来
        pub fn get_bets_back(&mut self) -> Result<String, String> {
            let bet_bank_temp = self.bet_bank.clone();
            let bets = bet_bank_temp.get_basket_vec();
            self.bet_bank = MyBank::new();
            self.owned_bank.stuff_in(&bets, true)
        }
        /// 游戏结束或放弃时，输家把钱交到钱池里
        pub fn send_bets_to_pool(&mut self, to:&mut CashPool) -> Result<String, String> {
            let bet_bank_temp = self.bet_bank.clone();
            let bets = bet_bank_temp.get_basket_vec();
            self.bet_bank.stuff_transfer(&bets, true, &mut (to.cash_pool))
        }
        /// 游戏结束或放弃时，输家把银行里的钱交到钱池里
        pub fn send_owned_to_pool(&mut self, to:&mut CashPool) -> Result<String, String> {
            let owned_bank_temp = self.owned_bank.clone();
            let bets = owned_bank_temp.get_basket_vec();
            self.owned_bank.stuff_transfer(&bets, true, &mut (to.cash_pool))
        }
        /// 游戏结束时，赢家从钱池里拿钱
        pub fn get_bets_from_pool(&mut self, from:&mut CashPool) -> Result<String, String> {
            let cash_pool_temp = from.cash_pool.clone();
            let bets = cash_pool_temp.get_basket_vec();
            from.cash_pool.stuff_transfer(&bets, true, &mut (self.owned_bank))
        }
        /// 抽取两张卡
        pub fn get_two_cards(&mut self, from:&mut CardPool) -> Result<String, String> {
            let card_pool_temp = from.card_pool.clone();
            let mut e_stack = String::new();
            loop {
                let mut two_cards = Vec::new();
                two_cards.push((card_pool_temp.read_random_item(),1));
                two_cards.push((card_pool_temp.read_random_item(),1));
                match from.card_pool.stuff_transfer(&two_cards, true, &mut (self.cards_bank)) {
                    Ok(T) => return Ok(T + &e_stack),
                    Err(E) => e_stack.push_str(&(E)),
                }
            }
        }
        /// 还卡
        pub fn send_cards_back(&mut self, to:&mut CardPool) -> Result<String, String> {
            let cards_bank_temp = self.cards_bank.clone();
            let cards = cards_bank_temp.get_basket_vec();
            self.cards_bank.stuff_transfer(&cards, true, &mut (to.card_pool))
        }
        /// 求最佳组合
        pub fn get_cards_max_value_and_category(&self, five_cards:&FiveCards) -> (i32, FiveCardsCategory) {
            let mut cards_vec = self.cards_bank.get_basket_vec();
            let mut five_cards_vec = five_cards.five_cards.get_basket_vec();
            cards_vec.append(&mut five_cards_vec);
            let mut max_value = 0;
            let mut max_category = FiveCardsCategory::HighCard;
            for i in 0..=5 {
                let mut j = i + 1;
                loop {
                    let mut five_cards_collect:Vec<&Card> = Vec::new();
                    for k in 0..=6 {
                        if k!=i && k!=j {
                            if let StuffType::CardType(card) = cards_vec[k].0 {
                                five_cards_collect.push(card);
                            }
                        }
                    }
                    let value = get_five_cards_value(&five_cards_collect);
                    let category = get_five_cards_category(&five_cards_collect);
                    if value > max_value {
                        max_value = value;
                        max_category = category;
                    }
                    j += 1;
                    if j >= 7 {
                        break;
                    }
                }
            }
            return (max_value,max_category);
        }
    }

    /// 钱池
    #[derive(Clone)]
    pub struct CashPool {
        pub cash_pool:MyBank,
    }

    /// 构造函数
    impl Player {
        pub fn new(name:&str) -> Player {
            let player = Player {
                name: name.to_string(),
                role: Role::Normal,
                owned_bank: MyBank::new(),
                bet_bank: MyBank::new(),
                cards_bank: MyBank::new(),
            };
            return player;
        }
    }

    /// 构造函数
    impl CashPool {
        pub fn new() -> CashPool {
            let cashes = CashPool {
                cash_pool:MyBank::new(),
            };
            return cashes;
        }
        pub fn clear_cash_pool(&mut self) -> Result<String, String> {
            self.cash_pool.stuff_clear()
        }
    }

    #[derive(Clone)]
    pub enum Role {
        Normal,
        DaMang(i32),
        XiaoMang(i32),
        PlaceBet,
        GiveUp,
        Quit,
    }


    /// 给玩家实现Display特性
    impl Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "我是{0}，",self.name)?;
            write!(f, "现在是{0}角色。", self.role)?;
            write!(f, "我拥有的资产：{0}$ {1}。", self.owned_bank.get_values_of_bank(),self.owned_bank)?;
            write!(f, "我下注的资产：{0}$ {1}。", self.bet_bank.get_values_of_bank(),self.bet_bank)?;
            write!(f, "我的卡组：{0}。", self.cards_bank)
        }
    }
    /// 给角色实现Display特性
    impl Display for Role {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Role::Normal => write!(f, "普通"),
                Role::DaMang(..) => write!(f, "大盲"),
                Role::XiaoMang(..) => write!(f, "小盲"),
                Role::PlaceBet => write!(f, "已下注"),
                Role::GiveUp => write!(f, "暂时放弃"),
                Role::Quit => write!(f, "旁观"),
            }
        }
    }

    /// 给钱池实现Display特性
    impl Display for CashPool {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{0}",self.cash_pool)
        }
    }
}


mod module_card {
    use std::fmt;
    use std::fmt::Display;
    use crate::BankManager;
    use crate::module_bank::MyBank;
    use crate::module_bank::StuffType::*;

    #[derive(Hash)]
    #[derive(Eq, PartialEq)]
    #[derive(Clone)]
    pub struct Card {
        name:String,
        value:i32,
        color:CardColor,
    }

    #[derive(Clone)]
    pub struct CardPool {
        pub card_pool:MyBank,
    }

    #[derive(Clone)]
    pub struct FiveCards {
        pub five_cards:MyBank,
    }

    #[derive(Hash)]
    #[derive(Eq, PartialEq)]
    #[derive(Clone)]
    pub enum CardColor {
        HeiTao,
        HongTao,
        MeiHua,
        FangKuai,
    }

    const CARD_GROUP:[&str;13] = ["2","3","4","5","6","7","8","9","10","J","Q","K","A"];

    impl CardPool {
        pub fn new() -> CardPool {
            let mut pool = CardPool {
                card_pool: MyBank::new()
            };
            for (val,&i) in CARD_GROUP.iter().enumerate() {
                let no_use = pool.card_pool.stuff_in(&CardType(Card{name:i.to_string(),value:val as i32,color:CardColor::HeiTao}), 1);
                let no_use = pool.card_pool.stuff_in(&CardType(Card{name:i.to_string(),value:val as i32,color:CardColor::HongTao}),1);
                let no_use = pool.card_pool.stuff_in(&CardType(Card{name:i.to_string(),value:val as i32,color:CardColor::MeiHua}),1);
                let no_use = pool.card_pool.stuff_in(&CardType(Card{name:i.to_string(),value:val as i32,color:CardColor::FangKuai}),1);
            }
            return pool;
        }
        /// 重置卡组
        pub fn reset_card_pool(&self) -> CardPool {
            return CardPool::new();
        }
    }

    impl FiveCards {
        pub fn new() -> FiveCards {
            let cards = FiveCards {
                five_cards:MyBank::new(),
            };
            return cards;
        }
        pub fn clear_five_cards(&mut self) -> Result<String, String> {
            self.five_cards.stuff_clear()
        }
        /// 抽取五张卡
        pub fn get_five_cards(&mut self, from:&mut CardPool) -> Result<String, String> {
            let mut e_stack = String::new();
            loop {
                let card_pool_temp = from.card_pool.clone();
                let mut five_cards = Vec::new();
                five_cards.push((card_pool_temp.read_random_item(),1));
                five_cards.push((card_pool_temp.read_random_item(),1));
                five_cards.push((card_pool_temp.read_random_item(),1));
                five_cards.push((card_pool_temp.read_random_item(),1));
                five_cards.push((card_pool_temp.read_random_item(),1));
                match from.card_pool.stuff_transfer(&five_cards, true, &mut (self.five_cards)) {
                    Ok(T) => return Ok(e_stack+&T),
                    Err(E) => e_stack.push_str(&(E)),
                }
            }
        }
        /// 还卡
        pub fn send_cards_back(&mut self, to:&mut CardPool) -> Result<String, String> {
            let five_cards_temp = self.five_cards.clone();
            let cards = five_cards_temp.get_basket_vec();
            self.five_cards.stuff_transfer(&cards, true, &mut (to.card_pool))
        }
    }

    pub enum FiveCardsCategory {
        RoyalFlush,
        StraightFlush,
        FourOfAKind,
        FullHouse,
        Flush,
        Straight,
        ThreeOfAKind,
        TwoPairs,
        Pair,
        HighCard,
    }

    pub fn get_five_cards_value(five_cards:&Vec<&Card>) -> i32 {
        let mut value = 0;
        // 先获取基本值
        for card in five_cards.iter() {
            value += card.value;
        }

        match get_five_cards_category(five_cards) {
            FiveCardsCategory::RoyalFlush => value += 900,
            FiveCardsCategory::StraightFlush => value += 800,
            FiveCardsCategory::FourOfAKind => value += 700,
            FiveCardsCategory::FullHouse => value += 600,
            FiveCardsCategory::Flush => value += 500,
            FiveCardsCategory::Straight => value += 400,
            FiveCardsCategory::ThreeOfAKind => value += 300,
            FiveCardsCategory::TwoPairs => value += 200,
            FiveCardsCategory::Pair => value += 100,
            FiveCardsCategory::HighCard => value += 0,
        }
        return value;
    }

    pub fn get_five_cards_category(five_cards:&Vec<&Card>) -> FiveCardsCategory {
        // 先排序，从大到小
        let mut sort_cards = five_cards.clone();
        let fun = |a:&&Card,b:&&Card| {
            let x1 = a.value;
            let x2 = b.value;
            x2.cmp(&x1)
        };
        // 获取按照面值排序的篮子东西
        sort_cards.sort_by(fun);

        // 一对的情况
        let mut result_is_pair:Vec<bool> = Vec::new();
        for i in 0..=3 {
            result_is_pair.push(is_pair(sort_cards[i],sort_cards[i+1]));
        }
        // 一对花色的情况
        let mut result_is_same_color:Vec<bool> = Vec::new();
        for i in 0..=3 {
            result_is_same_color.push(is_same_color(sort_cards[i],sort_cards[i+1]));
        }
        // 相邻的情况
        let mut result_is_near:Vec<bool> = Vec::new();
        for i in 0..=3 {
            result_is_near.push(is_near(sort_cards[i],sort_cards[i+1]));
        }

        // CCC   CCC   CCC   CCC   CCC
        // CCC-0-CCC-1-CCC-2-CCC-3-CCC
        // CCC   CCC   CCC   CCC   CCC

        // 判断皇家同花顺
        let mut is_RoyalFlush:bool = sort_cards[4].value>=10;
        for i in 0..=3 {
            is_RoyalFlush &= result_is_same_color[i] & result_is_near[i];
        }
        if is_RoyalFlush {
            return FiveCardsCategory::RoyalFlush;
        }

        // 判断同花顺
        let mut is_StraightFlush:bool = true;
        for i in 0..=3 {
            is_StraightFlush &= result_is_same_color[i] & result_is_near[i];
        }
        if is_StraightFlush {
            return FiveCardsCategory::StraightFlush;
        }

        // 判断四条
        let mut is_FourOfAKind:bool = false;
        is_FourOfAKind |= result_is_pair[0] && result_is_pair[1] && result_is_pair[2];
        is_FourOfAKind |= result_is_pair[1] && result_is_pair[2] && result_is_pair[3];
        if is_FourOfAKind {
            return FiveCardsCategory::FourOfAKind;
        }

        // 判断三带二
        let mut is_FullHouse:bool = false;
        is_FullHouse |= result_is_pair[0] && result_is_pair[1] && result_is_pair[3];
        is_FullHouse |= result_is_pair[2] && result_is_pair[3] && result_is_pair[0];
        if is_FullHouse {
            return FiveCardsCategory::FullHouse;
        }

        // 判断同花
        let mut is_Flush:bool = true;
        for i in 0..=3 {
            is_Flush &= result_is_same_color[i];
        }
        if is_Flush {
            return FiveCardsCategory::Flush;
        }

        // 判断顺子
        let mut is_Straight:bool = true;
        for i in 0..=3 {
            is_Straight &= result_is_near[i];
        }
        if is_Straight {
            return FiveCardsCategory::Straight;
        }

        // 判断三条
        let mut is_ThreeOfAKind:bool = false;
        is_ThreeOfAKind |= result_is_pair[0] && result_is_pair[1];
        is_ThreeOfAKind |= result_is_pair[1] && result_is_pair[2];
        is_ThreeOfAKind |= result_is_pair[2] && result_is_pair[3];
        if is_ThreeOfAKind {
            return FiveCardsCategory::ThreeOfAKind;
        }

        // 判断两个对子
        let mut is_TwoPairs:bool = false;
        is_TwoPairs |= result_is_pair[0] && result_is_pair[2];
        is_TwoPairs |= result_is_pair[1] && result_is_pair[3];
        is_TwoPairs |= result_is_pair[0] && result_is_pair[3];
        if is_TwoPairs {
            return FiveCardsCategory::TwoPairs;
        }

        // 判断对子
        let mut is_Pair:bool = false;
        for i in 0..=3 {
            is_Pair |= result_is_pair[i];
        }
        if is_Pair {
            return FiveCardsCategory::Pair;
        }

        // 剩余的就是高牌
        return FiveCardsCategory::HighCard;
    }

    // 检查是否是一对
    fn is_pair(card1:&Card, card2:&Card) -> bool {
        return if card1.value == card2.value {
            true
        } else {
            false
        }
    }

    // 检查花色是否一致
    fn is_same_color(card1:&Card, card2:&Card) -> bool {
        return if card1.color == card2.color {
            true
        } else {
            false
        }
    }

    // 检查是否相邻（card1大，card2小）
    fn is_near(card1:&Card, card2:&Card) -> bool {
        return if card1.value == card2.value + 1 {
            true
        } else {
            false
        }
    }


    /// 给卡实现Display特性
    impl Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.color {
                CardColor::HeiTao => write!(f, "黑桃{0}",self.name),
                CardColor::HongTao => write!(f, "红桃{0}",self.name),
                CardColor::MeiHua => write!(f, "梅花{0}",self.name),
                CardColor::FangKuai => write!(f, "方块{0}",self.name)
            }
        }
    }

    /// 给五卡种类实现Display特性
    impl Display for FiveCardsCategory {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                FiveCardsCategory::RoyalFlush => write!(f, "皇家同花顺"),
                FiveCardsCategory::StraightFlush => write!(f, "同花顺"),
                FiveCardsCategory::FourOfAKind => write!(f, "四条"),
                FiveCardsCategory::FullHouse => write!(f, "葫芦"),
                FiveCardsCategory::Flush => write!(f, "同花"),
                FiveCardsCategory::Straight => write!(f, "顺子"),
                FiveCardsCategory::ThreeOfAKind => write!(f, "三条"),
                FiveCardsCategory::TwoPairs => write!(f, "两对"),
                FiveCardsCategory::Pair => write!(f, "对子"),
                FiveCardsCategory::HighCard => write!(f, "高牌"),
            }
        }
    }

    /// 给卡组实现Display特性
    impl Display for CardPool {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{0}",self.card_pool)
        }
    }

    /// 给卡组实现Display特性
    impl Display for FiveCards {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{0}",self.five_cards)
        }
    }

}

mod module_game {
    use std::cmp::{min, Ordering};
    use std::collections::HashMap;
    use crate::CardPool;
    use crate::module_card::{FiveCards, FiveCardsCategory};
    use crate::module_player::{CashPool, Player, Role};
    use crate::module_bank::*;
    use std::fmt;
    use std::fmt::{Display, format};
    use rand::prelude::*;

    #[derive(Clone)]
    pub struct Game {
        pub players: Vec<Player>,
        pub cash_pool: CashPool,
        pub card_pool: CardPool,
        pub five_cards: FiveCards,
        pub game_status: GameStatus,
        pub last_XiaoMang_ID: i32,
        pub min_value_unit: i32,
    }

    pub enum MyEvent<'a> {
        AddBot { num: usize },
        ResetGame { assets: &'a Vec<(&'a StuffType, i32)> },
        StartNextGame,
        PickCards,
        PlaceABet { bet: &'a Vec<(&'a StuffType, i32)> },
        PlaceABet_Auto,
        ConfirmBalance,
        GiveUp,
    }

    #[derive(Clone)]
    pub enum GameStatus {
        Setting,
        CardsPicking,
        BetPlacing1,
        BetPlacing2,
        BetPlacing3,
        BetPlacing4,
        Balancing,
    }

    const MAX_PLAYER_NUM: usize = 10;
    const PLAYER_NAME: [&str; MAX_PLAYER_NUM] = ["ME", "Alice", "Bob", "Cara", "David", "Ederson", "Ford", "Gavin", "Harry", "Ian"];

    impl Game {
        pub fn new() -> Game {
            let mut game = Game {
                players: Vec::new(),
                cash_pool: CashPool::new(),
                card_pool: CardPool::new(),
                five_cards: FiveCards::new(),
                game_status: GameStatus::Setting,
                last_XiaoMang_ID: 0,
                min_value_unit: 1,
            };
            match game.add_a_player("ME") {
                Ok(T) => println!("{}", T),
                Err(E) => println!("{}", E),
            }
            return game;
        }
        pub fn get_min_value_unit(&mut self) {
            let mut value_unit = -1;
            for player in self.players.iter() {
                for item in player.owned_bank.get_basket_vec().iter() {
                    if value_unit < 0 || item.0.get_value() < value_unit {
                        value_unit = item.0.get_value();
                    }
                }
                for item in player.bet_bank.get_basket_vec().iter() {
                    if value_unit < 0 || item.0.get_value() < value_unit {
                        value_unit = item.0.get_value();
                    }
                }
            }
            self.min_value_unit = value_unit;
        }
        /// 根据玩家数量、总金额计算小盲金额
        pub fn get_XiaoMang_value(&self) -> i32 {
            let mut player_num_left = 0;
            for player in self.players.iter() {
                if let Role::PlaceBet|Role::GiveUp|Role::Normal|Role::XiaoMang(..)|Role::DaMang(..) = player.role {
                    player_num_left += 1;
                }
            }
            let mut value_of_all = 0;
            let value_unit = self.min_value_unit;
            for player in self.players.iter() {
                value_of_all += player.owned_bank.get_values_of_bank() + player.bet_bank.get_values_of_bank();
            }
            return if (value_of_all / player_num_left / 30) % value_unit != 0 {
                value_of_all / player_num_left / 30 + (value_unit - (value_of_all / player_num_left / 30) % value_unit)
            } else {
                value_of_all / player_num_left / 30
            }
        }
        /// 根据玩家数量、总金额计算大盲金额
        pub fn get_DaMang_value(&self) -> i32 {
            let mut player_num_left = 0;
            for player in self.players.iter() {
                if let Role::PlaceBet|Role::GiveUp|Role::Normal|Role::XiaoMang(..)|Role::DaMang(..) = player.role {
                    player_num_left += 1;
                }
            }
            let mut value_of_all = 0;
            let value_unit = self.min_value_unit;
            for player in self.players.iter() {
                value_of_all += player.owned_bank.get_values_of_bank() + player.bet_bank.get_values_of_bank();
            }
            return if (value_of_all / player_num_left / 30) % value_unit != 0 {
                (value_of_all / player_num_left / 30 + (value_unit - (value_of_all / player_num_left / 30) % value_unit))*2
            } else {
                (value_of_all / player_num_left / 30)*2
            }
        }
        /// 根据玩家数量、总金额计算最大下注金额
        pub fn get_max_bet_value(&self) -> i32 {
            let mut player_num_left = 0;
            for player in self.players.iter() {
                if let Role::PlaceBet|Role::GiveUp|Role::Normal|Role::XiaoMang(..)|Role::DaMang(..) = player.role {
                    player_num_left += 1;
                }
            }
            let mut value_of_all = 0;
            let value_unit = self.min_value_unit;
            for player in self.players.iter() {
                value_of_all += player.owned_bank.get_values_of_bank() + player.bet_bank.get_values_of_bank();
            }
            return if (value_of_all / player_num_left / 3) % value_unit != 0 {
                value_of_all/player_num_left/3 + (value_unit - (value_of_all/player_num_left/3)%value_unit)
            } else {
                value_of_all/player_num_left/3
            }
        }
        /// 添加玩家
        pub fn add_a_player(&mut self, name: &str) -> Result<String, String> {
            return if self.players.len() < MAX_PLAYER_NUM {
                let mut player = Player::new(name);
                self.players.push(player);
                Ok(format!("Succeed to add a new player called \"{0}\"!", name))
            } else {
                Err(format!("Fail to add a new player because there are too many players!"))
            }
        }
        /// 游戏初始化
        pub fn init_game(&mut self, initial: &Vec<(&StuffType, i32)>) -> Result<String, String> {
            let mut te_stack = String::new();
            match self.five_cards.clear_five_cards() {
                Ok(T) => te_stack.push_str(&(T)),
                Err(E) => te_stack.push_str(&(E)),
            }
            match self.cash_pool.clear_cash_pool() {
                Ok(T) => te_stack.push_str(&(T)),
                Err(E) => te_stack.push_str(&(E)),
            }
            self.card_pool = self.card_pool.reset_card_pool();

            // 随机选择小盲角色
            let length = self.players.len();
            let mut rng = thread_rng();
            let rand_num = rng.gen_range(0..length);

            for (ID, player) in self.players.iter_mut().enumerate() {
                match player.clear_my_bet_bank() {
                    Ok(T) => te_stack.push_str(&(T)),
                    Err(E) => te_stack.push_str(&(E)),
                }
                match player.clear_my_cards() {
                    Ok(T) => te_stack.push_str(&(T)),
                    Err(E) => te_stack.push_str(&(E)),
                }
                match player.initial_my_owned_bank(initial) {
                    Ok(T) => te_stack.push_str(&(T)),
                    Err(E) => te_stack.push_str(&(E)),
                }
            }
            self.get_min_value_unit();
            self.players[rand_num].role = Role::XiaoMang(self.get_XiaoMang_value());
            self.last_XiaoMang_ID = rand_num as i32;
            if length - 1 == rand_num {
                self.players[0].role = Role::DaMang(self.get_DaMang_value());
            } else {
                self.players[rand_num+1].role = Role::DaMang(self.get_DaMang_value());
            }
            Ok(te_stack)
        }

        /// 玩家事件响应
        pub fn receive_my_event(&mut self, event: MyEvent) -> Result<String, String> {
            let game_status = self.game_status.clone();
            match event {
                MyEvent::AddBot { num } => {
                    match game_status {
                        GameStatus::Setting => {
                            self.game_status = GameStatus::Setting;
                            let players_temp = self.players.clone();
                            let mut te_stack = String::new();
                            for i in 0..num {
                                match self.add_a_player(PLAYER_NAME[players_temp.len() + i]) {
                                    Ok(T) => te_stack.push_str(&T),
                                    Err(E) => te_stack.push_str(&E),
                                }
                            }
                            return Ok(te_stack);
                        },
                        GameStatus::CardsPicking => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing1 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing2 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing3 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing4=> {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::Balancing => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                    }
                },
                MyEvent::ResetGame { assets } => {
                    match game_status {
                        GameStatus::Setting => {
                            self.game_status = GameStatus::Setting;
                            return self.init_game(assets);
                        },
                        GameStatus::CardsPicking => {
                            self.game_status = GameStatus::Setting;
                            return self.init_game(assets);
                        },
                        GameStatus::BetPlacing1 => {
                            self.game_status = GameStatus::Setting;
                            return self.init_game(assets);
                        },
                        GameStatus::BetPlacing2 => {
                            self.game_status = GameStatus::Setting;
                            return self.init_game(assets);
                        },
                        GameStatus::BetPlacing3 => {
                            self.game_status = GameStatus::Setting;
                            return self.init_game(assets);
                        },
                        GameStatus::BetPlacing4 => {
                            self.game_status = GameStatus::Setting;
                            return self.init_game(assets);
                        },
                        GameStatus::Balancing => {
                            self.game_status = GameStatus::Setting;
                            return self.init_game(assets);
                        },
                    }
                },
                MyEvent::StartNextGame => {
                    match game_status {
                        GameStatus::Setting => {
                            self.game_status = GameStatus::CardsPicking;
                            return Ok("游戏开始！".to_string());
                        },
                        GameStatus::CardsPicking => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing1 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing2 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing3 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing4 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::Balancing => {
                            self.game_status = GameStatus::CardsPicking;
                            // 剩余玩家重新分配大小盲
                            let player_num = self.players.len();
                            let mut player_num_left = 0;
                            let mut ID:usize = self.last_XiaoMang_ID as usize;
                            for (ID, player) in self.players.iter_mut().enumerate() {
                                if let Role::PlaceBet|Role::GiveUp|Role::Normal|Role::XiaoMang(..)|Role::DaMang(..) = player.role {
                                    player_num_left += 1;
                                }
                            }
                            if player_num_left <= 1 {
                                self.game_status = GameStatus::Balancing;
                                return Ok(format!("游戏已经结束！只有{0}人留在场上！",player_num_left));
                            }
                            loop {
                                if ID == player_num - 1 {
                                    ID = 0;
                                } else {
                                    ID += 1;
                                }
                                if let Role::PlaceBet|Role::GiveUp|Role::Normal|Role::XiaoMang(..)|Role::DaMang(..) = self.players[ID].role {
                                    self.last_XiaoMang_ID = ID as i32;
                                    break;
                                }
                            }
                            let mut player_num_before_xiaomang = 0;
                            for (ID, player) in self.players.iter_mut().enumerate() {
                                if let Role::PlaceBet|Role::GiveUp|Role::Normal|Role::XiaoMang(..)|Role::DaMang(..) = player.role {
                                    if ID == self.last_XiaoMang_ID as usize {
                                        break;
                                    }
                                    player_num_before_xiaomang += 1;
                                }
                            }
                            let mut stack = Vec::new();
                            stack.push(Role::XiaoMang(self.get_XiaoMang_value()));
                            stack.push(Role::DaMang(self.get_DaMang_value()));
                            for i in 0..player_num_left-2 {
                                    stack.push(Role::Normal);
                            }
                            for i in 0..player_num_before_xiaomang {
                                if let Some(T) = stack.pop() {
                                    stack.reverse();
                                    stack.push(T);
                                    stack.reverse();
                                }
                            }
                            stack.reverse();
                            for (ID, player) in self.players.iter_mut().enumerate() {
                                if let Role::PlaceBet|Role::GiveUp|Role::Normal|Role::XiaoMang(..)|Role::DaMang(..) = player.role {
                                    if let Some(T) = stack.pop() {
                                        player.role = T;
                                    }
                                }
                            }
                            return Ok(format!("游戏开始！还有{0}人留在场上！",player_num_left));
                        },
                    }
                },
                MyEvent::PickCards => {
                    match game_status {
                        GameStatus::Setting => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::CardsPicking => {
                            self.game_status = GameStatus::BetPlacing1;
                            let player_num = self.players.len();
                            let mut te_stack = String::new();
                            // 每人抽两张卡
                            for player in self.players.iter_mut() {
                                match player.get_two_cards(&mut self.card_pool) {
                                    Ok(T) => te_stack.push_str(&T),
                                    Err(E) => te_stack.push_str(&E),
                                }
                            }
                            // 抽五张卡
                            match self.five_cards.get_five_cards(&mut self.card_pool) {
                                Ok(T) => te_stack.push_str(&T),
                                Err(E) => te_stack.push_str(&E),
                            }
                            // 从小盲开始出钱
                            let mut game_clone = self.clone();
                            'outer:for (ID, player) in game_clone.players.iter().enumerate() {
                                match self.players[ID].role {
                                    Role::XiaoMang(value) => {
                                        // 检查是不是我
                                        if player.name == "ME" {
                                            return Ok(te_stack + &format!("It's time for you!"));
                                        }
                                        // 下注指定的资金
                                        match self.players[ID].place_a_bet_with_value(value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    Role::DaMang(value) => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    continue 'outer;
                                                }
                                            }
                                        }
                                        // 检查是不是我
                                        if player.name == "ME" {
                                            return Ok(te_stack + &format!("It's time for you!"));
                                        }
                                        // 下注指定的资金
                                        match self.players[ID].place_a_bet_with_value(value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    Role::Normal => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    continue 'outer;
                                                }
                                            }
                                        }
                                        // 检查是不是我
                                        if player.name == "ME" {
                                            return Ok(te_stack + &format!("It's time for you!"));
                                        }
                                        // 获取需要用于比较的value
                                        let mut value = 0;
                                        if let Role::PlaceBet = self.players[last_player_ID].role {
                                            let players_clone = self.players.clone();
                                            value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                        } else {
                                            loop {
                                                if last_player_ID == 0 {
                                                    last_player_ID = player_num - 1;
                                                } else {
                                                    last_player_ID -= 1;
                                                }
                                                if last_player_ID == ID {
                                                    value = 0;
                                                    break;
                                                }
                                                if let Role::PlaceBet = self.players[last_player_ID].role {
                                                    let players_clone = self.players.clone();
                                                    value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                                    break;
                                                }
                                            }
                                        }
                                        // 根据上一者下注的资金来下注
                                        let game_clone = self.clone();
                                        match self.players[ID].place_a_bet_with_last_value(value,game_clone.get_max_bet_value(),game_clone.min_value_unit) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            return Ok(te_stack);
                        },
                        GameStatus::BetPlacing1 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing2 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing3 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing4 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::Balancing => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                    }
                },
                MyEvent::PlaceABet { .. }|MyEvent::PlaceABet_Auto => {
                    match game_status {
                        GameStatus::Setting => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::CardsPicking => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing1|GameStatus::BetPlacing2|GameStatus::BetPlacing3|GameStatus::BetPlacing4 => {
                            match game_status {
                                GameStatus::BetPlacing1 => self.game_status = GameStatus::BetPlacing2,
                                GameStatus::BetPlacing2 => self.game_status = GameStatus::BetPlacing3,
                                GameStatus::BetPlacing3 => self.game_status = GameStatus::BetPlacing4,
                                GameStatus::BetPlacing4 => self.game_status = GameStatus::Balancing,
                                _ => {}
                            }

                            let player_num = self.players.len();
                            let mut te_stack = String::new();
                            let ID = 0;

                            let players_clone = self.players.clone();
                            let bet_bank_backup = players_clone[ID].bet_bank.get_basket_vec();
                            if let MyEvent::PlaceABet {bet} = event {
                                // 我自己下注，有成功和失败的可能
                                match self.players[ID].role {
                                    Role::XiaoMang(value) => {
                                        // 下指定的注
                                        match self.players[ID].place_a_bet_and_check_value(bet,Ordering::Equal,value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => {
                                                self.players[ID].place_a_bet(&bet_bank_backup);
                                                match game_status {
                                                    GameStatus::BetPlacing1 => self.game_status = GameStatus::BetPlacing1,
                                                    GameStatus::BetPlacing2 => self.game_status = GameStatus::BetPlacing2,
                                                    GameStatus::BetPlacing3 => self.game_status = GameStatus::BetPlacing3,
                                                    GameStatus::BetPlacing4 => self.game_status = GameStatus::BetPlacing4,
                                                    _ => {}
                                                }
                                                te_stack.push_str(&E);
                                                return Err(te_stack);
                                            },
                                        }
                                    }
                                    Role::DaMang(value) => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    match game_status {
                                                        GameStatus::BetPlacing1 => self.game_status = GameStatus::BetPlacing1,
                                                        GameStatus::BetPlacing2 => self.game_status = GameStatus::BetPlacing2,
                                                        GameStatus::BetPlacing3 => self.game_status = GameStatus::BetPlacing3,
                                                        GameStatus::BetPlacing4 => self.game_status = GameStatus::BetPlacing4,
                                                        _ => {}
                                                    }
                                                    return Err(te_stack + &"It's not your turn!".to_string());
                                                }
                                            }
                                        }
                                        // 下指定的注
                                        match self.players[ID].place_a_bet_and_check_value(bet,Ordering::Equal,value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => {
                                                self.players[ID].place_a_bet(&bet_bank_backup);
                                                match game_status {
                                                    GameStatus::BetPlacing1 => self.game_status = GameStatus::BetPlacing1,
                                                    GameStatus::BetPlacing2 => self.game_status = GameStatus::BetPlacing2,
                                                    GameStatus::BetPlacing3 => self.game_status = GameStatus::BetPlacing3,
                                                    GameStatus::BetPlacing4 => self.game_status = GameStatus::BetPlacing4,
                                                    _ => {}
                                                }
                                                te_stack.push_str(&E);
                                                return Err(te_stack);
                                            },
                                        }
                                    }
                                    Role::Normal|Role::PlaceBet => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    match game_status {
                                                        GameStatus::BetPlacing1 => self.game_status = GameStatus::BetPlacing1,
                                                        GameStatus::BetPlacing2 => self.game_status = GameStatus::BetPlacing2,
                                                        GameStatus::BetPlacing3 => self.game_status = GameStatus::BetPlacing3,
                                                        GameStatus::BetPlacing4 => self.game_status = GameStatus::BetPlacing4,
                                                        _ => {}
                                                    }
                                                    return Err(te_stack + &"It's not your turn!".to_string());
                                                }
                                            }
                                        }
                                        // 获取需要用于比较的value
                                        let mut value = 0;
                                        if let Role::PlaceBet = self.players[last_player_ID].role {
                                            let players_clone = self.players.clone();
                                            value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                        } else {
                                            loop {
                                                if last_player_ID == 0 {
                                                    last_player_ID = player_num - 1;
                                                } else {
                                                    last_player_ID -= 1;
                                                }
                                                if last_player_ID == ID {
                                                    value = 0;
                                                    break;
                                                }
                                                if let Role::PlaceBet = self.players[last_player_ID].role {
                                                    let players_clone = self.players.clone();
                                                    value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                                    break;
                                                }
                                            }
                                        }
                                        // 下指定的注
                                        match self.players[ID].place_a_bet_and_check_value(bet,Ordering::Greater,value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => {
                                                self.players[ID].place_a_bet(&bet_bank_backup);
                                                match game_status {
                                                    GameStatus::BetPlacing1 => self.game_status = GameStatus::BetPlacing1,
                                                    GameStatus::BetPlacing2 => self.game_status = GameStatus::BetPlacing2,
                                                    GameStatus::BetPlacing3 => self.game_status = GameStatus::BetPlacing3,
                                                    GameStatus::BetPlacing4 => self.game_status = GameStatus::BetPlacing4,
                                                    _ => {}
                                                }
                                                te_stack.push_str(&E);
                                                return Err(te_stack);
                                            },
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            // 剩下的玩家继续下注
                            let mut game_clone = self.clone();
                            'outer:for (ID, player) in game_clone.players.iter().enumerate() {
                                if let MyEvent::PlaceABet {..} = event {
                                    if player.name == "ME" {
                                        continue;
                                    }
                                }
                                if let GameStatus::Balancing = self.game_status {
                                    if ID == self.last_XiaoMang_ID as usize {
                                        return Ok(te_stack + &"Game End Normally!".to_string());
                                    }
                                }
                                match self.players[ID].role {
                                    Role::XiaoMang(value) => {
                                        // 下注指定的资金
                                        match self.players[ID].place_a_bet_with_value(value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    Role::DaMang(value) => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    continue 'outer;
                                                }
                                            }
                                        }
                                        // 下注指定的资金
                                        match self.players[ID].place_a_bet_with_value(value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    Role::Normal|Role::PlaceBet => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    continue 'outer;
                                                }
                                            }
                                        }
                                        // 获取需要用于比较的value
                                        let mut value = 0;
                                        if let Role::PlaceBet = self.players[last_player_ID].role {
                                            let players_clone = self.players.clone();
                                            value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                        } else {
                                            loop {
                                                if last_player_ID == 0 {
                                                    last_player_ID = player_num - 1;
                                                } else {
                                                    last_player_ID -= 1;
                                                }
                                                if last_player_ID == ID {
                                                    value = 0;
                                                    break;
                                                }
                                                if let Role::PlaceBet = self.players[last_player_ID].role {
                                                    let players_clone = self.players.clone();
                                                    value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                                    break;
                                                }
                                            }
                                        }
                                        // 根据上一者下注的资金来下注
                                        let game_clone = self.clone();
                                        match self.players[ID].place_a_bet_with_last_value(value,game_clone.get_max_bet_value(),game_clone.min_value_unit) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            return Ok(te_stack);
                        },
                        GameStatus::Balancing => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                    }
                },
                MyEvent::ConfirmBalance => {
                    match game_status {
                        GameStatus::Setting => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::CardsPicking => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing1 => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::Balancing | GameStatus::BetPlacing2 | GameStatus::BetPlacing3 | GameStatus::BetPlacing4 => {
                            match game_status {
                                GameStatus::BetPlacing2 | GameStatus::BetPlacing3 | GameStatus::BetPlacing4 => {
                                    let mut game_clone = self.clone();
                                    let mut players_left = 0;
                                    for (ID, player) in game_clone.players.iter().enumerate() {
                                        if let Role::PlaceBet = player.role {
                                            players_left += 1;
                                        }
                                    }
                                    if players_left == 1 {
                                        self.game_status = GameStatus::Balancing;
                                    }
                                }
                                _ => {}
                            }
                            let mut te_stack = String::new();
                            if let GameStatus::Balancing = self.game_status {
                                if self.five_cards.five_cards.get_basket_vec().len() == 0 {
                                    return Err("Balancing has been done before!".to_string());
                                }
                                // 计算所有还留在场上的玩家的卡牌value
                                // 最大者赢
                                let mut winner_ID = 0;
                                let mut max_value = 0;
                                let mut max_category = FiveCardsCategory::HighCard;

                                let mut game_clone = self.clone();
                                for (ID, player) in game_clone.players.iter().enumerate() {
                                    if let Role::PlaceBet = player.role {
                                        let (value, category) = player.get_cards_max_value_and_category(&self.five_cards);
                                        if value > max_value {
                                            max_value = value;
                                            max_category = category;
                                            winner_ID = ID;
                                        }
                                    }
                                }
                                // 将所有玩家的bet输入到cashpool，并还卡
                                let mut game_clone = self.clone();
                                for (ID, player) in game_clone.players.iter_mut().enumerate() {
                                    if self.players[ID].owned_bank.get_values_of_bank() < self.get_XiaoMang_value() {
                                        match self.players[ID].send_owned_to_pool(&mut self.cash_pool) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    match self.players[ID].send_bets_to_pool(&mut self.cash_pool) {
                                        Ok(T) => te_stack.push_str(&T),
                                        Err(E) => te_stack.push_str(&E),
                                    }
                                    match self.players[ID].send_cards_back(&mut self.card_pool) {
                                        Ok(T) => te_stack.push_str(&T),
                                        Err(E) => te_stack.push_str(&E),
                                    }
                                }

                                // 将cashpool输入到赢家
                                match self.players[winner_ID].get_bets_from_pool(&mut self.cash_pool) {
                                    Ok(T) => te_stack.push_str(&T),
                                    Err(E) => te_stack.push_str(&E),
                                }
                                // 把五卡还回去
                                match self.five_cards.send_cards_back(&mut self.card_pool) {
                                    Ok(T) => te_stack.push_str(&T),
                                    Err(E) => te_stack.push_str(&E),
                                }
                                // owned_bank为空的玩家设置为quit
                                let mut game_clone = self.clone();
                                for (ID, player) in game_clone.players.iter().enumerate() {
                                    if player.owned_bank.get_values_of_bank() == 0 {
                                        self.players[ID].role = Role::Quit;
                                    }
                                }
                                return Ok(format!("{0} is the Winner of this game! He/She gets the cards group of {1}!", self.players[winner_ID].name, max_category) + &te_stack);
                            }
                            return Err(te_stack);
                        },
                    }
                }
                MyEvent::GiveUp => {
                    match game_status {
                        GameStatus::Setting => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::CardsPicking => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                        GameStatus::BetPlacing1|GameStatus::BetPlacing2|GameStatus::BetPlacing3|GameStatus::BetPlacing4 => {
                            match game_status {
                                GameStatus::BetPlacing1 => self.game_status = GameStatus::BetPlacing2,
                                GameStatus::BetPlacing2 => self.game_status = GameStatus::BetPlacing3,
                                GameStatus::BetPlacing3 => self.game_status = GameStatus::BetPlacing4,
                                GameStatus::BetPlacing4 => self.game_status = GameStatus::Balancing,
                                _ => {}
                            }
                            let player_num = self.players.len();
                            let mut te_stack = String::new();
                            let ID = 0;

                            // 我放弃
                            self.players[ID].role = Role::GiveUp;
                            // 剩下的玩家继续下注
                            let mut game_clone = self.clone();
                            'outer:for (ID, player) in game_clone.players.iter().enumerate() {
                                if let MyEvent::PlaceABet {..} = event {
                                    if player.name == "ME" {
                                        continue;
                                    }
                                }
                                if let GameStatus::Balancing = self.game_status {
                                    if ID == self.last_XiaoMang_ID as usize {
                                        return Ok(te_stack + &"Game End Normally!".to_string());
                                    }
                                }
                                match self.players[ID].role {
                                    Role::XiaoMang(value) => {
                                        // 下注指定的资金
                                        match self.players[ID].place_a_bet_with_value(value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    Role::DaMang(value) => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    continue 'outer;
                                                }
                                            }
                                        }
                                        // 下注指定的资金
                                        match self.players[ID].place_a_bet_with_value(value) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    Role::Normal|Role::PlaceBet => {
                                        // 检查是否其他人都放弃或者退出了
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if last_player_ID == ID {
                                                self.game_status = GameStatus::Balancing;
                                                return Ok(te_stack + &format!("{0} win the game because nobody place a bet!", self.players[ID].name));
                                            }
                                            if let Role::PlaceBet|Role::DaMang(..)|Role::XiaoMang(..)|Role::Normal = self.players[last_player_ID].role {
                                                break;
                                            }
                                        }
                                        // 检查前面第一个不是旁观的角色是不是下注或放弃，如果是就可以下注，否则不能下注
                                        let mut last_player_ID = ID;
                                        loop {
                                            if last_player_ID == 0 {
                                                last_player_ID = player_num - 1;
                                            } else {
                                                last_player_ID -= 1;
                                            }
                                            if let Role::PlaceBet|Role::Normal|Role::GiveUp|Role::XiaoMang(..)|Role::DaMang(..) = self.players[last_player_ID].role {
                                                if let Role::PlaceBet|Role::GiveUp = self.players[last_player_ID].role {
                                                    break;
                                                } else {
                                                    continue 'outer;
                                                }
                                            }
                                        }
                                        // 获取需要用于比较的value
                                        let mut value = 0;
                                        if let Role::PlaceBet = self.players[last_player_ID].role {
                                            let players_clone = self.players.clone();
                                            value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                        } else {
                                            loop {
                                                if last_player_ID == 0 {
                                                    last_player_ID = player_num - 1;
                                                } else {
                                                    last_player_ID -= 1;
                                                }
                                                if last_player_ID == ID {
                                                    value = 0;
                                                    break;
                                                }
                                                if let Role::PlaceBet = self.players[last_player_ID].role {
                                                    let players_clone = self.players.clone();
                                                    value = players_clone[last_player_ID].bet_bank.get_values_of_bank();
                                                    break;
                                                }
                                            }
                                        }
                                        // 根据上一者下注的资金来下注
                                        let game_clone = self.clone();
                                        match self.players[ID].place_a_bet_with_last_value(value,game_clone.get_max_bet_value(),game_clone.min_value_unit) {
                                            Ok(T) => te_stack.push_str(&T),
                                            Err(E) => te_stack.push_str(&E),
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            return Ok(te_stack);
                        },
                        GameStatus::Balancing => {
                            return Err(format!("Cannot do {0} while status of game is {1}!", event, game_status));
                        },
                    }
                },
            }
        }
    }

    /// 给Game实现Display特性
    impl Display for Game {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "游戏状态：{0}\n", self.game_status)?;
            write!(f, "小盲下注：{0}\n", self.get_XiaoMang_value())?;
            write!(f, "大盲下注：{0}\n", self.get_DaMang_value())?;
            write!(f, "最大下注：{0}\n", self.get_max_bet_value())?;
            write!(f, "玩家情况：\n")?;
            for player in self.players.iter() {
                write!(f, "{0}\n", player)?;
            }
            write!(f, "钱池情况：{0}\n", self.cash_pool)?;
            write!(f, "卡组情况：{0}\n", self.card_pool)?;
            write!(f, "五卡情况：{0}\n", self.five_cards)
        }
    }

    /// 给状态机实现Display特性
    impl Display for GameStatus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                GameStatus::Setting => write!(f, "等待玩家设置/等待玩家开始游戏"),
                GameStatus::CardsPicking => write!(f, "等待玩家抽卡"),
                GameStatus::BetPlacing1 => write!(f, "等待玩家第一轮下注"),
                GameStatus::BetPlacing2 => write!(f, "等待玩家第二轮下注"),
                GameStatus::BetPlacing3 => write!(f, "等待玩家第三轮下注"),
                GameStatus::BetPlacing4 => write!(f, "等待玩家第四轮下注"),
                GameStatus::Balancing => write!(f, "等待玩家结算/已结算，等待玩家开始游戏")
            }
        }
    }

    /// 给事件实现Display特性
    impl Display for MyEvent<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MyEvent::AddBot { num, .. } => write!(f, "【添加{0}个机器人】", num),
                MyEvent::ResetGame { .. } => write!(f, "【重新初始化游戏】"),
                MyEvent::StartNextGame => write!(f, "【开始下一轮游戏】"),
                MyEvent::PlaceABet { .. } => write!(f, "【下注】"),
                MyEvent::PlaceABet_Auto => write!(f, "【自动下注】"),
                MyEvent::PickCards => write!(f, "【抽卡】"),
                MyEvent::ConfirmBalance => write!(f, "【确定结算】"),
                MyEvent::GiveUp => write!(f, "【弃卡】"),
            }
        }
    }
}


// #[test]
fn test_card() {
    use module_card::*;

    for i in 0..100 {
        let mut cards = CardPool::new();
        let mut game = Game::new();

        println!("{0}\n", cards);
        match game.five_cards.get_five_cards(&mut cards) {
            Ok(T) => println!("{}", T),
            Err(E) => println!("{}", E),
        }
        println!("{0}\n", game);
    }
}

#[test]
fn test_game() {
    use module_game::*;
    use StuffType::GeneralType;
    let mut game = Game::new();

    match game.receive_my_event(MyEvent::AddBot {num:9}) {
        Ok(T) => println!("{}",T),
        Err(E) => println!("{}",E),
    }
    println!("{0}\n", game);

    let stuff:Vec<(&StuffType,i32)> = vec![(&GeneralType("洗发水",50),2),
                                           (&GeneralType("牛肉干",30),4),
                                           (&GeneralType("书本",20),6),
                                           (&GeneralType("水杯",10),8),
                                           (&GeneralType("直尺",5),10)];

    match game.receive_my_event(MyEvent::ResetGame {assets:&stuff}) {
        Ok(T) => println!("{}",T),
        Err(E) => println!("{}",E),
    }
    println!("{0}\n", game);

    for k in 0..10 {
        match game.receive_my_event(MyEvent::StartNextGame) {
            Ok(T) => println!("{}",T),
            Err(E) => println!("{}",E),
        }
        println!("{0}\n", game);

        match game.receive_my_event(MyEvent::PickCards) {
            Ok(T) => println!("{}",T),
            Err(E) => println!("{}",E),
        }
        println!("{0}\n", game);

        for i in 0..4 {
            match game.receive_my_event(MyEvent::PlaceABet_Auto) {
                Ok(T) => println!("{}",T),
                Err(E) => println!("{}",E),
            }
            println!("{0}\n", game);
        }

        match game.receive_my_event(MyEvent::ConfirmBalance) {
            Ok(T) => println!("{}",T),
            Err(E) => println!("{}",E),
        }
        println!("{0}\n", game);
    }
}

