pub mod version;
///interface [ContactInformation](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ContactInformation.html)
pub enum ContactInformation {}
unsafe impl ::java_oxide::ReferenceType for ContactInformation {}
unsafe impl ::java_oxide::JniType for ContactInformation {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ContactInformation")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ContactInformation {}
impl ContactInformation {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ContactInformation"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///**get** public static final [EMPTY](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ContactInformation.html#EMPTY)
    pub fn EMPTY<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ContactInformation>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"EMPTY",
                        c"Lnet/fabricmc/loader/api/metadata/ContactInformation;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///class [ContactInformation.1](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ContactInformation.1.html)
enum ContactInformation__1 {}
unsafe impl ::java_oxide::ReferenceType for ContactInformation__1 {}
unsafe impl ::java_oxide::JniType for ContactInformation__1 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ContactInformation$1")
    }
}
unsafe impl ::java_oxide::AssignableTo<ContactInformation> for ContactInformation__1 {}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ContactInformation__1 {}
impl ContactInformation__1 {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/metadata/ContactInformation$1"),
                )
                .as_global()
            })
            .as_raw()
    }
}
///interface [CustomValue](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.html)
pub enum CustomValue {}
unsafe impl ::java_oxide::ReferenceType for CustomValue {}
unsafe impl ::java_oxide::JniType for CustomValue {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for CustomValue {}
impl CustomValue {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/CustomValue"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.html#getType())
    pub fn getType<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>>,
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
                        c"getType",
                        c"()Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAsObject](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.html#getAsObject())
    pub fn getAsObject<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvObject>>,
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
                        c"getAsObject",
                        c"()Lnet/fabricmc/loader/api/metadata/CustomValue$CvObject;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAsArray](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.html#getAsArray())
    pub fn getAsArray<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvArray>>,
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
                        c"getAsArray",
                        c"()Lnet/fabricmc/loader/api/metadata/CustomValue$CvArray;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAsString](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.html#getAsString())
    pub fn getAsString<'env>(
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
                        c"getAsString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAsBoolean](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.html#getAsBoolean())
    pub fn getAsBoolean<'env>(
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
                        c"getAsBoolean",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [CustomValue.CvArray](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvArray.html)
pub enum CustomValue_CvArray {}
unsafe impl ::java_oxide::ReferenceType for CustomValue_CvArray {}
unsafe impl ::java_oxide::JniType for CustomValue_CvArray {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue$CvArray")
    }
}
unsafe impl ::java_oxide::AssignableTo<CustomValue> for CustomValue_CvArray {}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for CustomValue_CvArray {}
impl CustomValue_CvArray {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/metadata/CustomValue$CvArray"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[size](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvArray.html#size())
    pub fn size<'env>(
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
                        c"size",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[get](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvArray.html#get(int))
    pub fn get<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, CustomValue>>,
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
                        c"(I)Lnet/fabricmc/loader/api/metadata/CustomValue;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [CustomValue.CvObject](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvObject.html)
pub enum CustomValue_CvObject {}
unsafe impl ::java_oxide::ReferenceType for CustomValue_CvObject {}
unsafe impl ::java_oxide::JniType for CustomValue_CvObject {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue$CvObject")
    }
}
unsafe impl ::java_oxide::AssignableTo<CustomValue> for CustomValue_CvObject {}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for CustomValue_CvObject {}
impl CustomValue_CvObject {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/metadata/CustomValue$CvObject"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[size](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvObject.html#size())
    pub fn size<'env>(
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
                        c"size",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[containsKey](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvObject.html#containsKey(java.lang.String))
    pub fn containsKey<'env>(
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
                        c"containsKey",
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[get](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvObject.html#get(java.lang.String))
    pub fn get<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, CustomValue>>,
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
                        c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/metadata/CustomValue;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [CustomValue.CvType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html)
pub enum CustomValue_CvType {}
unsafe impl ::java_oxide::ReferenceType for CustomValue_CvType {}
unsafe impl ::java_oxide::JniType for CustomValue_CvType {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue$CvType")
    }
}
impl CustomValue_CvType {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/CustomValue$CvType"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#values())
    pub fn values<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<
                'env,
                ::java_oxide::ObjectArray<CustomValue_CvType, crate::java::lang::Throwable>,
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
                        c"()[Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[valueOf](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#valueOf(java.lang.String))
    pub fn valueOf<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_oxide :: JMethodID :: from_raw (__jni_env . require_static_method (__jni_class , c"valueOf" , c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;"))) . as_raw () ;
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [OBJECT](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#OBJECT)
    pub fn OBJECT<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"OBJECT",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [ARRAY](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#ARRAY)
    pub fn ARRAY<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"ARRAY",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [STRING](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#STRING)
    pub fn STRING<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"STRING",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [NUMBER](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#NUMBER)
    pub fn NUMBER<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"NUMBER",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [BOOLEAN](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#BOOLEAN)
    pub fn BOOLEAN<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"BOOLEAN",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [NULL](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#NULL)
    pub fn NULL<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"NULL",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [ModDependency](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.html)
pub enum ModDependency {}
unsafe impl ::java_oxide::ReferenceType for ModDependency {}
unsafe impl ::java_oxide::JniType for ModDependency {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModDependency")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ModDependency {}
impl ModDependency {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModDependency"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getKind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.html#getKind())
    pub fn getKind<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>>,
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
                        c"getKind",
                        c"()Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getModId](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.html#getModId())
    pub fn getModId<'env>(
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
                        c"getModId",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[matches](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.html#matches(net.fabricmc.loader.api.Version))
    pub fn matches<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::net::fabricmc::loader::api::Version>,
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
                        c"matches",
                        c"(Lnet/fabricmc/loader/api/Version;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getVersionRequirements](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.html#getVersionRequirements())
    pub fn getVersionRequirements<'env>(
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
                        c"getVersionRequirements",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [ModDependency.Kind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html)
pub enum ModDependency_Kind {}
unsafe impl ::java_oxide::ReferenceType for ModDependency_Kind {}
unsafe impl ::java_oxide::JniType for ModDependency_Kind {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModDependency$Kind")
    }
}
impl ModDependency_Kind {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModDependency$Kind"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#values())
    pub fn values<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<
                'env,
                ::java_oxide::ObjectArray<ModDependency_Kind, crate::java::lang::Throwable>,
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
                        c"()[Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[valueOf](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#valueOf(java.lang.String))
    pub fn valueOf<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_oxide :: JMethodID :: from_raw (__jni_env . require_static_method (__jni_class , c"valueOf" , c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;"))) . as_raw () ;
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getKey](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#getKey())
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
    ///[isPositive](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#isPositive())
    pub fn isPositive<'env>(
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
                        c"isPositive",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isSoft](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#isSoft())
    pub fn isSoft<'env>(
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
                        c"isSoft",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[parse](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#parse(java.lang.String))
    pub fn parse<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_oxide :: JMethodID :: from_raw (__jni_env . require_static_method (__jni_class , c"parse" , c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;"))) . as_raw () ;
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [DEPENDS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#DEPENDS)
    pub fn DEPENDS<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"DEPENDS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [RECOMMENDS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#RECOMMENDS)
    pub fn RECOMMENDS<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"RECOMMENDS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SUGGESTS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#SUGGESTS)
    pub fn SUGGESTS<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SUGGESTS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [CONFLICTS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#CONFLICTS)
    pub fn CONFLICTS<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"CONFLICTS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [BREAKS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#BREAKS)
    pub fn BREAKS<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"BREAKS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///enum [ModEnvironment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModEnvironment.html)
pub enum ModEnvironment {}
unsafe impl ::java_oxide::ReferenceType for ModEnvironment {}
unsafe impl ::java_oxide::JniType for ModEnvironment {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModEnvironment")
    }
}
impl ModEnvironment {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModEnvironment"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModEnvironment.html#values())
    pub fn values<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<
                'env,
                ::java_oxide::ObjectArray<ModEnvironment, crate::java::lang::Throwable>,
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
                        c"()[Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[valueOf](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModEnvironment.html#valueOf(java.lang.String))
    pub fn valueOf<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ModEnvironment>>,
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
                        c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[matches](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModEnvironment.html#matches(net.fabricmc.api.EnvType))
    pub fn matches<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::net::fabricmc::api::EnvType>,
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
                        c"matches",
                        c"(Lnet/fabricmc/api/EnvType;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [CLIENT](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModEnvironment.html#CLIENT)
    pub fn CLIENT<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModEnvironment>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"CLIENT",
                        c"Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SERVER](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModEnvironment.html#SERVER)
    pub fn SERVER<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModEnvironment>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SERVER",
                        c"Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [UNIVERSAL](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModEnvironment.html#UNIVERSAL)
    pub fn UNIVERSAL<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModEnvironment>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"UNIVERSAL",
                        c"Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [ModMetadata](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html)
pub enum ModMetadata {}
unsafe impl ::java_oxide::ReferenceType for ModMetadata {}
unsafe impl ::java_oxide::JniType for ModMetadata {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModMetadata")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ModMetadata {}
impl ModMetadata {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModMetadata"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getType())
    pub fn getType<'env>(
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
                        c"getType",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getId](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getId())
    pub fn getId<'env>(
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
                        c"getId",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getProvides](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getProvides())
    pub fn getProvides<'env>(
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
                        c"getProvides",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getVersion())
    pub fn getVersion<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, crate::net::fabricmc::loader::api::Version>,
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
                        c"getVersion",
                        c"()Lnet/fabricmc/loader/api/Version;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getEnvironment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getEnvironment())
    pub fn getEnvironment<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ModEnvironment>>,
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
                        c"getEnvironment",
                        c"()Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getDependencies](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getDependencies())
    pub fn getDependencies<'env>(
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
                        c"getDependencies",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getDepends](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getDepends())
    #[deprecated]
    pub fn getDepends<'env>(
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
                        c"getDepends",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getRecommends](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getRecommends())
    #[deprecated]
    pub fn getRecommends<'env>(
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
                        c"getRecommends",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getSuggests](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getSuggests())
    #[deprecated]
    pub fn getSuggests<'env>(
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
                        c"getSuggests",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getConflicts](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getConflicts())
    #[deprecated]
    pub fn getConflicts<'env>(
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
                        c"getConflicts",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getBreaks](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getBreaks())
    #[deprecated]
    pub fn getBreaks<'env>(
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
                        c"getBreaks",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getName](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getName())
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
    ///[getDescription](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getDescription())
    pub fn getDescription<'env>(
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
                        c"getDescription",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAuthors](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getAuthors())
    pub fn getAuthors<'env>(
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
                        c"getAuthors",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getContributors](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getContributors())
    pub fn getContributors<'env>(
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
                        c"getContributors",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getContact](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getContact())
    pub fn getContact<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ContactInformation>>,
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
                        c"getContact",
                        c"()Lnet/fabricmc/loader/api/metadata/ContactInformation;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getLicense](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getLicense())
    pub fn getLicense<'env>(
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
                        c"getLicense",
                        c"()Ljava/util/Collection;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[containsCustomValue](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#containsCustomValue(java.lang.String))
    pub fn containsCustomValue<'env>(
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
                        c"containsCustomValue",
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getCustomValue](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#getCustomValue(java.lang.String))
    pub fn getCustomValue<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, CustomValue>>,
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
                        c"getCustomValue",
                        c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/metadata/CustomValue;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[containsCustomElement](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModMetadata.html#containsCustomElement(java.lang.String))
    #[deprecated]
    pub fn containsCustomElement<'env>(
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
                        c"containsCustomElement",
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [ModOrigin](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.html)
pub enum ModOrigin {}
unsafe impl ::java_oxide::ReferenceType for ModOrigin {}
unsafe impl ::java_oxide::JniType for ModOrigin {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModOrigin")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for ModOrigin {}
impl ModOrigin {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModOrigin"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getKind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.html#getKind())
    pub fn getKind<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ModOrigin_Kind>>,
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
                        c"getKind",
                        c"()Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getParentModId](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.html#getParentModId())
    pub fn getParentModId<'env>(
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
                        c"getParentModId",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getParentSubLocation](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.html#getParentSubLocation())
    pub fn getParentSubLocation<'env>(
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
                        c"getParentSubLocation",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [ModOrigin.Kind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html)
pub enum ModOrigin_Kind {}
unsafe impl ::java_oxide::ReferenceType for ModOrigin_Kind {}
unsafe impl ::java_oxide::JniType for ModOrigin_Kind {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModOrigin$Kind")
    }
}
impl ModOrigin_Kind {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModOrigin$Kind"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#values())
    pub fn values<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<
                'env,
                ::java_oxide::ObjectArray<ModOrigin_Kind, crate::java::lang::Throwable>,
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
                        c"()[Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[valueOf](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#valueOf(java.lang.String))
    pub fn valueOf<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<crate::java::lang::String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ModOrigin_Kind>>,
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
                        c"(Ljava/lang/String;)Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [PATH](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#PATH)
    pub fn PATH<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModOrigin_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"PATH",
                        c"Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [NESTED](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#NESTED)
    pub fn NESTED<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModOrigin_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"NESTED",
                        c"Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [UNKNOWN](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#UNKNOWN)
    pub fn UNKNOWN<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::option::Option<::java_oxide::Local<'env, ModOrigin_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_oxide::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_oxide::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"UNKNOWN",
                        c"Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [Person](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/Person.html)
pub enum Person {}
unsafe impl ::java_oxide::ReferenceType for Person {}
unsafe impl ::java_oxide::JniType for Person {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/Person")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for Person {}
impl Person {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/Person"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getName](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/Person.html#getName())
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
    ///[getContact](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/loader/api/metadata/Person.html#getContact())
    pub fn getContact<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ContactInformation>>,
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
                        c"getContact",
                        c"()Lnet/fabricmc/loader/api/metadata/ContactInformation;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
