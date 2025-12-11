#![feature(arbitrary_self_types)]

pub mod sys {
    pub use oxidizemc_fabric_bindings::*;
    pub type __Env<'a> = java_oxide::Env<'a>;
}
#[cfg(feature = "derive")]
pub mod derive {
    pub use oxidizemc_fabric_derive::*;
}
#[cfg(feature = "build")]
pub mod build;
pub(crate) mod util;

use self::{
    sys::{java::lang::String as JString, *},
    util::VM,
};
use java_oxide::{Env, Global, Local};

#[doc(hidden)]
pub(crate) static __VM: VM = VM::new();

#[doc(hidden)]
pub fn __java_entrypoint<F: FnOnce()>(env: Env, user_main: F) {
    __VM.init(env.vm());
    user_main()
}

pub struct LoggerFactory {}
impl LoggerFactory {
    pub fn get_logger(name: &str) -> JLogger {
        __VM.get().with_env(|env: Env| {
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
        self.inner.vm().with_env(|env| {
            let logger: Local<org::slf4j::Logger> = self.inner.as_local(env);
            logger
                .getName()
                .expect("Exception was thrown")
                .expect("Object was null")
                .to_string_lossy()
        })
    }

    pub fn info(&self, text: String) {
        self.inner.vm().with_env(|env| {
            let logger: Local<org::slf4j::Logger> = self.inner.as_local(env);
            logger
                .info_String(JString::from_env_str(env, text))
                .expect("Exception was thrown");
        })
    }
}
