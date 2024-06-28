use ic_cdk::export::candid::CandidType;
use serde::{Deserialize, Serialize};
use ic_cdk_macros::query;

#[derive(CandidType, Serialize, Deserialize)]
struct Timer {
    minutes: u32,
}

#[query]
fn get_timer() -> Timer {
    Timer { minutes: 25 }
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
