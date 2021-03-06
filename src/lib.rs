#![feature(slice_concat_ext)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

pub mod outgoing;
pub mod incoming;

extern crate hyper;
extern crate serde;
extern crate serde_json;
