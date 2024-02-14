use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

//tokens represent the amount of power each user has over the DAO
#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct TruthTokens {
    pub amount: u64,
}

impl TruthTokens {
    pub fn new(amount: u64) -> Self {
        return TruthTokens { amount };
    }
}

impl Add for TruthTokens {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        TruthTokens {
            amount: self.amount + other.amount,
        }
    }
}

impl AddAssign for TruthTokens {
    fn add_assign(&mut self, rhs: Self) {
        self.amount += rhs.amount
    }
}

impl Sub for TruthTokens {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        TruthTokens {
            amount: self.amount - other.amount,
        }
    }
}

impl SubAssign for TruthTokens {
    fn sub_assign(&mut self, rhs: Self) {
        self.amount -= rhs.amount;
    }
}

impl Mul<u64> for TruthTokens {
    type Output = TruthTokens;
    fn mul(self, rhs: u64) -> Self {
        TruthTokens {
            amount: self.amount * rhs,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Article {
    pub id: u64,
    pub timestamp: u64,
    pub author: Principal,
    pub content: ArticleContent,
    pub upvotes: TruthTokens,
    pub downvotes: TruthTokens,
    pub voters: Vec<Principal>,
    pub ranking: i32,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ArticleContent {
    pub canister_id: Principal,
    pub title: String,
    // timestamp in article already does this
    // pub date: String,
    pub text: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub tokens: TruthTokens,
    pub profile: UserProfile,
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct UserProfile {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub username: String,
}

//stuff to persist

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct DaoStableStorage {
    pub accounts: Vec<Account>,
    pub articles: Vec<Article>,
    pub secret_key: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ArticleSearchResult {
    Article(Article),
    Invalid(String),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Vote {
    Up,
    Down,
}
