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

pub use self::{
    add_action::TrafficActionNewRequest,
    add_chain::TrafficChainNewRequest,
    add_filter::TrafficFilterNewRequest,
    add_qdisc::QDiscNewRequest,
    del_action::TrafficActionDelRequest,
    del_chain::TrafficChainDelRequest,
    del_filter::TrafficFilterDelRequest,
    del_qdisc::QDiscDelRequest,
    get::{
        QDiscGetRequest, TrafficActionGetRequest, TrafficActionKind,
        TrafficChainGetRequest, TrafficClassGetRequest,
        TrafficFilterGetRequest,
    },
    handle::{
        QDiscHandle, TrafficActionHandle, TrafficChainHandle,
        TrafficClassHandle, TrafficFilterHandle,
    },
};
