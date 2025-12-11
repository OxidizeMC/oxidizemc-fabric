///final class java/lang/Class
pub enum Class {}
unsafe impl ::java_oxide::ReferenceType for Class {}
unsafe impl ::java_oxide::JniType for Class {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"java/lang/Class")
    }
}
unsafe impl ::java_oxide::AssignableTo<Object> for Class {}
impl Class {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"java/lang/Class"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///toString
    pub fn toString<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toGenericString
    pub fn toGenericString<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toGenericString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///forName
    pub fn forName_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"forName",
                        c"(Ljava/lang/String;)Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///newInstance
    #[deprecated]
    pub fn newInstance<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Object>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"newInstance",
                        c"()Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isInstance
    pub fn isInstance<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Object>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isInstance",
                        c"(Ljava/lang/Object;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isAssignableFrom
    pub fn isAssignableFrom<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Class>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isAssignableFrom",
                        c"(Ljava/lang/Class;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isInterface
    pub fn isInterface<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isInterface",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isArray
    pub fn isArray<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isArray",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isPrimitive
    pub fn isPrimitive<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isPrimitive",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isAnnotation
    pub fn isAnnotation<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isAnnotation",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isSynthetic
    pub fn isSynthetic<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isSynthetic",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getName
    pub fn getName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
    ///getSuperclass
    pub fn getSuperclass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getSuperclass",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getPackageName
    pub fn getPackageName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getPackageName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getInterfaces
    pub fn getInterfaces<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Class, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getInterfaces",
                        c"()[Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getComponentType
    pub fn getComponentType<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getComponentType",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getModifiers
    pub fn getModifiers<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"getModifiers",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getSigners
    pub fn getSigners<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Object, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getSigners",
                        c"()[Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getDeclaringClass
    pub fn getDeclaringClass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getDeclaringClass",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getEnclosingClass
    pub fn getEnclosingClass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getEnclosingClass",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getSimpleName
    pub fn getSimpleName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getSimpleName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getTypeName
    pub fn getTypeName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getTypeName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getCanonicalName
    pub fn getCanonicalName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getCanonicalName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isUnnamedClass
    pub fn isUnnamedClass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isUnnamedClass",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isAnonymousClass
    pub fn isAnonymousClass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isAnonymousClass",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isLocalClass
    pub fn isLocalClass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isLocalClass",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isMemberClass
    pub fn isMemberClass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isMemberClass",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getClasses
    pub fn getClasses<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Class, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getClasses",
                        c"()[Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getDeclaredClasses
    pub fn getDeclaredClasses<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Class, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getDeclaredClasses",
                        c"()[Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///desiredAssertionStatus
    pub fn desiredAssertionStatus<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"desiredAssertionStatus",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isEnum
    pub fn isEnum<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isEnum",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isRecord
    pub fn isRecord<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isRecord",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getEnumConstants
    pub fn getEnumConstants<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Object, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getEnumConstants",
                        c"()[Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///cast
    pub fn cast<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Object>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Object>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"cast",
                        c"(Ljava/lang/Object;)Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///asSubclass
    pub fn asSubclass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Class>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"asSubclass",
                        c"(Ljava/lang/Class;)Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isAnnotationPresent
    pub fn isAnnotationPresent<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Class>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isAnnotationPresent",
                        c"(Ljava/lang/Class;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getNestHost
    pub fn getNestHost<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getNestHost",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isNestmateOf
    pub fn isNestmateOf<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Class>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isNestmateOf",
                        c"(Ljava/lang/Class;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getNestMembers
    pub fn getNestMembers<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Class, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getNestMembers",
                        c"()[Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///descriptorString
    pub fn descriptorString<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"descriptorString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///componentType
    pub fn componentType<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"componentType",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///arrayType
    pub fn arrayType<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"arrayType",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isHidden
    pub fn isHidden<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isHidden",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getPermittedSubclasses
    pub fn getPermittedSubclasses<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Class, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getPermittedSubclasses",
                        c"()[Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isSealed
    pub fn isSealed<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isSealed",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///class java/lang/Object
pub enum Object {}
unsafe impl ::java_oxide::ReferenceType for Object {}
unsafe impl ::java_oxide::JniType for Object {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"java/lang/Object")
    }
}
impl Object {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"java/lang/Object"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///<init>
    pub fn new<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
    ///getClass
    pub fn getClass<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Class>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getClass",
                        c"()Ljava/lang/Class;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///hashCode
    pub fn hashCode<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"hashCode",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///equals
    pub fn equals<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Object>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"equals",
                        c"(Ljava/lang/Object;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toString
    pub fn toString<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///notify
    pub fn notify<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"notify",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///notifyAll
    pub fn notifyAll<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"notifyAll",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///wait
    pub fn wait<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"wait",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///wait
    pub fn wait_long<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i64,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"wait",
                        c"(J)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///wait
    pub fn wait_long_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i64,
        arg1: i32,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"wait",
                        c"(JI)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///final class java/lang/StackTraceElement
pub enum StackTraceElement {}
unsafe impl ::java_oxide::ReferenceType for StackTraceElement {}
unsafe impl ::java_oxide::JniType for StackTraceElement {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"java/lang/StackTraceElement")
    }
}
unsafe impl ::java_oxide::AssignableTo<Object> for StackTraceElement {}
impl StackTraceElement {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"java/lang/StackTraceElement"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///<init>
    pub fn new_String_String_String_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: impl ::java_oxide::AsArg<String>,
        arg2: impl ::java_oxide::AsArg<String>,
        arg3: i32,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
                ::java_oxide::AsJValue::as_jvalue(&arg3),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    pub fn new_String_String_String_String_String_String_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: impl ::java_oxide::AsArg<String>,
        arg2: impl ::java_oxide::AsArg<String>,
        arg3: impl ::java_oxide::AsArg<String>,
        arg4: impl ::java_oxide::AsArg<String>,
        arg5: impl ::java_oxide::AsArg<String>,
        arg6: i32,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
                ::java_oxide::AsJValue::as_jvalue(&arg3),
                ::java_oxide::AsJValue::as_jvalue(&arg4),
                ::java_oxide::AsJValue::as_jvalue(&arg5),
                ::java_oxide::AsJValue::as_jvalue(&arg6),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_oxide :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"<init>" , c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V"))) . as_raw () ;
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///getFileName
    pub fn getFileName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getFileName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getLineNumber
    pub fn getLineNumber<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"getLineNumber",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getModuleName
    pub fn getModuleName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getModuleName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getModuleVersion
    pub fn getModuleVersion<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getModuleVersion",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getClassLoaderName
    pub fn getClassLoaderName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getClassLoaderName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getClassName
    pub fn getClassName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getClassName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getMethodName
    pub fn getMethodName<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getMethodName",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isNativeMethod
    pub fn isNativeMethod<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isNativeMethod",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toString
    pub fn toString<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///equals
    pub fn equals<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Object>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"equals",
                        c"(Ljava/lang/Object;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///hashCode
    pub fn hashCode<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"hashCode",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///final class java/lang/String
pub enum String {}
unsafe impl ::java_oxide::ReferenceType for String {}
unsafe impl ::java_oxide::JniType for String {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"java/lang/String")
    }
}
unsafe impl ::java_oxide::AssignableTo<Object> for String {}
impl String {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"java/lang/String"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///<init>
    pub fn new<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
    ///<init>
    pub fn new_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
    ///<init>
    pub fn new_char_array<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::CharArray>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
                        c"([C)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    pub fn new_char_array_int_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::CharArray>,
        arg1: i32,
        arg2: i32,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
                        c"([CII)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    pub fn new_int_array_int_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::IntArray>,
        arg1: i32,
        arg2: i32,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
                        c"([III)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    #[deprecated]
    pub fn new_byte_array_int_int_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ByteArray>,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
                ::java_oxide::AsJValue::as_jvalue(&arg3),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"([BIII)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    #[deprecated]
    pub fn new_byte_array_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ByteArray>,
        arg1: i32,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
                        c"([BI)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    pub fn new_byte_array_int_int_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ByteArray>,
        arg1: i32,
        arg2: i32,
        arg3: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
                ::java_oxide::AsJValue::as_jvalue(&arg3),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"([BIILjava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    pub fn new_byte_array_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ByteArray>,
        arg1: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
                        c"([BLjava/lang/String;)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    pub fn new_byte_array_int_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ByteArray>,
        arg1: i32,
        arg2: i32,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
                        c"([BII)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///<init>
    pub fn new_byte_array<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ByteArray>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
                        c"([B)V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///length
    pub fn length<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"length",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isEmpty
    pub fn isEmpty<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isEmpty",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///charAt
    pub fn charAt<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<u16, ::java_oxide::Local<'env, Throwable>> {
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
                        c"charAt",
                        c"(I)C",
                    ))
                })
                .as_raw();
            __jni_env.call_char_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///codePointAt
    pub fn codePointAt<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"codePointAt",
                        c"(I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///codePointBefore
    pub fn codePointBefore<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"codePointBefore",
                        c"(I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///codePointCount
    pub fn codePointCount<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"codePointCount",
                        c"(II)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///offsetByCodePoints
    pub fn offsetByCodePoints<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"offsetByCodePoints",
                        c"(II)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getChars
    pub fn getChars<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
        arg2: impl ::java_oxide::AsArg<::java_oxide::CharArray>,
        arg3: i32,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getChars",
                        c"(II[CI)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getBytes
    #[deprecated]
    pub fn getBytes_int_int_byte_array_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
        arg2: impl ::java_oxide::AsArg<::java_oxide::ByteArray>,
        arg3: i32,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getBytes",
                        c"(II[BI)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getBytes
    pub fn getBytes_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ::java_oxide::ByteArray>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getBytes",
                        c"(Ljava/lang/String;)[B",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getBytes
    pub fn getBytes<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ::java_oxide::ByteArray>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getBytes",
                        c"()[B",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///equals
    pub fn equals<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Object>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"equals",
                        c"(Ljava/lang/Object;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///equalsIgnoreCase
    pub fn equalsIgnoreCase<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"equalsIgnoreCase",
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///compareTo
    pub fn compareTo<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"(Ljava/lang/String;)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///compareToIgnoreCase
    pub fn compareToIgnoreCase<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"compareToIgnoreCase",
                        c"(Ljava/lang/String;)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///regionMatches
    pub fn regionMatches_int_String_int_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: impl ::java_oxide::AsArg<String>,
        arg2: i32,
        arg3: i32,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"regionMatches",
                        c"(ILjava/lang/String;II)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///regionMatches
    pub fn regionMatches_boolean_int_String_int_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: bool,
        arg1: i32,
        arg2: impl ::java_oxide::AsArg<String>,
        arg3: i32,
        arg4: i32,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_oxide::AsJValue::as_jvalue(&arg0),
                ::java_oxide::AsJValue::as_jvalue(&arg1),
                ::java_oxide::AsJValue::as_jvalue(&arg2),
                ::java_oxide::AsJValue::as_jvalue(&arg3),
                ::java_oxide::AsJValue::as_jvalue(&arg4),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"regionMatches",
                        c"(ZILjava/lang/String;II)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///startsWith
    pub fn startsWith_String_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: i32,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"startsWith",
                        c"(Ljava/lang/String;I)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///startsWith
    pub fn startsWith_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"startsWith",
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///endsWith
    pub fn endsWith<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"endsWith",
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///hashCode
    pub fn hashCode<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"hashCode",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///indexOf
    pub fn indexOf_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"indexOf",
                        c"(I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///indexOf
    pub fn indexOf_int_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"indexOf",
                        c"(II)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///indexOf
    pub fn indexOf_int_int_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"indexOf",
                        c"(III)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///lastIndexOf
    pub fn lastIndexOf_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"lastIndexOf",
                        c"(I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///lastIndexOf
    pub fn lastIndexOf_int_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"lastIndexOf",
                        c"(II)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///indexOf
    pub fn indexOf_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"indexOf",
                        c"(Ljava/lang/String;)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///indexOf
    pub fn indexOf_String_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"indexOf",
                        c"(Ljava/lang/String;I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///indexOf
    pub fn indexOf_String_int_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: i32,
        arg2: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"indexOf",
                        c"(Ljava/lang/String;II)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///lastIndexOf
    pub fn lastIndexOf_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"lastIndexOf",
                        c"(Ljava/lang/String;)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///lastIndexOf
    pub fn lastIndexOf_String_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: i32,
    ) -> ::std::result::Result<i32, ::java_oxide::Local<'env, Throwable>> {
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
                        c"lastIndexOf",
                        c"(Ljava/lang/String;I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///substring
    pub fn substring_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"substring",
                        c"(I)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///substring
    pub fn substring_int_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
        arg1: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"substring",
                        c"(II)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///concat
    pub fn concat<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"concat",
                        c"(Ljava/lang/String;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///replace
    pub fn replace_char_char<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: u16,
        arg1: u16,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"replace",
                        c"(CC)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///matches
    pub fn matches<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"(Ljava/lang/String;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///replaceFirst
    pub fn replaceFirst<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"replaceFirst",
                        c"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///replaceAll
    pub fn replaceAll<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"replaceAll",
                        c"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///split
    pub fn split_String_int<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<String, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"split",
                        c"(Ljava/lang/String;I)[Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///splitWithDelimiters
    pub fn splitWithDelimiters<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<String, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"splitWithDelimiters",
                        c"(Ljava/lang/String;I)[Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///split
    pub fn split_String<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<String, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"split",
                        c"(Ljava/lang/String;)[Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toLowerCase
    pub fn toLowerCase<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toLowerCase",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toUpperCase
    pub fn toUpperCase<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toUpperCase",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///trim
    pub fn trim<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"trim",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///strip
    pub fn strip<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"strip",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///stripLeading
    pub fn stripLeading<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"stripLeading",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///stripTrailing
    pub fn stripTrailing<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"stripTrailing",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///isBlank
    pub fn isBlank<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, Throwable>> {
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
                        c"isBlank",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///indent
    pub fn indent<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"indent",
                        c"(I)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///stripIndent
    pub fn stripIndent<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"stripIndent",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///translateEscapes
    pub fn translateEscapes<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"translateEscapes",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toString
    pub fn toString<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toCharArray
    pub fn toCharArray<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, ::java_oxide::CharArray>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toCharArray",
                        c"()[C",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///format
    pub fn format_String_Object_array<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: impl ::java_oxide::AsArg<::java_oxide::ObjectArray<Object, Throwable>>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"format",
                        c"(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///formatted
    pub fn formatted<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ObjectArray<Object, Throwable>>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"formatted",
                        c"([Ljava/lang/Object;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_Object<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<Object>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"(Ljava/lang/Object;)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_char_array<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::CharArray>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"([C)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_char_array_int_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::CharArray>,
        arg1: i32,
        arg2: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"valueOf",
                        c"([CII)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///copyValueOf
    pub fn copyValueOf_char_array_int_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::CharArray>,
        arg1: i32,
        arg2: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"copyValueOf",
                        c"([CII)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///copyValueOf
    pub fn copyValueOf_char_array<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::CharArray>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"copyValueOf",
                        c"([C)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_boolean<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: bool,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"(Z)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_char<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: u16,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"(C)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_int<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"(I)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_long<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: i64,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"(J)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_float<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: f32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"(F)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///valueOf
    pub fn valueOf_double<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: f64,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"(D)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///intern
    pub fn intern<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"intern",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///repeat
    pub fn repeat<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"repeat",
                        c"(I)Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///class java/lang/Throwable
pub enum Throwable {}
unsafe impl ::java_oxide::ReferenceType for Throwable {}
unsafe impl ::java_oxide::JniType for Throwable {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"java/lang/Throwable")
    }
}
unsafe impl ::java_oxide::AssignableTo<Object> for Throwable {}
impl Throwable {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<Object>> =
            ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"java/lang/Throwable"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///<init>
    pub fn new<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
    ///<init>
    pub fn new_String<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<String>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
    ///<init>
    pub fn new_String_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<String>,
        arg1: impl ::java_oxide::AsArg<Throwable>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
    ///<init>
    pub fn new_Throwable<'env>(
        __jni_env: ::java_oxide::Env<'env>,
        arg0: impl ::java_oxide::AsArg<Throwable>,
    ) -> ::std::result::Result<::java_oxide::Local<'env, Self>, ::java_oxide::Local<'env, Throwable>>
    {
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
    ///getMessage
    pub fn getMessage<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getMessage",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getLocalizedMessage
    pub fn getLocalizedMessage<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getLocalizedMessage",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getCause
    pub fn getCause<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Throwable>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getCause",
                        c"()Ljava/lang/Throwable;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///initCause
    pub fn initCause<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Throwable>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Throwable>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"initCause",
                        c"(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///toString
    pub fn toString<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, String>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"toString",
                        c"()Ljava/lang/String;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///printStackTrace
    pub fn printStackTrace<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"printStackTrace",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///fillInStackTrace
    pub fn fillInStackTrace<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_oxide::Local<'env, Throwable>>,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"fillInStackTrace",
                        c"()Ljava/lang/Throwable;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getStackTrace
    pub fn getStackTrace<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<StackTraceElement, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getStackTrace",
                        c"()[Ljava/lang/StackTraceElement;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///setStackTrace
    pub fn setStackTrace<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<::java_oxide::ObjectArray<StackTraceElement, Throwable>>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"setStackTrace",
                        c"([Ljava/lang/StackTraceElement;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///addSuppressed
    pub fn addSuppressed<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<Throwable>,
    ) -> ::std::result::Result<(), ::java_oxide::Local<'env, Throwable>> {
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
                        c"addSuppressed",
                        c"(Ljava/lang/Throwable;)V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///getSuppressed
    pub fn getSuppressed<'env>(
        self: &::java_oxide::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_oxide::Local<'env, ::java_oxide::ObjectArray<Throwable, Throwable>>,
        >,
        ::java_oxide::Local<'env, Throwable>,
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
                        c"getSuppressed",
                        c"()[Ljava/lang/Throwable;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
