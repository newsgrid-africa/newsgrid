use crate::env::CanisterEnvironment;
use crate::service::DaoService;
use crate::types::DaoStableStorage;
use crate::SERVICE;
use ic_cdk_macros::init;

#[init]
fn init(init_state: DaoStableStorage) {
    ic_cdk::setup();

    let mut init_service = DaoService::from(init_state);
    init_service.env = Box::new(CanisterEnvironment {});

    SERVICE.with(|service| *service.borrow_mut() = init_service);
}
