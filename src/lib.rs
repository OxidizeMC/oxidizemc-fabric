#![feature(arbitrary_self_types)]

pub extern crate jsys;

use java_oxide::{Env, Local, Global};
use jsys::*;
use jsys::java::lang::String as JString;

pub struct LoggerFactory {}
impl LoggerFactory {
    pub fn get_logger<'env>(env: Env<'env>, name: &str) -> Logger {
        let logger: Local<'env, org::slf4j::Logger> = org::slf4j::LoggerFactory::getLogger_String(
            env,
            JString::from_env_str(env, name),
        )
        .expect("Java exception was thrown when constructing `logger`")
        .expect("Constructed `logger` was null");
        Logger { inner: logger.as_global() }
    }
}

pub struct Logger {
    inner: Global<org::slf4j::Logger>,
}
impl Logger {
    pub fn get_name(&self) -> String {
        self.inner.vm().with_env(|env| {
            let logger: Local<org::slf4j::Logger> = self.inner.as_local(env);
            logger
                .getName()
                .expect("Exception was thrown when getting logger name")
                .expect("Logger name was unexpectedly null")
                .to_string_lossy()
        })
    }

    pub fn info(&self, text: String) {
        self.inner.vm().with_env(|env| {
            let logger: Local<org::slf4j::Logger> = self.inner.as_local(env);
            logger.info_String(JString::from_env_str(env, text)).expect("Exception was thrown when logging");
        })
    }
}
