///interface org/slf4j/Logger
pub enum Logger {}
unsafe impl ::java_oxide::ReferenceType for Logger {}
unsafe impl ::java_oxide::JniType for Logger {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"org/slf4j/Logger")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for Logger {}
impl Logger {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"org/slf4j/Logger"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///getName
    pub fn getName<'env>(
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
                        c"getName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isTraceEnabled
    pub fn isTraceEnabled<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"isTraceEnabled",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///trace
    pub fn trace_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"trace",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///trace
    pub fn trace_String_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"trace",
                        c"(Ljava/lang/String;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///trace
    pub fn trace_String_Object_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"trace",
                        c"(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///trace
    pub fn trace_String_Object_array<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<
            ::java_oxide::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>,
        >,
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
                        c"trace",
                        c"(Ljava/lang/String;[Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///trace
    pub fn trace_String_Throwable<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
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
                        c"trace",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isDebugEnabled
    pub fn isDebugEnabled<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"isDebugEnabled",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///debug
    pub fn debug_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"debug",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///debug
    pub fn debug_String_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"debug",
                        c"(Ljava/lang/String;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///debug
    pub fn debug_String_Object_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"debug",
                        c"(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///debug
    pub fn debug_String_Object_array<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<
            ::java_oxide::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>,
        >,
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
                        c"debug",
                        c"(Ljava/lang/String;[Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///debug
    pub fn debug_String_Throwable<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
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
                        c"debug",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isInfoEnabled
    pub fn isInfoEnabled<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"isInfoEnabled",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///info
    pub fn info_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"info",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///info
    pub fn info_String_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"info",
                        c"(Ljava/lang/String;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///info
    pub fn info_String_Object_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"info",
                        c"(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///info
    pub fn info_String_Object_array<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<
            ::java_oxide::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>,
        >,
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
                        c"info",
                        c"(Ljava/lang/String;[Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///info
    pub fn info_String_Throwable<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
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
                        c"info",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isWarnEnabled
    pub fn isWarnEnabled<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"isWarnEnabled",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///warn
    pub fn warn_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"warn",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///warn
    pub fn warn_String_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"warn",
                        c"(Ljava/lang/String;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///warn
    pub fn warn_String_Object_array<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<
            ::java_oxide::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>,
        >,
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
                        c"warn",
                        c"(Ljava/lang/String;[Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///warn
    pub fn warn_String_Object_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"warn",
                        c"(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///warn
    pub fn warn_String_Throwable<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
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
                        c"warn",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isErrorEnabled
    pub fn isErrorEnabled<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"isErrorEnabled",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///error
    pub fn error_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"error",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///error
    pub fn error_String_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
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
                        c"error",
                        c"(Ljava/lang/String;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///error
    pub fn error_String_Object_Object<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"error",
                        c"(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///error
    pub fn error_String_Object_array<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<
            ::java_oxide::ObjectArray<crate::java::lang::Object, crate::java::lang::Throwable>,
        >,
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
                        c"error",
                        c"(Ljava/lang/String;[Ljava/lang/Object;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///error
    pub fn error_String_Throwable<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
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
                        c"error",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///public static final ROOT_LOGGER_NAME
    pub const ROOT_LOGGER_NAME: &'static str = "ROOT";
}
///final class org/slf4j/LoggerFactory
pub enum LoggerFactory {}
unsafe impl ::java_oxide::ReferenceType for LoggerFactory {}
unsafe impl ::java_oxide::JniType for LoggerFactory {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"org/slf4j/LoggerFactory")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for LoggerFactory {}
impl LoggerFactory {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"org/slf4j/LoggerFactory"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///getLogger
    pub fn getLogger_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Logger>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"getLogger",
                        c"(Ljava/lang/String;)Lorg/slf4j/Logger;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///getLogger
    pub fn getLogger_Class<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::Class>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Logger>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"getLogger",
                        c"(Ljava/lang/Class;)Lorg/slf4j/Logger;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///public static final PROVIDER_PROPERTY_KEY
    pub const PROVIDER_PROPERTY_KEY: &'static str = "slf4j.provider";
}
