#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate hyper;
extern crate xml;
extern crate encoding;
extern crate codegen;

#[macro_use]
extern crate error_chain;

mod wsdl;
pub mod autogen;

pub use wsdl::schema::{
    Documented,
    NamedItem,
    Wsdl,
    WsdlBinding,
    WsdlOperationBinding,
    WsdlInputBinding,
    WsdlOutputBinding,
    WsdlFaultBinding,
    WsdlPort,
    WsdlService
};
