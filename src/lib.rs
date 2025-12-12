#![feature(arbitrary_self_types)]

pub mod sys {
    pub use oxidizemc_fabric_bindings::*;
    pub type __Env<'a> = java_oxide::Env<'a>;
}
#[cfg(feature = "build")]
pub mod build;

use self::{
    sys::{java::lang::String as JString, *}
};
use java_oxide::{Env, Global, Local};
use oxidizemc::JVM;

pub struct LoggerFactory {}
impl LoggerFactory {
    pub fn get_logger(name: &str) -> JLogger {
        JVM.with_env(|env: Env| {
            let logger: Local<org::slf4j::Logger> =
                org::slf4j::LoggerFactory::getLogger_String(env, JString::from_env_str(env, name))
                    .expect("Exception was thrown")
                    .expect("Object was null");
            JLogger {
                inner: logger.as_global(),
            }
        })
    }
}

pub struct JLogger {
    inner: Global<org::slf4j::Logger>,
}
impl JLogger {
    pub fn get_name(&self) -> String {
        JVM.with_env(|env| {
            let logger: Local<org::slf4j::Logger> = self.inner.as_local(env);
            logger
                .getName()
                .expect("Exception was thrown")
                .expect("Object was null")
                .to_string_lossy()
        })
    }

    pub fn info(&self, text: String) {
        JVM.with_env(|env| {
            let logger: Local<org::slf4j::Logger> = self.inner.as_local(env);
            logger
                .info_String(JString::from_env_str(env, text))
                .expect("Exception was thrown");
        })
    }
}
