mod env;
mod init;
mod service;
mod types;

use crate::service::DaoService;
use crate::types::*;
use ic_cdk;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    static SERVICE: RefCell<DaoService> = RefCell::default();
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_users() -> Vec<Account> {
    SERVICE.with(|service| service.borrow().list_users())
}

#[ic_cdk::update]
#[ic_cdk::export::candid::candid_method]
fn add_user(
    principal: Principal,
    profile: UserProfile,
    secret_key: String,
) -> Result<Principal, String> {
    SERVICE.with(|service| {
        service
            .borrow_mut()
            .add_user(principal, profile, secret_key)
    })
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_articles() -> Vec<Article> {
    SERVICE.with(|service| service.borrow().get_all_articles())
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_articles_sorted() -> Vec<Article> {
    SERVICE.with(|service| service.borrow().get_all_articles_sorted())
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_article(article: u64) -> ArticleSearchResult {
    SERVICE.with(|service| service.borrow().get_article(article))
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn check_truthtoken_balance() -> TruthTokens {
    SERVICE.with(|service| service.borrow().token_balance())
}

#[ic_cdk::update]
#[ic_cdk::export::candid::candid_method]
fn create_article(content: ArticleContent) -> Result<Article, String> {
    SERVICE.with(|service| service.borrow_mut().add_article(content))
}

#[ic_cdk::update]
#[ic_cdk::export::candid::candid_method]
fn vote(article_id: u64, args: Vote) -> Result<String, String> {
    SERVICE.with(|service| service.borrow_mut().vote(args, article_id))
}

ic_cdk::export::candid::export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::export_candid;
    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;
        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap();
        write(
            dir.join("./newsgrid_backend/newsgrid_backend.did"),
            export_candid(),
        )
        .expect("Write failed.");
    }
}
