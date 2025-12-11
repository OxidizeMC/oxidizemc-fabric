///interface java/util/function/BiConsumer
pub enum BiConsumer {}
unsafe impl ::java_oxide::ReferenceType for BiConsumer {}
unsafe impl ::java_oxide::JniType for BiConsumer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"java/util/function/BiConsumer")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for BiConsumer {}
impl BiConsumer {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"java/util/function/BiConsumer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///accept
    pub fn accept<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::Object>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"accept",
                        c"(Ljava/lang/Object;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///andThen
    pub fn andThen<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<BiConsumer>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, BiConsumer>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"andThen",
                        c"(Ljava/util/function/BiConsumer;)Ljava/util/function/BiConsumer;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface java/util/function/Consumer
pub enum Consumer {}
unsafe impl ::java_oxide::ReferenceType for Consumer {}
unsafe impl ::java_oxide::JniType for Consumer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"java/util/function/Consumer")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for Consumer {}
impl Consumer {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"java/util/function/Consumer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///accept
    pub fn accept<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"accept",
                        c"(Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///andThen
    pub fn andThen<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Consumer>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Consumer>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"andThen",
                        c"(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
