// SPDX-License-Identifier: MIT

//! Traffic control manipulation utilities.
//! See [`tc`].
//!
//! [`tc`]: https://man7.org/linux/man-pages/man8/tc.8.html

mod add_action;
mod add_chain;
mod add_filter;
mod add_qdisc;
mod del_action;
mod del_chain;
mod del_filter;
mod del_qdisc;
mod get;
mod handle;

#[cfg(test)]
mod test;

pub use self::add_action::TrafficActionNewRequest;
pub use self::add_chain::TrafficChainNewRequest;
pub use self::add_filter::TrafficFilterNewRequest;
pub use self::add_qdisc::QDiscNewRequest;
pub use self::del_action::TrafficActionDelRequest;
pub use self::del_chain::TrafficChainDelRequest;
pub use self::del_filter::TrafficFilterDelRequest;
pub use self::del_qdisc::QDiscDelRequest;
pub use self::get::{
    QDiscGetRequest, TrafficActionGetRequest, TrafficActionKind,
    TrafficChainGetRequest, TrafficClassGetRequest, TrafficFilterGetRequest,
};
pub use self::handle::{
    QDiscHandle, TrafficActionHandle, TrafficChainHandle, TrafficClassHandle,
    TrafficFilterHandle,
};
pub use self::{
    add_filter::TrafficFilterNewRequest,
    add_qdisc::QDiscNewRequest,
    del_filter::TrafficFilterDelRequest,
    del_qdisc::QDiscDelRequest,
    get::{
        QDiscGetRequest, TrafficChainGetRequest, TrafficClassGetRequest,
        TrafficFilterGetRequest,
    },
    handle::{
        QDiscHandle, TrafficChainHandle, TrafficClassHandle,
        TrafficFilterHandle,
    },
};
