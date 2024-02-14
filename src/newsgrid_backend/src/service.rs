use crate::env::{EmptyEnvironment, Environment};
use crate::types::*;
use ic_cdk::export::Principal;
use std::collections::HashMap;

pub struct DaoService {
    pub env: Box<dyn Environment>,
    pub accounts: HashMap<Principal, (TruthTokens, UserProfile)>,
    pub articles: HashMap<u64, Article>,
    pub next_article_id: u64,
    pub secret_key: String,
}

impl Default for DaoService {
    fn default() -> Self {
        DaoService {
            env: Box::new(EmptyEnvironment {}),
            accounts: HashMap::new(),
            articles: HashMap::new(),
            secret_key: String::from(""),
            next_article_id: 0,
        }
    }
}

impl From<DaoStableStorage> for DaoService {
    fn from(storage: DaoStableStorage) -> DaoService {
        let accounts = storage
            .accounts
            .clone()
            .into_iter()
            .map(|a| (a.owner, (a.tokens, a.profile)))
            .collect();

        let articles = storage
            .articles
            .clone()
            .into_iter()
            .map(|p| (p.id, p))
            .collect();

        DaoService {
            env: Box::new(EmptyEnvironment {}),
            accounts,
            articles,
            next_article_id: 0,
            secret_key: storage.secret_key,
        }
    }
}

impl DaoService {
    pub fn list_users(&self) -> Vec<Account> {
        self.accounts
            .clone()
            .into_iter()
            .map(|(owner, (tokens, profile))| Account {
                owner,
                tokens,
                profile,
            })
            .collect()
    }

    pub fn add_user(
        &mut self,
        principal: Principal,
        profile: UserProfile,
        secret_key: String,
    ) -> Result<Principal, String> {
        if secret_key == self.secret_key {
            self.accounts
                .insert(principal, (TruthTokens { amount: 100 }, profile));
            return Ok(principal);
        } else {
            return Err(String::from("unauthorized"));
        }
    }

    pub fn get_all_articles(&self) -> Vec<Article> {
        let articles = self.articles.values().cloned().collect();
        return articles;
    }

    pub fn get_all_articles_sorted(&self) -> Vec<Article> {
        let mut articles = self.get_all_articles();
        articles.sort_by(|a, b| a.ranking.cmp(&b.ranking));
        return articles;
    }

    pub fn get_article(&self, article_id: u64) -> ArticleSearchResult {
        match self.articles.get(&article_id).cloned() {
            Some(article) => ArticleSearchResult::Article(article),
            None => ArticleSearchResult::Invalid(String::from("article not found")),
        }
    }

    pub fn token_balance(&self) -> TruthTokens {
        let caller = self.env.caller();
        let (tokens, profile) = self
            .accounts
            .get(&caller)
            .cloned()
            .unwrap_or_else(|| Default::default());

        return tokens;
    }

    pub fn add_article(&mut self, article_content: ArticleContent) -> Result<Article, String> {
        let article_id = self.next_article_id;
        self.next_article_id += 1;

        let new_article = Article {
            id: article_id,
            timestamp: self.env.now(),
            author: self.env.caller(),
            content: article_content,
            upvotes: Default::default(),
            downvotes: Default::default(),
            voters: vec![],
            ranking: Default::default(),
        };

        self.articles.insert(article_id, new_article.clone());
        Ok(new_article)
    }

    pub fn vote(&mut self, vote: Vote, article_id: u64) -> Result<String, String> {
        let caller = self.env.caller();
        match self.articles.get_mut(&article_id) {
            Some(article) => {
                article.voters.push(caller);
                match vote {
                    Vote::Up => {
                        article.upvotes += TruthTokens::new(100);
                        article.ranking += 1;
                        Ok(String::from("upvote added"))
                    }

                    Vote::Down => {
                        article.downvotes += TruthTokens::new(100);
                        article.ranking -= 1;
                        Ok(String::from("downvote added"))
                    }
                }
            }
            None => Err(String::from("article not found")),
        }
    }
}
