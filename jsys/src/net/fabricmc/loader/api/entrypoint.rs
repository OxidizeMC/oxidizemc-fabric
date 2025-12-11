///interface [EntrypointContainer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/entrypoint/EntrypointContainer.html)
pub enum EntrypointContainer {}
unsafe impl ::java_oxide::ReferenceType for EntrypointContainer {}
unsafe impl ::java_oxide::JniType for EntrypointContainer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/entrypoint/EntrypointContainer")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for EntrypointContainer {}
impl EntrypointContainer {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/entrypoint/EntrypointContainer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getEntrypoint](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/entrypoint/EntrypointContainer.html#getEntrypoint())
    pub fn getEntrypoint<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getEntrypoint",
                        c"()Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getProvider](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/entrypoint/EntrypointContainer.html#getProvider())
    pub fn getProvider<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, crate::net::fabricmc::loader::api::ModContainer>,
        >,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getProvider",
                        c"()Lnet/fabricmc/loader/api/ModContainer;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getDefinition](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/entrypoint/EntrypointContainer.html#getDefinition())
    pub fn getDefinition<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::String>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getDefinition",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [PreLaunchEntrypoint](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint.html)
pub enum PreLaunchEntrypoint {}
unsafe impl ::java_oxide::ReferenceType for PreLaunchEntrypoint {}
unsafe impl ::java_oxide::JniType for PreLaunchEntrypoint {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for PreLaunchEntrypoint {}
impl PreLaunchEntrypoint {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onPreLaunch](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint.html#onPreLaunch())
    pub fn onPreLaunch<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"onPreLaunch",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
