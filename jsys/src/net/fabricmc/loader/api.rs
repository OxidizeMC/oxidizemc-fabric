pub mod entrypoint;
pub mod metadata;
///class [EntrypointException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/EntrypointException.html)
pub enum EntrypointException {}
unsafe impl ::java_oxide::ReferenceType for EntrypointException {}
unsafe impl ::java_oxide::JniType for EntrypointException {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/EntrypointException")
    }
}
impl EntrypointException {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/EntrypointException"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[EntrypointException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/EntrypointException.html#EntrypointException(java.lang.String.java.lang.Throwable))
    #[deprecated]
    pub fn new_String_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[EntrypointException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/EntrypointException.html#EntrypointException(java.lang.String.java.lang.String.java.lang.Throwable))
    #[deprecated]
    pub fn new_String_String_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[EntrypointException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/EntrypointException.html#EntrypointException(java.lang.String))
    #[deprecated]
    pub fn new_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[EntrypointException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/EntrypointException.html#EntrypointException(java.lang.Throwable))
    #[deprecated]
    pub fn new_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getKey](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/EntrypointException.html#getKey())
    pub fn getKey<'env>(
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
                        c"getKey",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [FabricLoader](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html)
pub enum FabricLoader {}
unsafe impl ::java_oxide::ReferenceType for FabricLoader {}
unsafe impl ::java_oxide::JniType for FabricLoader {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/FabricLoader")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for FabricLoader {}
impl FabricLoader {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/FabricLoader"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getInstance](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getInstance())
    pub fn getInstance<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, FabricLoader>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"getInstance",
                        c"()Lnet/fabricmc/loader/api/FabricLoader;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[invokeEntrypoints](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#invokeEntrypoints(java.lang.String.java.lang.Class.java.util.function.Consumer))
    pub fn invokeEntrypoints<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Class>,
        arg2: impl ::java_oxide::AsArg<crate::java::util::function::Consumer>,
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
                        c"invokeEntrypoints",
                        c"(Ljava/lang/String;Ljava/lang/Class;Ljava/util/function/Consumer;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getObjectShare](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getObjectShare())
    pub fn getObjectShare<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ObjectShare>>,
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
                        c"getObjectShare",
                        c"()Lnet/fabricmc/loader/api/ObjectShare;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getMappingResolver](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getMappingResolver())
    pub fn getMappingResolver<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, MappingResolver>>,
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
                        c"getMappingResolver",
                        c"()Lnet/fabricmc/loader/api/MappingResolver;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAllMods](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getAllMods())
    pub fn getAllMods<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::util::Collection>>,
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
                        c"getAllMods",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isModLoaded](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#isModLoaded(java.lang.String))
    pub fn isModLoaded<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"isModLoaded",
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isDevelopmentEnvironment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#isDevelopmentEnvironment())
    pub fn isDevelopmentEnvironment<'env>(
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
                        c"isDevelopmentEnvironment",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getEnvironmentType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getEnvironmentType())
    pub fn getEnvironmentType<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::net::fabricmc::api::EnvType>>,
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
                        c"getEnvironmentType",
                        c"()Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getRawGameVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getRawGameVersion())
    pub fn getRawGameVersion<'env>(
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
                        c"getRawGameVersion",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getGameInstance](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getGameInstance())
    #[deprecated]
    pub fn getGameInstance<'env>(
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
                        c"getGameInstance",
                        c"()Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getLaunchArguments](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/FabricLoader.html#getLaunchArguments(boolean))
    pub fn getLaunchArguments<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: bool,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<
                'env,
                ::java_oxide::ObjectArray<crate::java::lang::String, crate::java::lang::Throwable>,
            >,
        >,
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
                        c"getLaunchArguments",
                        c"(Z)[Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [LanguageAdapter](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/LanguageAdapter.html)
pub enum LanguageAdapter {}
unsafe impl ::java_oxide::ReferenceType for LanguageAdapter {}
unsafe impl ::java_oxide::JniType for LanguageAdapter {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/LanguageAdapter")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for LanguageAdapter {}
impl LanguageAdapter {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/LanguageAdapter"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getDefault](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/LanguageAdapter.html#getDefault())
    pub fn getDefault<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, LanguageAdapter>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"getDefault",
                        c"()Lnet/fabricmc/loader/api/LanguageAdapter;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[create](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/LanguageAdapter.html#create(net.fabricmc.loader.api.ModContainer.java.lang.String.java.lang.Class))
    pub fn create<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<ModContainer>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::Class>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
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
            let __jni_method = __METHOD . get_or_init (| | :: java_oxide :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"create" , c"(Lnet/fabricmc/loader/api/ModContainer;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///class [LanguageAdapterException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/LanguageAdapterException.html)
pub enum LanguageAdapterException {}
unsafe impl ::java_oxide::ReferenceType for LanguageAdapterException {}
unsafe impl ::java_oxide::JniType for LanguageAdapterException {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/LanguageAdapterException")
    }
}
impl LanguageAdapterException {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/LanguageAdapterException"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[LanguageAdapterException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/LanguageAdapterException.html#LanguageAdapterException(java.lang.String))
    pub fn new_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[LanguageAdapterException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/LanguageAdapterException.html#LanguageAdapterException(java.lang.Throwable))
    pub fn new_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[LanguageAdapterException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/LanguageAdapterException.html#LanguageAdapterException(java.lang.String.java.lang.Throwable))
    pub fn new_String_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [MappingResolver](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/MappingResolver.html)
pub enum MappingResolver {}
unsafe impl ::java_oxide::ReferenceType for MappingResolver {}
unsafe impl ::java_oxide::JniType for MappingResolver {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/MappingResolver")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for MappingResolver {}
impl MappingResolver {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/MappingResolver"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getNamespaces](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/MappingResolver.html#getNamespaces())
    pub fn getNamespaces<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::util::Collection>>,
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
                        c"getNamespaces",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getCurrentRuntimeNamespace](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/MappingResolver.html#getCurrentRuntimeNamespace())
    pub fn getCurrentRuntimeNamespace<'env>(
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
                        c"getCurrentRuntimeNamespace",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[mapClassName](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/MappingResolver.html#mapClassName(java.lang.String.java.lang.String))
    pub fn mapClassName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::String>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
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
                        c"mapClassName",
                        c"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[unmapClassName](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/MappingResolver.html#unmapClassName(java.lang.String.java.lang.String))
    pub fn unmapClassName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::String>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
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
                        c"unmapClassName",
                        c"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[mapFieldName](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/MappingResolver.html#mapFieldName(java.lang.String.java.lang.String.java.lang.String.java.lang.String))
    pub fn mapFieldName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg3: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::String>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
                ::java_oxide::AsJValue::as_jvalue(&arg3),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_oxide :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"mapFieldName" , c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[mapMethodName](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/MappingResolver.html#mapMethodName(java.lang.String.java.lang.String.java.lang.String.java.lang.String))
    pub fn mapMethodName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg2: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg3: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::String>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
                ::java_oxide::AsJValue::as_jvalue(&arg3),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_oxide :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"mapMethodName" , c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [ModContainer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ModContainer.html)
pub enum ModContainer {}
unsafe impl ::java_oxide::ReferenceType for ModContainer {}
unsafe impl ::java_oxide::JniType for ModContainer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/ModContainer")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ModContainer {}
impl ModContainer {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/ModContainer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getMetadata](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ModContainer.html#getMetadata())
    pub fn getMetadata<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, crate::net::fabricmc::loader::api::metadata::ModMetadata>,
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
                        c"getMetadata",
                        c"()Lnet/fabricmc/loader/api/metadata/ModMetadata;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getOrigin](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ModContainer.html#getOrigin())
    pub fn getOrigin<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, crate::net::fabricmc::loader::api::metadata::ModOrigin>,
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
                        c"getOrigin",
                        c"()Lnet/fabricmc/loader/api/metadata/ModOrigin;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getContainedMods](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ModContainer.html#getContainedMods())
    pub fn getContainedMods<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::util::Collection>>,
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
                        c"getContainedMods",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [ObjectShare](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ObjectShare.html)
pub enum ObjectShare {}
unsafe impl ::java_oxide::ReferenceType for ObjectShare {}
unsafe impl ::java_oxide::JniType for ObjectShare {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/ObjectShare")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ObjectShare {}
impl ObjectShare {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/ObjectShare"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[get](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ObjectShare.html#get(java.lang.String))
    pub fn get<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
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
                        c"get",
                        c"(Ljava/lang/String;)Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[whenAvailable](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ObjectShare.html#whenAvailable(java.lang.String.java.util.function.BiConsumer))
    pub fn whenAvailable<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::util::function::BiConsumer>,
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
                        c"whenAvailable",
                        c"(Ljava/lang/String;Ljava/util/function/BiConsumer;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[put](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ObjectShare.html#put(java.lang.String.java.lang.Object))
    pub fn put<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
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
                        c"put",
                        c"(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[putIfAbsent](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ObjectShare.html#putIfAbsent(java.lang.String.java.lang.Object))
    pub fn putIfAbsent<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
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
                        c"putIfAbsent",
                        c"(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[remove](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/ObjectShare.html#remove(java.lang.String))
    pub fn remove<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
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
                        c"remove",
                        c"(Ljava/lang/String;)Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [SemanticVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/SemanticVersion.html)
pub enum SemanticVersion {}
unsafe impl ::java_oxide::ReferenceType for SemanticVersion {}
unsafe impl ::java_oxide::JniType for SemanticVersion {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/SemanticVersion")
    }
}
unsafe impl ::java_oxide::AssignableTo<Version> for SemanticVersion {}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for SemanticVersion {}
impl SemanticVersion {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/SemanticVersion"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getVersionComponentCount](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/SemanticVersion.html#getVersionComponentCount())
    pub fn getVersionComponentCount<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"getVersionComponentCount",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getVersionComponent](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/SemanticVersion.html#getVersionComponent(int))
    pub fn getVersionComponent<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"getVersionComponent",
                        c"(I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[hasWildcard](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/SemanticVersion.html#hasWildcard())
    pub fn hasWildcard<'env>(
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
                        c"hasWildcard",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[compareTo](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/SemanticVersion.html#compareTo(net.fabricmc.loader.api.SemanticVersion))
    #[deprecated]
    pub fn compareTo<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<SemanticVersion>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
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
                        c"compareTo",
                        c"(Lnet/fabricmc/loader/api/SemanticVersion;)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[parse](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/SemanticVersion.html#parse(java.lang.String))
    pub fn parse<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, SemanticVersion>>,
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
                        c"parse",
                        c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/SemanticVersion;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///public static final [COMPONENT_WILDCARD](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/SemanticVersion.html#COMPONENT_WILDCARD)
    pub const COMPONENT_WILDCARD: i32 = -2147483648;
}
///interface [Version](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/Version.html)
pub enum Version {}
unsafe impl ::java_oxide::ReferenceType for Version {}
unsafe impl ::java_oxide::JniType for Version {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/Version")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for Version {}
impl Version {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/Version"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getFriendlyString](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/Version.html#getFriendlyString())
    pub fn getFriendlyString<'env>(
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
                        c"getFriendlyString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[parse](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/Version.html#parse(java.lang.String))
    pub fn parse<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Version>>,
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
                        c"parse",
                        c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/Version;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
}
///class [VersionParsingException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/VersionParsingException.html)
pub enum VersionParsingException {}
unsafe impl ::java_oxide::ReferenceType for VersionParsingException {}
unsafe impl ::java_oxide::JniType for VersionParsingException {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/VersionParsingException")
    }
}
impl VersionParsingException {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/VersionParsingException"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[VersionParsingException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/VersionParsingException.html#VersionParsingException())
    pub fn new<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[VersionParsingException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/VersionParsingException.html#VersionParsingException(java.lang.Throwable))
    pub fn new_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[VersionParsingException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/VersionParsingException.html#VersionParsingException(java.lang.String))
    pub fn new_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[VersionParsingException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/VersionParsingException.html#VersionParsingException(java.lang.String.java.lang.Throwable))
    pub fn new_String_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Throwable>,
    ) -> ::std::result::Result<
        ::java_oxide::Local<'env, Self>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
}
