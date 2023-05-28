use crate::chapter10::payoff_constructible::PayoffHelper;
use crate::chapter10::payoff_factory::PayoffFactory;
use crate::chapter4::payoff3::{PayoffCall, PayoffPut};
use once_cell::sync::OnceCell;

static REGISTER_CALL: OnceCell<PayoffHelper<PayoffCall>> = OnceCell::new();

static REGISTER_PUT: OnceCell<PayoffHelper<PayoffPut>> = OnceCell::new();

impl PayoffFactory {
    pub fn register() {
        REGISTER_CALL.get_or_init(|| PayoffHelper::new("call".to_string()));
        REGISTER_PUT.get_or_init(|| PayoffHelper::new("put".to_string()));
    }
}
