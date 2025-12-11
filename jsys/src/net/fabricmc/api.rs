///interface [ClientModInitializer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/ClientModInitializer.html)
pub enum ClientModInitializer {}
unsafe impl ::java_oxide::ReferenceType for ClientModInitializer {}
unsafe impl ::java_oxide::JniType for ClientModInitializer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/ClientModInitializer")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ClientModInitializer {}
impl ClientModInitializer {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/ClientModInitializer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onInitializeClient](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/ClientModInitializer.html#onInitializeClient())
    pub fn onInitializeClient<'env>(
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
                        c"onInitializeClient",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [DedicatedServerModInitializer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/DedicatedServerModInitializer.html)
pub enum DedicatedServerModInitializer {}
unsafe impl ::java_oxide::ReferenceType for DedicatedServerModInitializer {}
unsafe impl ::java_oxide::JniType for DedicatedServerModInitializer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/DedicatedServerModInitializer")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object>
    for DedicatedServerModInitializer
{
}
impl DedicatedServerModInitializer {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/DedicatedServerModInitializer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onInitializeServer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/DedicatedServerModInitializer.html#onInitializeServer())
    pub fn onInitializeServer<'env>(
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
                        c"onInitializeServer",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [EnvType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvType.html)
pub enum EnvType {}
unsafe impl ::java_oxide::ReferenceType for EnvType {}
unsafe impl ::java_oxide::JniType for EnvType {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/EnvType")
    }
}
impl EnvType {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/EnvType"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvType.html#values())
    pub fn values<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<
                'env,
                ::java_oxide::ObjectArray<EnvType, crate::java::lang::Throwable>,
            >,
        >,
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
                        c"values",
                        c"()[Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[valueOf](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvType.html#valueOf(java.lang.String))
    pub fn valueOf<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, EnvType>>,
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
                        c"valueOf",
                        c"(Ljava/lang/String;)Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [CLIENT](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvType.html#CLIENT)
    pub fn CLIENT<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, EnvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"CLIENT",
                        c"Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SERVER](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvType.html#SERVER)
    pub fn SERVER<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, EnvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SERVER",
                        c"Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [Environment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/Environment.html)
pub enum Environment {}
unsafe impl ::java_oxide::ReferenceType for Environment {}
unsafe impl ::java_oxide::JniType for Environment {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/Environment")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for Environment {}
impl Environment {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/Environment"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[value](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/Environment.html#value())
    pub fn value<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, EnvType>>,
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
                        c"value",
                        c"()Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [EnvironmentInterface](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvironmentInterface.html)
pub enum EnvironmentInterface {}
unsafe impl ::java_oxide::ReferenceType for EnvironmentInterface {}
unsafe impl ::java_oxide::JniType for EnvironmentInterface {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/EnvironmentInterface")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for EnvironmentInterface {}
impl EnvironmentInterface {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/EnvironmentInterface"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[value](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvironmentInterface.html#value())
    pub fn value<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, EnvType>>,
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
                        c"value",
                        c"()Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[itf](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvironmentInterface.html#itf())
    pub fn itf<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Class>>,
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
                        c"itf",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [EnvironmentInterfaces](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvironmentInterfaces.html)
pub enum EnvironmentInterfaces {}
unsafe impl ::java_oxide::ReferenceType for EnvironmentInterfaces {}
unsafe impl ::java_oxide::JniType for EnvironmentInterfaces {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/EnvironmentInterfaces")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for EnvironmentInterfaces {}
impl EnvironmentInterfaces {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/EnvironmentInterfaces"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[value](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/EnvironmentInterfaces.html#value())
    pub fn value<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<
                'env,
                ::java_oxide::ObjectArray<EnvironmentInterface, crate::java::lang::Throwable>,
            >,
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
                        c"value",
                        c"()[Lnet/fabricmc/api/EnvironmentInterface;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [ModInitializer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/ModInitializer.html)
pub enum ModInitializer {}
unsafe impl ::java_oxide::ReferenceType for ModInitializer {}
unsafe impl ::java_oxide::JniType for ModInitializer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/api/ModInitializer")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ModInitializer {}
impl ModInitializer {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/api/ModInitializer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onInitialize](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/api/ModInitializer.html#onInitialize())
    pub fn onInitialize<'env>(
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
                        c"onInitialize",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
