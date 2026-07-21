#[allow(dead_code)]
pub(crate) type __JniErr = ::prebindgen::lang::JniBindingError<()>;
/// See module-level docs at [`owned_object_prerequisite_items`].
#[allow(dead_code)]
pub(crate) struct OwnedObject<T: ?Sized> {
    ptr: *const T,
}
impl<T: ?Sized> std::ops::Deref for OwnedObject<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
impl<T: ?Sized> std::ops::DerefMut for OwnedObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}
impl<T: ?Sized> OwnedObject<T> {
    /// Borrow a `T` whose backing `Box<T>` lives on the
    /// Java side. Stores only the pointer; the wrapper
    /// does not own the heap allocation and never frees
    /// it on drop.
    ///
    /// # Safety
    ///
    /// `ptr` must be the result of an earlier
    /// `Box::into_raw(Box::new(v))` and the allocation
    /// must still be live (Java still owns it). The Java
    /// side is responsible for sequencing this call
    /// against any concurrent free or consume (via
    /// `NativeHandle.withPtr` read-lock vs `consume` /
    /// `close` write-lock) so the borrow cannot race a
    /// deallocation on the same pointer.
    #[allow(dead_code)]
    pub(crate) unsafe fn from_raw(ptr: *const T) -> Self {
        Self { ptr }
    }
}
#[allow(non_snake_case, dead_code)]
pub(crate) fn signal_binding_error(
    env: &mut jni::JNIEnv,
    sink: &jni::objects::JObject,
    mid: &::prebindgen::lang::CachedIfaceMethod,
    fqn: &str,
    descr: &str,
    je: &str,
) {
    if env.exception_check().unwrap_or(false) {
        return;
    }
    let __je: jni::objects::JObject = match env.new_string(je) {
        Ok(s) => s.into(),
        Err(e) => {
            tracing::error!("signal_binding_error: new_string failed: {}", e);
            return;
        }
    };
    let __args = [
        jni::sys::jvalue {
            l: __je.as_raw(),
        },
    ];
    if let Err(e) = mid.call_object(env, fqn, "run", descr, sink, &__args) {
        tracing::error!("signal_binding_error: error-callback invoke failed: {}", e);
    }
}
#[allow(non_snake_case, dead_code)]
pub(crate) fn signal_domain_error(
    env: &mut jni::JNIEnv,
    sink: &jni::objects::JObject,
    mid: &::prebindgen::lang::CachedIfaceMethod,
    fqn: &str,
    descr: &str,
    ze: &[jni::sys::jvalue],
) {
    if env.exception_check().unwrap_or(false) {
        return;
    }
    if let Err(e) = mid.call_object(env, fqn, "run", descr, sink, ze) {
        tracing::error!("signal_domain_error: error-callback invoke failed: {}", e);
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_Encoding_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Encoding));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Encoding>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_ZBytes_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZBytes));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::ZBytes>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_config_Config_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Config));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Config>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_keyexpr_KeyExpr_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::KeyExpr));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::KeyExpr>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_liveliness_LivelinessToken_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::LivelinessToken));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::LivelinessToken>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_Publisher_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Publisher));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Publisher>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_Subscriber_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Subscriber));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Subscriber>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Querier_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Querier));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Querier>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Query_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Query));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Query>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Queryable_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Queryable));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Queryable>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ReplyError_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ReplyError));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::ReplyError>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Reply_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Reply));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Reply>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_sample_Sample_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Sample));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Sample>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_Hello_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Hello));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Hello>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_Scout_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Scout));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Scout>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_session_Session_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Session));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Session>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_time_Timestamp_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Timestamp));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Timestamp>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
const _: () = {
    const fn __assert_copy<T: ::core::marker::Copy>() {}
    __assert_copy::<zenoh_flat::ZenohId>();
};
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_zenoh_bytes())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohBytesId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_zenoh_bytes())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_zenoh_string())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohStringId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_zenoh_string())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohSerialized<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_zenoh_serialized())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohSerializedId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_zenoh_serialized())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOctetStream<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_octet_stream())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOctetStreamId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_octet_stream())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextPlain<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_plain())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextPlainId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_plain())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_json())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_json())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_json())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJsonId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_json())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCdr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_cdr())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCdrId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_cdr())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCbor<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_cbor())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCborId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_cbor())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_yaml())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYamlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_yaml())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_yaml())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextYamlId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_yaml())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_json5())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJson5Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_json5())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationPythonSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_python_serialized_object())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationPythonSerializedObjectId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_python_serialized_object())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationProtobuf<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_protobuf())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationProtobufId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_protobuf())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJavaSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_java_serialized_object())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJavaSerializedObjectId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_java_serialized_object())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOpenmetricsText<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_openmetrics_text())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOpenmetricsTextId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_openmetrics_text())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImagePng<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_png())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImagePngId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_png())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageJpeg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_jpeg())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageJpegId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_jpeg())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageGif<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_gif())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageGifId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_gif())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageBmp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_bmp())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageBmpId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_bmp())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageWebp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_webp())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageWebpId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_webp())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_xml())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXmlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_xml())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXWwwFormUrlencoded<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_x_www_form_urlencoded())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXWwwFormUrlencodedId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_x_www_form_urlencoded())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextHtml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_html())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextHtmlId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_html())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_xml())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextXmlId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_xml())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCss<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_css())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCssId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_css())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJavascript<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_javascript())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJavascriptId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_javascript())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextMarkdown<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_markdown())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextMarkdownId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_markdown())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCsv<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_csv())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCsvId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_csv())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSql<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_sql())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSqlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_sql())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCoapPayload<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_coap_payload())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCoapPayloadId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_coap_payload())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonPatchJson<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_json_patch_json())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonPatchJsonId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_json_patch_json())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonSeq<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_json_seq())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonSeqId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_json_seq())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonpath<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_jsonpath())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonpathId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_jsonpath())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJwt<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_jwt())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJwtId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_jwt())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_mp4())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationMp4Id<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_mp4())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSoapXml<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_soap_xml())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSoapXmlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_soap_xml())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYang<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_yang())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYangId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_yang())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioAac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_aac())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioAacId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_aac())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioFlac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_flac())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioFlacId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_flac())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_mp4())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioMp4Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_mp4())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_ogg())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioOggId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_ogg())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioVorbis<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_vorbis())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioVorbisId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_vorbis())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH261<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h261())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH261Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h261())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH263<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h263())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH263Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h263())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH264<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h264())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH264Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h264())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH265<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h265())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH265Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h265())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH266<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h266())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH266Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h266())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_mp4())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoMp4Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_mp4())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_ogg())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoOggId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_ogg())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoRaw<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_raw())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoRawId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_raw())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp8<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_vp8())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp8Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_vp8())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp9<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_vp9())
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp9Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_vp9())
    };
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Config_to_jlong_d1f60c7d<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Config,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn CongestionControl_to_jint_62e38379<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::CongestionControl,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ConsolidationMode_to_jint_dd4eaedc<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ConsolidationMode,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Encoding_to_jlong_072adb3b<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::Encoding,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Encoding_to_jlong_e0e31e0d<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Encoding,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Hello_to_jlong_bbd3fc65<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Hello,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JByteArray_to_Option_Vec_u8_6f4428ab<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JByteArray<'v>,
) -> ::core::result::Result<Option<Vec<u8>>, __JniErr> {
    Ok({ if v.is_null() { None } else { Some(JByteArray_to_Vec_u8_7936d5de(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JByteArray_to_Vec_u8_7936d5de<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JByteArray<'v>,
) -> ::core::result::Result<Vec<u8>, __JniErr> {
    Ok({
        env.convert_byte_array(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("decode_byte_array: {}", e))
            })?
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JByteArray_to_ZenohId_2caee6f1<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JByteArray<'v>,
) -> ::core::result::Result<zenoh_flat::ZenohId, __JniErr> {
    Ok({
        let __bytes = env
            .convert_byte_array(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("value-blob decode: {}", e))
            })?;
        if __bytes.len() != ::core::mem::size_of::<zenoh_flat::ZenohId>() {
            return ::core::result::Result::Err(
                <__JniErr as ::core::convert::From<
                    String,
                >>::from("value-blob decode: wrong byte length".to_string()),
            );
        }
        unsafe {
            ::core::ptr::read_unaligned(__bytes.as_ptr() as *const zenoh_flat::ZenohId)
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_CongestionControl_7053bb49<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<zenoh_flat::CongestionControl>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jint = env
                .call_method(&v, "intValue", "()I", &[])
                .and_then(|val| val.i())
                .map(|__x| __x as jni::sys::jint)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jint_to_CongestionControl_62e38379(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_ConsolidationMode_25de8913<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<zenoh_flat::ConsolidationMode>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jint = env
                .call_method(&v, "intValue", "()I", &[])
                .and_then(|val| val.i())
                .map(|__x| __x as jni::sys::jint)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jint_to_ConsolidationMode_dd4eaedc(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_Priority_ad5cbb32<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<zenoh_flat::Priority>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jint = env
                .call_method(&v, "intValue", "()I", &[])
                .and_then(|val| val.i())
                .map(|__x| __x as jni::sys::jint)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jint_to_Priority_447102d2(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_QueryTarget_08d4f26d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<zenoh_flat::QueryTarget>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jint = env
                .call_method(&v, "intValue", "()I", &[])
                .and_then(|val| val.i())
                .map(|__x| __x as jni::sys::jint)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jint_to_QueryTarget_71d4db6a(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_Reliability_60b5e063<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<zenoh_flat::Reliability>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jint = env
                .call_method(&v, "intValue", "()I", &[])
                .and_then(|val| val.i())
                .map(|__x| __x as jni::sys::jint)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jint_to_Reliability_5d4a96c8(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_ReplyKeyExpr_91b36eb3<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<zenoh_flat::ReplyKeyExpr>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jint = env
                .call_method(&v, "intValue", "()I", &[])
                .and_then(|val| val.i())
                .map(|__x| __x as jni::sys::jint)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jint_to_ReplyKeyExpr_0d9719f5(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_bool_5c82fffd<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<bool>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jboolean = env
                .call_method(&v, "booleanValue", "()Z", &[])
                .and_then(|val| val.z())
                .map(|__x| __x as jni::sys::jboolean)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jboolean_to_bool_31306d98(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_i32_bf7397c9<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<i32>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jint = env
                .call_method(&v, "intValue", "()I", &[])
                .and_then(|val| val.i())
                .map(|__x| __x as jni::sys::jint)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jint_to_i32_a3e3b6ef(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_Option_i64_2ba9a5ed<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<Option<i64>, __JniErr> {
    Ok({
        if !v.is_null() {
            let __unboxed: jni::sys::jlong = env
                .call_method(&v, "longValue", "()J", &[])
                .and_then(|val| val.j())
                .map(|__x| __x as jni::sys::jlong)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Option unbox: {}", e)))?;
            Some(jlong_to_i64_fbf9a9bc(env, &__unboxed)?)
        } else {
            None
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_Hello_Send_Sync_static_d937ec1a<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Hello) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Hello)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(&__invoke_class, "run", "(I[BLjava/util/List;)V")
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Hello)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Hello| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Hello)", e)))?;
                env.push_local_frame(16)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Hello)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::sys::jvalue = {
                        let __enc0 = match WhatAmI_to_jint_4c5d5738(
                            &mut env,
                            zenoh_flat::hello_get_whatami(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc0 }
                    };
                    let __cb0_obj1: jni::objects::JObject = {
                        let __enc1 = match ZenohId_to_JByteArray_2caee6f1(
                            &mut env,
                            zenoh_flat::hello_get_zid(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc1.into()
                    };
                    let __cb0_obj2: jni::objects::JObject = {
                        let __enc2 = match Vec_String_to_JObject_1e282499(
                            &mut env,
                            zenoh_flat::hello_get_locators(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc2
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                __cb0_obj0,
                                jni::sys::jvalue {
                                    l: __cb0_obj1.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj2.as_raw(),
                                },
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Hello)"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_Query_Send_Sync_static_6c353bcb<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Query) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Query)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Long;Ljava/lang/Integer;Ljava/lang/String;Ljava/lang/Long;IJ)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Query)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Query| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Query)", e)))?;
                env.push_local_frame(22)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Query)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::objects::JObject = {
                        let __enc0 = match str_to_JString_7b77dc67(
                            &mut env,
                            zenoh_flat::keyexpr_get_str(
                                zenoh_flat::query_get_keyexpr(&__cb_arg0),
                            ),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc0.into()
                    };
                    let __cb0_obj1: jni::objects::JObject = {
                        let __enc1 = match String_to_JString_c7f3ca43(
                            &mut env,
                            zenoh_flat::query_get_parameters(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc1.into()
                    };
                    let __cb0_obj3: jni::objects::JObject = match zenoh_flat::query_get_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc3 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::encoding_get_id(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc3) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj4: jni::objects::JObject = match zenoh_flat::query_get_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc4 = match Option_String_to_JString_56d5e304(
                                &mut env,
                                zenoh_flat::encoding_get_schema(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __enc4.into()
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj6: jni::sys::jvalue = {
                        let __enc6 = match ReplyKeyExpr_to_jint_0d9719f5(
                            &mut env,
                            zenoh_flat::query_get_accepts_replies(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc6 }
                    };
                    let __cb0_obj2: jni::objects::JObject = match zenoh_flat::query_get_payload(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h2: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
                                &mut env,
                                __n0,
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jlong(&mut env, __h2) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::query_get_attachment(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h5: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
                                &mut env,
                                __n0,
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jlong(&mut env, __h5) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj7: jni::sys::jvalue = jni::sys::jvalue {
                        j: std::boxed::Box::into_raw(std::boxed::Box::new(__cb_arg0))
                            as jni::sys::jlong,
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                jni::sys::jvalue {
                                    l: __cb0_obj0.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj1.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj2.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj3.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj4.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj5.as_raw(),
                                },
                                __cb0_obj6,
                                __cb0_obj7,
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Query)"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_Reply_Send_Sync_static_a5b82e2d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Reply) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Reply)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "([BIZLjava/lang/String;Ljava/lang/Long;Ljava/lang/Integer;Ljava/lang/String;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Boolean;Ljava/lang/Integer;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Integer;[BLjava/lang/Integer;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Integer;Ljava/lang/String;)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Reply)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Reply| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Reply)", e)))?;
                env.push_local_frame(46)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Reply)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::objects::JObject = {
                        let __enc0 = match Option_ZenohId_to_JByteArray_6880b2ba(
                            &mut env,
                            zenoh_flat::reply_get_replier_zid(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc0.into()
                    };
                    let __cb0_obj1: jni::sys::jvalue = {
                        let __enc1 = match i32_to_jint_a3e3b6ef(
                            &mut env,
                            zenoh_flat::reply_get_replier_eid(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc1 }
                    };
                    let __cb0_obj2: jni::sys::jvalue = {
                        let __enc2 = match bool_to_jboolean_31306d98(
                            &mut env,
                            zenoh_flat::reply_is_ok(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { z: __enc2 }
                    };
                    let __cb0_obj3: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc3 = match str_to_JString_7b77dc67(
                                &mut env,
                                zenoh_flat::keyexpr_get_str(
                                    zenoh_flat::sample_get_key_expr(__n0),
                                ),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __enc3.into()
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc5 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::encoding_get_id(
                                    zenoh_flat::sample_get_encoding(__n0),
                                ),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc5) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj6: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc6 = match Option_String_to_JString_56d5e304(
                                &mut env,
                                zenoh_flat::encoding_get_schema(
                                    zenoh_flat::sample_get_encoding(__n0),
                                ),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __enc6.into()
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj7: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc7 = match SampleKind_to_jint_d7ea75a8(
                                &mut env,
                                zenoh_flat::sample_get_kind(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc7) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj8: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            match zenoh_flat::sample_get_timestamp(__n0) {
                                ::core::option::Option::Some(__n1) => {
                                    let __enc8 = match i64_to_jlong_fbf9a9bc(
                                        &mut env,
                                        zenoh_flat::timestamp_get_ntp64(__n1),
                                    ) {
                                        ::core::result::Result::Ok(__w) => __w,
                                        ::core::result::Result::Err(__e) => {
                                            return ::core::result::Result::Err(
                                                <__JniErr as ::core::convert::From<
                                                    String,
                                                >>::from(__e.to_string()),
                                            );
                                        }
                                    };
                                    match ::prebindgen::lang::box_jlong(&mut env, __enc8) {
                                        ::core::result::Result::Ok(__o) => __o,
                                        ::core::result::Result::Err(__e) => {
                                            return ::core::result::Result::Err(
                                                <__JniErr as ::core::convert::From<String>>::from(__e),
                                            );
                                        }
                                    }
                                }
                                ::core::option::Option::None => {
                                    jni::objects::JObject::null()
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj9: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc9 = match bool_to_jboolean_31306d98(
                                &mut env,
                                zenoh_flat::sample_get_express(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jboolean(&mut env, __enc9) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj10: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc10 = match Priority_to_jint_447102d2(
                                &mut env,
                                zenoh_flat::sample_get_priority(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc10) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj11: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc11 = match CongestionControl_to_jint_62e38379(
                                &mut env,
                                zenoh_flat::sample_get_congestion_control(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc11) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj13: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc13 = match Reliability_to_jint_5d4a96c8(
                                &mut env,
                                zenoh_flat::sample_get_reliability(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc13) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj14: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc14 = match Option_ZenohId_to_JByteArray_6880b2ba(
                                &mut env,
                                zenoh_flat::sample_get_source_zid(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __enc14.into()
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj15: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc15 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::sample_get_source_eid(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc15) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj16: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc16 = match i64_to_jlong_fbf9a9bc(
                                &mut env,
                                zenoh_flat::sample_get_source_sn(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jlong(&mut env, __enc16) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj18: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc18 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::encoding_get_id(
                                    zenoh_flat::reply_error_get_encoding(__n0),
                                ),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jint(&mut env, __enc18) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj19: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc19 = match Option_String_to_JString_56d5e304(
                                &mut env,
                                zenoh_flat::encoding_get_schema(
                                    zenoh_flat::reply_error_get_encoding(__n0),
                                ),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __enc19.into()
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj4: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h4: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
                                &mut env,
                                zenoh_flat::sample_get_payload(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jlong(&mut env, __h4) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj12: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            match zenoh_flat::sample_get_attachment(__n0) {
                                ::core::option::Option::Some(__n1) => {
                                    let __h12: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
                                        &mut env,
                                        __n1,
                                    ) {
                                        ::core::result::Result::Ok(__w) => __w,
                                        ::core::result::Result::Err(__e) => {
                                            return ::core::result::Result::Err(
                                                <__JniErr as ::core::convert::From<
                                                    String,
                                                >>::from(__e.to_string()),
                                            );
                                        }
                                    };
                                    match ::prebindgen::lang::box_jlong(&mut env, __h12) {
                                        ::core::result::Result::Ok(__o) => __o,
                                        ::core::result::Result::Err(__e) => {
                                            return ::core::result::Result::Err(
                                                <__JniErr as ::core::convert::From<
                                                    String,
                                                >>::from(__e.to_string()),
                                            );
                                        }
                                    }
                                }
                                ::core::option::Option::None => {
                                    jni::objects::JObject::null()
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj17: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h17: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
                                &mut env,
                                zenoh_flat::reply_error_get_payload(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jlong(&mut env, __h17) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                jni::sys::jvalue {
                                    l: __cb0_obj0.as_raw(),
                                },
                                __cb0_obj1,
                                __cb0_obj2,
                                jni::sys::jvalue {
                                    l: __cb0_obj3.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj4.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj5.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj6.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj7.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj8.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj9.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj10.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj11.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj12.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj13.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj14.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj15.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj16.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj17.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj18.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj19.as_raw(),
                                },
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Reply)"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_Sample_Send_Sync_static_a050ca1d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Sample) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Sample)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(Ljava/lang/String;JILjava/lang/String;ILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Sample)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Sample| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Sample)", e)))?;
                env.push_local_frame(34)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Sample)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::objects::JObject = {
                        let __enc0 = match str_to_JString_7b77dc67(
                            &mut env,
                            zenoh_flat::keyexpr_get_str(
                                zenoh_flat::sample_get_key_expr(&__cb_arg0),
                            ),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc0.into()
                    };
                    let __cb0_obj2: jni::sys::jvalue = {
                        let __enc2 = match i32_to_jint_a3e3b6ef(
                            &mut env,
                            zenoh_flat::encoding_get_id(
                                zenoh_flat::sample_get_encoding(&__cb_arg0),
                            ),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc2 }
                    };
                    let __cb0_obj3: jni::objects::JObject = {
                        let __enc3 = match Option_String_to_JString_56d5e304(
                            &mut env,
                            zenoh_flat::encoding_get_schema(
                                zenoh_flat::sample_get_encoding(&__cb_arg0),
                            ),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc3.into()
                    };
                    let __cb0_obj4: jni::sys::jvalue = {
                        let __enc4 = match SampleKind_to_jint_d7ea75a8(
                            &mut env,
                            zenoh_flat::sample_get_kind(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc4 }
                    };
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::sample_get_timestamp(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc5 = match i64_to_jlong_fbf9a9bc(
                                &mut env,
                                zenoh_flat::timestamp_get_ntp64(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jlong(&mut env, __enc5) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj6: jni::sys::jvalue = {
                        let __enc6 = match bool_to_jboolean_31306d98(
                            &mut env,
                            zenoh_flat::sample_get_express(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { z: __enc6 }
                    };
                    let __cb0_obj7: jni::sys::jvalue = {
                        let __enc7 = match Priority_to_jint_447102d2(
                            &mut env,
                            zenoh_flat::sample_get_priority(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc7 }
                    };
                    let __cb0_obj8: jni::sys::jvalue = {
                        let __enc8 = match CongestionControl_to_jint_62e38379(
                            &mut env,
                            zenoh_flat::sample_get_congestion_control(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc8 }
                    };
                    let __cb0_obj10: jni::sys::jvalue = {
                        let __enc10 = match Reliability_to_jint_5d4a96c8(
                            &mut env,
                            zenoh_flat::sample_get_reliability(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc10 }
                    };
                    let __cb0_obj11: jni::objects::JObject = {
                        let __enc11 = match Option_ZenohId_to_JByteArray_6880b2ba(
                            &mut env,
                            zenoh_flat::sample_get_source_zid(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc11.into()
                    };
                    let __cb0_obj12: jni::sys::jvalue = {
                        let __enc12 = match i32_to_jint_a3e3b6ef(
                            &mut env,
                            zenoh_flat::sample_get_source_eid(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc12 }
                    };
                    let __cb0_obj13: jni::sys::jvalue = {
                        let __enc13 = match i64_to_jlong_fbf9a9bc(
                            &mut env,
                            zenoh_flat::sample_get_source_sn(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { j: __enc13 }
                    };
                    let __cb0_obj1: jni::sys::jvalue = {
                        let __h1: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
                            &mut env,
                            zenoh_flat::sample_get_payload(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { j: __h1 }
                    };
                    let __cb0_obj9: jni::objects::JObject = match zenoh_flat::sample_get_attachment(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h9: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
                                &mut env,
                                __n0,
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen::lang::box_jlong(&mut env, __h9) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                jni::sys::jvalue {
                                    l: __cb0_obj0.as_raw(),
                                },
                                __cb0_obj1,
                                __cb0_obj2,
                                jni::sys::jvalue {
                                    l: __cb0_obj3.as_raw(),
                                },
                                __cb0_obj4,
                                jni::sys::jvalue {
                                    l: __cb0_obj5.as_raw(),
                                },
                                __cb0_obj6,
                                __cb0_obj7,
                                __cb0_obj8,
                                jni::sys::jvalue {
                                    l: __cb0_obj9.as_raw(),
                                },
                                __cb0_obj10,
                                jni::sys::jvalue {
                                    l: __cb0_obj11.as_raw(),
                                },
                                __cb0_obj12,
                                __cb0_obj13,
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Sample)"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_Send_Sync_static_90cfb0b9<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<impl Fn() + Send + Sync + 'static, __JniErr> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to get callback class for {}: {}", "Fn()", e)))?;
        let __invoke_id = env
            .get_method_id(&__invoke_class, "run", "()V")
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn()", e)))?;
        Box::new(move || {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn()", e)))?;
                env.push_local_frame(16)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn()", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn()"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JString_to_Option_String_56d5e304<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JString<'v>,
) -> ::core::result::Result<Option<String>, __JniErr> {
    Ok({ if v.is_null() { None } else { Some(JString_to_String_c7f3ca43(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JString_to_String_c7f3ca43<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JString<'v>,
) -> ::core::result::Result<String, __JniErr> {
    Ok({
        let s = env
            .get_string(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("decode_string: {}", e))
            })?;
        s.into()
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn KeyExpr_to_jlong_57109ee0<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::KeyExpr,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn KeyExpr_to_jlong_5d6bcc5b<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::KeyExpr,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn LivelinessToken_to_jlong_d3477f0e<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::LivelinessToken,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_Encoding_to_jlong_e89ec09d<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::Encoding>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => Encoding_to_jlong_072adb3b(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ReplyError_to_jlong_d3e8c438<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::ReplyError>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => ReplyError_to_jlong_41e7bd88(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_Sample_to_jlong_e48d7024<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::Sample>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => Sample_to_jlong_26fb3fbd(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_String_to_JString_56d5e304<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<String>,
) -> ::core::result::Result<jni::objects::JString<'a>, __JniErr> {
    Ok({
        match v {
            Some(value) => String_to_JString_c7f3ca43(env, value)?,
            None => jni::objects::JObject::null().into(),
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_Timestamp_to_jlong_880c755c<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::Timestamp>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => Timestamp_to_jlong_a93920dc(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ZBytes_to_jlong_c521cd2f<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::ZBytes>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => ZBytes_to_jlong_56134c74(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ZenohId_to_JByteArray_6880b2ba<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<zenoh_flat::ZenohId>,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        match v {
            Some(value) => ZenohId_to_JByteArray_2caee6f1(env, value)?,
            None => jni::objects::JObject::null().into(),
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Priority_to_jint_447102d2<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Priority,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Publisher_to_jlong_7bfc8296<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Publisher,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Querier_to_jlong_9db85a56<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Querier,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn QueryTarget_to_jint_71d4db6a<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::QueryTarget,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Query_to_jlong_3af47090<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Query,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Queryable_to_jlong_f7f9bb6c<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Queryable,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Reliability_to_jint_5d4a96c8<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Reliability,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ReplyError_to_jlong_41e7bd88<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ReplyError,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ReplyError_to_jlong_9db9d1a6<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ReplyError,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ReplyKeyExpr_to_jint_0d9719f5<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ReplyKeyExpr,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Reply_to_jlong_8e506ce5<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Reply,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Config_Error_to_Config_745597c5<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::Config, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::Config, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Error_to_unit_1cf21a9d<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<(), zenoh_flat::Error>,
) -> ::core::result::Result<(), zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_KeyExpr_Error_to_KeyExpr_61def08d<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::KeyExpr, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::KeyExpr, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_LivelinessToken_Error_to_LivelinessToken_9f3adb18<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::LivelinessToken, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::LivelinessToken, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Publisher_Error_to_Publisher_f0c2f227<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::Publisher, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::Publisher, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Querier_Error_to_Querier_d4f296fb<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::Querier, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::Querier, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Queryable_Error_to_Queryable_841895c5<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::Queryable, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::Queryable, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Scout_Error_to_Scout_6400eee0<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::Scout, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::Scout, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Session_Error_to_Session_d603a635<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::Session, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::Session, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_String_Error_to_String_1b7d13da<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<String, zenoh_flat::Error>,
) -> ::core::result::Result<String, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_Subscriber_Error_to_Subscriber_f7ac5ca1<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::Subscriber, zenoh_flat::Error>,
) -> ::core::result::Result<zenoh_flat::Subscriber, zenoh_flat::Error> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn SampleKind_to_jint_d7ea75a8<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::SampleKind,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Sample_to_jlong_26fb3fbd<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::Sample,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Sample_to_jlong_f8134321<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Sample,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Scout_to_jlong_794eae84<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Scout,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Session_to_jlong_4d3982f6<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Session,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn SetIntersectionLevel_to_jint_0e49fc84<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::SetIntersectionLevel,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn String_to_JString_c7f3ca43<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: String,
) -> ::core::result::Result<jni::objects::JString<'a>, __JniErr> {
    Ok({
        env.new_string(v.as_str())
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_string: {}", e))
            })?
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Subscriber_to_jlong_73e1b4a2<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Subscriber,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Timestamp_to_jlong_a93920dc<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::Timestamp,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Timestamp_to_jlong_bfea2165<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Timestamp,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Vec_String_to_JObject_1e282499<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Vec<String>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __list_obj = env
            .new_object("java/util/ArrayList", "()V", &[])
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Vec<_>: new ArrayList: {}", e)))?;
        let __list = jni::objects::JList::from_env(env, &__list_obj)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Vec<_>: list-from-env: {}", e)))?;
        for __elem in v.into_iter() {
            let __elem_wire = String_to_JString_c7f3ca43(env, __elem)?;
            let __elem_obj: jni::objects::JObject = __elem_wire.into();
            __list
                .add(env, &__elem_obj)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Vec<_>: list-add: {}", e)))?;
        }
        __list_obj
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Vec_ZenohId_to_JObject_cd7f8e6c<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Vec<zenoh_flat::ZenohId>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __list_obj = env
            .new_object("java/util/ArrayList", "()V", &[])
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Vec<_>: new ArrayList: {}", e)))?;
        let __list = jni::objects::JList::from_env(env, &__list_obj)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Vec<_>: list-from-env: {}", e)))?;
        for __elem in v.into_iter() {
            let __elem_wire = ZenohId_to_JByteArray_2caee6f1(env, __elem)?;
            let __elem_obj: jni::objects::JObject = __elem_wire.into();
            __list
                .add(env, &__elem_obj)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Vec<_>: list-add: {}", e)))?;
        }
        __list_obj
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Vec_u8_to_JByteArray_7936d5de<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Vec<u8>,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        env.byte_array_from_slice(v.as_slice())
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_byte_array: {}", e))
            })?
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn WhatAmI_to_jint_4c5d5738<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::WhatAmI,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZBytes_to_jlong_141dc9e1<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZBytes,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZBytes_to_jlong_56134c74<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZBytes,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZenohId_to_JByteArray_2caee6f1<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZenohId,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        let __bytes: &[u8] = unsafe {
            ::core::slice::from_raw_parts(
                (&v as *const zenoh_flat::ZenohId) as *const u8,
                ::core::mem::size_of::<zenoh_flat::ZenohId>(),
            )
        };
        env.byte_array_from_slice(__bytes)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("value-blob encode: {}", e))
            })?
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn bool_to_jboolean_31306d98<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: bool,
) -> ::core::result::Result<jni::sys::jboolean, __JniErr> {
    Ok(v as jni::sys::jboolean)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn i32_to_jint_a3e3b6ef<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: i32,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok(v as jni::sys::jint)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn i64_to_jlong_fbf9a9bc<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: i64,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(v as jni::sys::jlong)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jboolean_to_bool_31306d98<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jboolean,
) -> ::core::result::Result<bool, __JniErr> {
    Ok(*v != 0)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_CongestionControl_62e38379<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::CongestionControl, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::CongestionControl::Drop,
            1 => zenoh_flat::CongestionControl::Block,
            2 => zenoh_flat::CongestionControl::BlockFirst,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "invalid {} discriminant: {}", "CongestionControl", other
                        ),
                    ),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_ConsolidationMode_dd4eaedc<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::ConsolidationMode, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::ConsolidationMode::Auto,
            1 => zenoh_flat::ConsolidationMode::None,
            2 => zenoh_flat::ConsolidationMode::Monotonic,
            3 => zenoh_flat::ConsolidationMode::Latest,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "invalid {} discriminant: {}", "ConsolidationMode", other
                        ),
                    ),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_Priority_447102d2<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::Priority, __JniErr> {
    Ok({
        match *v as i64 {
            1 => zenoh_flat::Priority::RealTime,
            2 => zenoh_flat::Priority::InteractiveHigh,
            3 => zenoh_flat::Priority::InteractiveLow,
            4 => zenoh_flat::Priority::DataHigh,
            5 => zenoh_flat::Priority::Data,
            6 => zenoh_flat::Priority::DataLow,
            7 => zenoh_flat::Priority::Background,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("invalid {} discriminant: {}", "Priority", other)),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_QueryTarget_71d4db6a<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::QueryTarget, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::QueryTarget::BestMatching,
            1 => zenoh_flat::QueryTarget::All,
            2 => zenoh_flat::QueryTarget::AllComplete,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!("invalid {} discriminant: {}", "QueryTarget", other),
                    ),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_Reliability_5d4a96c8<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::Reliability, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::Reliability::BestEffort,
            1 => zenoh_flat::Reliability::Reliable,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!("invalid {} discriminant: {}", "Reliability", other),
                    ),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_ReplyKeyExpr_0d9719f5<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::ReplyKeyExpr, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::ReplyKeyExpr::Any,
            1 => zenoh_flat::ReplyKeyExpr::MatchingQuery,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!("invalid {} discriminant: {}", "ReplyKeyExpr", other),
                    ),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_SampleKind_d7ea75a8<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::SampleKind, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::SampleKind::Put,
            1 => zenoh_flat::SampleKind::Delete,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("invalid {} discriminant: {}", "SampleKind", other)),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_SetIntersectionLevel_0e49fc84<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::SetIntersectionLevel, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::SetIntersectionLevel::Disjoint,
            1 => zenoh_flat::SetIntersectionLevel::Intersects,
            2 => zenoh_flat::SetIntersectionLevel::Includes,
            3 => zenoh_flat::SetIntersectionLevel::Equals,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "invalid {} discriminant: {}", "SetIntersectionLevel", other
                        ),
                    ),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_WhatAmI_4c5d5738<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::WhatAmI, __JniErr> {
    Ok({
        match *v as i64 {
            1 => zenoh_flat::WhatAmI::Router,
            2 => zenoh_flat::WhatAmI::Peer,
            4 => zenoh_flat::WhatAmI::Client,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("invalid {} discriminant: {}", "WhatAmI", other)),
                );
            }
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jint_to_i32_a3e3b6ef<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<i32, __JniErr> {
    Ok(*v)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Config_d1f60c7d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Config>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Config) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Encoding_e0e31e0d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Encoding>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Encoding) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Hello_bbd3fc65<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Hello>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Hello) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_KeyExpr_5d6bcc5b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::KeyExpr>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::KeyExpr) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_LivelinessToken_d3477f0e<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::LivelinessToken>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::LivelinessToken) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_Config_61908788<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::Config>>, __JniErr> {
    Ok({ if *v == 0 { None } else { Some(jlong_to_Config_d1f60c7d(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_Encoding_e89ec09d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::Encoding>>, __JniErr> {
    Ok({ if *v == 0 { None } else { Some(jlong_to_Encoding_e0e31e0d(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_KeyExpr_d960fa7d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::KeyExpr>>, __JniErr> {
    Ok({ if *v == 0 { None } else { Some(jlong_to_KeyExpr_5d6bcc5b(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_KeyExpr_f7eec5be<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<zenoh_flat::KeyExpr>, __JniErr> {
    Ok({
        if *v == 0 {
            None
        } else if (*v & 1) == 1 {
            return ::core::result::Result::Err(
                <__JniErr as ::core::convert::From<
                    String,
                >>::from("Operation on a closed native handle.".to_string()),
            );
        } else {
            Some(*std::boxed::Box::from_raw(*v as *mut zenoh_flat::KeyExpr))
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_ZBytes_e82c3945<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<zenoh_flat::ZBytes>, __JniErr> {
    Ok({
        if *v == 0 {
            None
        } else if (*v & 1) == 1 {
            return ::core::result::Result::Err(
                <__JniErr as ::core::convert::From<
                    String,
                >>::from("Operation on a closed native handle.".to_string()),
            );
        } else {
            Some(*std::boxed::Box::from_raw(*v as *mut zenoh_flat::ZBytes))
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Publisher_7bfc8296<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Publisher>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Publisher) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Querier_9db85a56<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Querier>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Querier) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Query_3af47090<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Query>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Query) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Queryable_f7f9bb6c<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Queryable>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Queryable) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ReplyError_9db9d1a6<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ReplyError>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ReplyError) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Reply_8e506ce5<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Reply>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Reply) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Sample_f8134321<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Sample>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Sample) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Scout_794eae84<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Scout>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Scout) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Session_4d3982f6<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Session>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Session) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Subscriber_73e1b4a2<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Subscriber>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Subscriber) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Timestamp_bfea2165<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Timestamp>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Timestamp) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZBytes_141dc9e1<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZBytes>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZBytes) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_i64_fbf9a9bc<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<i64, __JniErr> {
    Ok(*v)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn std_borrow_Cow_u8_to_JByteArray_c6a6bddf<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: ::std::borrow::Cow<'_, [u8]>,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        env.byte_array_from_slice(&v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_byte_array: {}", e))
            })?
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn str_to_JString_7b77dc67<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &str,
) -> ::core::result::Result<jni::objects::JString<'a>, __JniErr> {
    Ok({
        env.new_string(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_str: {}", e))
            })?
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn unit_to_unit_9ecccf8e<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: (),
) -> ::core::result::Result<(), __JniErr> {
    Ok(v)
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configGetJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    c: jni::sys::jlong,
    key: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let c = match jlong_to_Config_d1f60c7d(&mut env, &c) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let key = match JString_to_String_c7f3ca43(&mut env, &key) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = match zenoh_flat::config_get_json(&c, &key) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return jni::objects::JObject::null().into();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return jni::objects::JObject::null().into();
        }
    };
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configInsertJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    c: jni::sys::jlong,
    key: jni::objects::JString<'a>,
    value: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let mut c = match jlong_to_Config_d1f60c7d(&mut env, &c) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let key = match JString_to_String_c7f3ca43(&mut env, &key) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let value = match JString_to_String_c7f3ca43(&mut env, &value) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::config_insert_json5(&mut c, &key, &value) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    c: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let c = match jlong_to_Config_d1f60c7d(&mut env, &c) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::config_new_clone(&c);
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewDefault<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::config_new_default();
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewFromFile<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    path: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let path = match JString_to_String_c7f3ca43(&mut env, &path) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::config_new_from_file(&path) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewFromJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::config_new_from_json(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewFromJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::config_new_from_json5(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewFromYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::config_new_from_yaml(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingGetId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::encoding_get_id(&e);
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingGetSchema<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::encoding_get_schema(&e);
    match Option_String_to_JString_56d5e304(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_clone(&e);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewFromId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    id: jni::sys::jint,
    schema: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let id = match jint_to_i32_a3e3b6ef(&mut env, &id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let schema = match JString_to_Option_String_56d5e304(&mut env, &schema) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_from_id(id, schema);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewFromString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_from_string(s);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewWithSchema<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e_sel: jni::sys::jint,
    e_0_0_present: jni::sys::jboolean,
    e_0_0_value: jni::sys::jint,
    e_0_1: jni::objects::JString<'a>,
    e_1: jni::sys::jlong,
    schema: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_e_sel = match jint_to_i32_a3e3b6ef(&mut env, &e_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_e_0_0: Option<i32> = if e_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &e_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_e_0_1 = match JString_to_Option_String_56d5e304(&mut env, &e_0_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_e_1 = match jlong_to_Option_Encoding_e89ec09d(&mut env, &e_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_e = match {
        match __exp_e_sel {
            0i32 => {
                match __exp_e_0_0 {
                    ::core::option::Option::Some(__p0) => {
                        ::core::result::Result::Ok(
                            zenoh_flat::encoding_new_from_id(__p0, __exp_e_0_1),
                        )
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_e_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let schema = match JString_to_String_c7f3ca43(&mut env, &schema) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_with_schema(&__folded_e, schema);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingToString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::encoding_to_string(&e);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_helloGetLocators<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    h: jni::sys::jlong,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match jlong_to_Hello_bbd3fc65(&mut env, &h) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/StringFolder";
    const __CB_DESCR: &str = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;";
    let __vec = zenoh_flat::hello_get_locators(&h);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let __enc = match String_to_JString_c7f3ca43(&mut env, __elem) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj: jni::objects::JObject = __enc.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_helloGetWhatami<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    h: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match jlong_to_Hello_bbd3fc65(&mut env, &h) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::hello_get_whatami(&h);
    match WhatAmI_to_jint_4c5d5738(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_helloGetZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    h: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match jlong_to_Hello_bbd3fc65(&mut env, &h) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::hello_get_zid(&h);
    match ZenohId_to_JByteArray_2caee6f1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_initAndroidLogs<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    filter: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let filter = match JString_to_String_c7f3ca43(&mut env, &filter) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = zenoh_flat::init_android_logs(&filter);
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_initZenohLogsFromEnvOr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    fallback_filter: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let fallback_filter = match JString_to_String_c7f3ca43(&mut env, &fallback_filter) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = zenoh_flat::init_zenoh_logs_from_env_or(&fallback_filter);
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprGetStr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    ke: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &ke) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::keyexpr_get_str(&ke);
    match str_to_JString_7b77dc67(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprIncludes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a: jni::sys::jlong,
    b_sel: jni::sys::jint,
    b_0: jni::objects::JString<'a>,
    b_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &a) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_sel = match jint_to_i32_a3e3b6ef(&mut env, &b_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_0 = match JString_to_Option_String_56d5e304(&mut env, &b_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &b_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __folded_b = match {
        match __exp_b_sel {
            0i32 => {
                match __exp_b_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_b_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::keyexpr_includes(&a, &__folded_b);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprIntersects<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a: jni::sys::jlong,
    b_sel: jni::sys::jint,
    b_0: jni::objects::JString<'a>,
    b_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &a) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_sel = match jint_to_i32_a3e3b6ef(&mut env, &b_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_0 = match JString_to_Option_String_56d5e304(&mut env, &b_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &b_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __folded_b = match {
        match __exp_b_sel {
            0i32 => {
                match __exp_b_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_b_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::keyexpr_intersects(&a, &__folded_b);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewAutocanonize<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_autocanonize(s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match KeyExpr_to_jlong_5d6bcc5b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    ke: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &ke) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::keyexpr_new_clone(&ke);
    match KeyExpr_to_jlong_5d6bcc5b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewConcat<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a_sel: jni::sys::jint,
    a_0: jni::objects::JString<'a>,
    a_1: jni::sys::jlong,
    b: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_a_sel = match jint_to_i32_a3e3b6ef(&mut env, &a_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_0 = match JString_to_Option_String_56d5e304(&mut env, &a_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &a_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_a = match {
        match __exp_a_sel {
            0i32 => {
                match __exp_a_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_a_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let b = match JString_to_String_c7f3ca43(&mut env, &b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_concat(&__folded_a, b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match KeyExpr_to_jlong_5d6bcc5b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewJoin<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a_sel: jni::sys::jint,
    a_0: jni::objects::JString<'a>,
    a_1: jni::sys::jlong,
    b: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_a_sel = match jint_to_i32_a3e3b6ef(&mut env, &a_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_0 = match JString_to_Option_String_56d5e304(&mut env, &a_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &a_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_a = match {
        match __exp_a_sel {
            0i32 => {
                match __exp_a_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_a_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let b = match JString_to_String_c7f3ca43(&mut env, &b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_join(&__folded_a, b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match KeyExpr_to_jlong_5d6bcc5b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewTryFrom<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_try_from(s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match KeyExpr_to_jlong_5d6bcc5b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprRelationTo<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a: jni::sys::jlong,
    b_sel: jni::sys::jint,
    b_0: jni::objects::JString<'a>,
    b_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &a) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_sel = match jint_to_i32_a3e3b6ef(&mut env, &b_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_0 = match JString_to_Option_String_56d5e304(&mut env, &b_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &b_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __folded_b = match {
        match __exp_b_sel {
            0i32 => {
                match __exp_b_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_b_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::keyexpr_relation_to(&a, &__folded_b);
    match SetIntersectionLevel_to_jint_0e49fc84(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprToString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    ke: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &ke) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::keyexpr_to_string(&ke);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_livelinessDeclareSubscriber<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    history: jni::sys::jboolean,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let history = match jboolean_to_bool_31306d98(&mut env, &history) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match JObject_to_impl_Fn_Sample_Send_Sync_static_a050ca1d(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match JObject_to_impl_Fn_Send_Sync_static_90cfb0b9(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::liveliness_declare_subscriber(
        &session,
        __folded_key_expr,
        history,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Subscriber_to_jlong_73e1b4a2(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_livelinessDeclareToken<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::liveliness_declare_token(&session, __folded_key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match LivelinessToken_to_jlong_d3477f0e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_livelinessGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    timeout_ms: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_d960fa7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let timeout_ms = match jlong_to_i64_fbf9a9bc(&mut env, &timeout_ms) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let callback = match JObject_to_impl_Fn_Reply_Send_Sync_static_a5b82e2d(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match JObject_to_impl_Fn_Send_Sync_static_90cfb0b9(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::liveliness_get(
        &session,
        &__folded_key_expr,
        timeout_ms,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_open<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    config: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    if config == 0 || (config & 1) == 1 {
        signal_binding_error(
            &mut env,
            &__error_sink,
            &__SINK_MID,
            __SINK_FQN,
            __SINK_DESCR,
            "Operation on a closed native handle.",
        );
        return 0 as jni::sys::jlong;
    }
    let config: zenoh_flat::Config = unsafe {
        *std::boxed::Box::from_raw(config as *mut zenoh_flat::Config)
    };
    let __out = match zenoh_flat::open(config) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Session_to_jlong_4d3982f6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersContainsKey<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let k = match JString_to_String_c7f3ca43(&mut env, &k) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::parameters_contains_key(&s, &k);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersExtend<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    other: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let other = match JString_to_String_c7f3ca43(&mut env, &other) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_extend(&s, &other);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match JString_to_String_c7f3ca43(&mut env, &k) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_get(&s, &k);
    match Option_String_to_JString_56d5e304(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersInsert<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    v: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match JString_to_String_c7f3ca43(&mut env, &k) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let v = match JString_to_String_c7f3ca43(&mut env, &v) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_insert(&s, &k, &v);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersIsWellFormed<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::parameters_is_well_formed(&s);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersRemove<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match JString_to_String_c7f3ca43(&mut env, &k) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_remove(&s, &k);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersValues<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match JString_to_String_c7f3ca43(&mut env, &k) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/StringFolder";
    const __CB_DESCR: &str = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;";
    let __vec = zenoh_flat::parameters_values(&s, &k);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let __enc = match String_to_JString_c7f3ca43(&mut env, __elem) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj: jni::objects::JObject = __enc.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_publisherDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match jlong_to_Publisher_7bfc8296(&mut env, &publisher) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::publisher_delete(&publisher, __folded_attachment) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_publisherPut<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match jlong_to_Publisher_7bfc8296(&mut env, &publisher) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::publisher_put(
        &publisher,
        __folded_payload,
        __folded_encoding.as_ref(),
        __folded_attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_querierGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    querier: jni::sys::jlong,
    parameters: jni::objects::JString<'a>,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let querier = match jlong_to_Querier_9db85a56(&mut env, &querier) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let parameters = match JString_to_Option_String_56d5e304(&mut env, &parameters) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Option_Vec_u8_6f4428ab(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match match __exp_payload {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let callback = match JObject_to_impl_Fn_Reply_Send_Sync_static_a5b82e2d(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match JObject_to_impl_Fn_Send_Sync_static_90cfb0b9(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::querier_get(
        &querier,
        parameters,
        __folded_payload,
        __folded_encoding.as_ref(),
        __folded_attachment,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetAcceptsReplies<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::query_get_accepts_replies(&q);
    match ReplyKeyExpr_to_jint_0d9719f5(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetAttachment<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_attachment(&q);
    match Option_ZBytes_to_jlong_c521cd2f(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetEncoding<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_encoding(&q);
    match Option_Encoding_to_jlong_e89ec09d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetKeyexpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_keyexpr(&q);
    match KeyExpr_to_jlong_57109ee0(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetParameters<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::query_get_parameters(&q);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetPayload<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_payload(&q);
    match Option_ZBytes_to_jlong_c521cd2f(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplyDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    timestamp_ntp64_present: jni::sys::jboolean,
    timestamp_ntp64_value: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_d960fa7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let timestamp_ntp64 = if timestamp_ntp64_present != 0u8 {
        let __timestamp_ntp64_val = match jlong_to_i64_fbf9a9bc(
            &mut env,
            &timestamp_ntp64_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__timestamp_ntp64_val)
    } else {
        ::core::option::Option::None
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let __out = match zenoh_flat::query_reply_delete(
        &query,
        &__folded_key_expr,
        timestamp_ntp64,
        __folded_attachment,
        express,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplyError<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::query_reply_error(
        &query,
        __folded_payload,
        __folded_encoding.as_ref(),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplySample<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    sample: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    if sample == 0 || (sample & 1) == 1 {
        signal_binding_error(
            &mut env,
            &__error_sink,
            &__SINK_MID,
            __SINK_FQN,
            __SINK_DESCR,
            "Operation on a closed native handle.",
        );
        return ();
    }
    let sample: zenoh_flat::Sample = unsafe {
        *std::boxed::Box::from_raw(sample as *mut zenoh_flat::Sample)
    };
    let __out = match zenoh_flat::query_reply_sample(&query, sample) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplySuccess<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    timestamp_ntp64_present: jni::sys::jboolean,
    timestamp_ntp64_value: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_d960fa7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let timestamp_ntp64 = if timestamp_ntp64_present != 0u8 {
        let __timestamp_ntp64_val = match jlong_to_i64_fbf9a9bc(
            &mut env,
            &timestamp_ntp64_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__timestamp_ntp64_val)
    } else {
        ::core::option::Option::None
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let __out = match zenoh_flat::query_reply_success(
        &query,
        &__folded_key_expr,
        __folded_payload,
        __folded_encoding.as_ref(),
        timestamp_ntp64,
        __folded_attachment,
        express,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyErrorGetEncoding<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_ReplyError_9db9d1a6(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_error_get_encoding(&e);
    match Encoding_to_jlong_072adb3b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyErrorGetPayload<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_ReplyError_9db9d1a6(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_error_get_payload(&e);
    match ZBytes_to_jlong_56134c74(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyGetErr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_get_err(&r);
    match Option_ReplyError_to_jlong_d3e8c438(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyGetReplierEid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::reply_get_replier_eid(&r);
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyGetReplierZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::reply_get_replier_zid(&r);
    match Option_ZenohId_to_JByteArray_6880b2ba(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyGetSample<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_get_sample(&r);
    match Option_Sample_to_jlong_e48d7024(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyIsOk<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::reply_is_ok(&r);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetAttachment<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_attachment(&s);
    match Option_ZBytes_to_jlong_c521cd2f(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetCongestionControl<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_congestion_control(&s);
    match CongestionControl_to_jint_62e38379(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetEncoding<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_encoding(&s);
    match Encoding_to_jlong_072adb3b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetExpress<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::sample_get_express(&s);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetKeyExpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_key_expr(&s);
    match KeyExpr_to_jlong_57109ee0(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetKind<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_kind(&s);
    match SampleKind_to_jint_d7ea75a8(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetPayload<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_payload(&s);
    match ZBytes_to_jlong_56134c74(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetPriority<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_priority(&s);
    match Priority_to_jint_447102d2(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetReliability<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_reliability(&s);
    match Reliability_to_jint_5d4a96c8(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetSourceEid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_source_eid(&s);
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetSourceSn<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_source_sn(&s);
    match i64_to_jlong_fbf9a9bc(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetSourceZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::sample_get_source_zid(&s);
    match Option_ZenohId_to_JByteArray_6880b2ba(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetTimestamp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_timestamp(&s);
    match Option_Timestamp_to_jlong_880c755c(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleNewDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    timestamp_ntp64_present: jni::sys::jboolean,
    timestamp_ntp64_value: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    congestion_control_present: jni::sys::jboolean,
    congestion_control_value: jni::sys::jint,
    priority_present: jni::sys::jboolean,
    priority_value: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    reliability_present: jni::sys::jboolean,
    reliability_value: jni::sys::jint,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let timestamp_ntp64 = if timestamp_ntp64_present != 0u8 {
        let __timestamp_ntp64_val = match jlong_to_i64_fbf9a9bc(
            &mut env,
            &timestamp_ntp64_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__timestamp_ntp64_val)
    } else {
        ::core::option::Option::None
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let congestion_control = if congestion_control_present != 0u8 {
        let __congestion_control_val = match jint_to_CongestionControl_62e38379(
            &mut env,
            &congestion_control_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__congestion_control_val)
    } else {
        ::core::option::Option::None
    };
    let priority = if priority_present != 0u8 {
        let __priority_val = match jint_to_Priority_447102d2(&mut env, &priority_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__priority_val)
    } else {
        ::core::option::Option::None
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let reliability = if reliability_present != 0u8 {
        let __reliability_val = match jint_to_Reliability_5d4a96c8(
            &mut env,
            &reliability_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__reliability_val)
    } else {
        ::core::option::Option::None
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/sample/SampleBuilderRaw";
    const __CB_DESCR: &str = "(Ljava/lang/String;JILjava/lang/String;ILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)Ljava/lang/Object;";
    let __out = zenoh_flat::sample_new_delete(
        __folded_key_expr,
        timestamp_ntp64,
        __folded_attachment,
        congestion_control,
        priority,
        express,
        reliability,
    );
    let __obj0: jni::objects::JObject = {
        let __enc0 = match str_to_JString_7b77dc67(
            &mut env,
            zenoh_flat::keyexpr_get_str(zenoh_flat::sample_get_key_expr(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        __enc0.into()
    };
    let __obj2: jni::sys::jvalue = {
        let __enc2 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::encoding_get_id(zenoh_flat::sample_get_encoding(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc2 }
    };
    let __obj3: jni::objects::JObject = {
        let __enc3 = match Option_String_to_JString_56d5e304(
            &mut env,
            zenoh_flat::encoding_get_schema(zenoh_flat::sample_get_encoding(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        __enc3.into()
    };
    let __obj4: jni::sys::jvalue = {
        let __enc4 = match SampleKind_to_jint_d7ea75a8(
            &mut env,
            zenoh_flat::sample_get_kind(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc4 }
    };
    let __obj5: jni::objects::JObject = match zenoh_flat::sample_get_timestamp(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __enc5 = match i64_to_jlong_fbf9a9bc(
                &mut env,
                zenoh_flat::timestamp_get_ntp64(__n0),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e.to_string(),
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __enc5) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e,
                    );
                    return jni::objects::JObject::null().into();
                }
            }
        }
        ::core::option::Option::None => jni::objects::JObject::null(),
    };
    let __obj6: jni::sys::jvalue = {
        let __enc6 = match bool_to_jboolean_31306d98(
            &mut env,
            zenoh_flat::sample_get_express(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { z: __enc6 }
    };
    let __obj7: jni::sys::jvalue = {
        let __enc7 = match Priority_to_jint_447102d2(
            &mut env,
            zenoh_flat::sample_get_priority(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc7 }
    };
    let __obj8: jni::sys::jvalue = {
        let __enc8 = match CongestionControl_to_jint_62e38379(
            &mut env,
            zenoh_flat::sample_get_congestion_control(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc8 }
    };
    let __obj10: jni::sys::jvalue = {
        let __enc10 = match Reliability_to_jint_5d4a96c8(
            &mut env,
            zenoh_flat::sample_get_reliability(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc10 }
    };
    let __obj11: jni::objects::JObject = {
        let __enc11 = match Option_ZenohId_to_JByteArray_6880b2ba(
            &mut env,
            zenoh_flat::sample_get_source_zid(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        __enc11.into()
    };
    let __obj12: jni::sys::jvalue = {
        let __enc12 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::sample_get_source_eid(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc12 }
    };
    let __obj13: jni::sys::jvalue = {
        let __enc13 = match i64_to_jlong_fbf9a9bc(
            &mut env,
            zenoh_flat::sample_get_source_sn(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __enc13 }
    };
    let __obj1: jni::sys::jvalue = {
        let __h1: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
            &mut env,
            zenoh_flat::sample_get_payload(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h1 }
    };
    let __obj9: jni::objects::JObject = match zenoh_flat::sample_get_attachment(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __h9: jni::sys::jlong = match ZBytes_to_jlong_56134c74(&mut env, __n0) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e.to_string(),
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __h9) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e.to_string(),
                    );
                    return jni::objects::JObject::null().into();
                }
            }
        }
        ::core::option::Option::None => jni::objects::JObject::null(),
    };
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[
                jni::sys::jvalue {
                    l: __obj0.as_raw(),
                },
                __obj1,
                __obj2,
                jni::sys::jvalue {
                    l: __obj3.as_raw(),
                },
                __obj4,
                jni::sys::jvalue {
                    l: __obj5.as_raw(),
                },
                __obj6,
                __obj7,
                __obj8,
                jni::sys::jvalue {
                    l: __obj9.as_raw(),
                },
                __obj10,
                jni::sys::jvalue {
                    l: __obj11.as_raw(),
                },
                __obj12,
                __obj13,
            ],
        )
    {
        ::core::result::Result::Ok(__o) => __o,
        ::core::result::Result::Err(__e) => {
            let _ = env.exception_describe();
            let __e2 = <__JniErr as ::core::convert::From<
                String,
            >>::from(__e.to_string());
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e2.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleNewPut<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    timestamp_ntp64_present: jni::sys::jboolean,
    timestamp_ntp64_value: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    congestion_control_present: jni::sys::jboolean,
    congestion_control_value: jni::sys::jint,
    priority_present: jni::sys::jboolean,
    priority_value: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    reliability_present: jni::sys::jboolean,
    reliability_value: jni::sys::jint,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let timestamp_ntp64 = if timestamp_ntp64_present != 0u8 {
        let __timestamp_ntp64_val = match jlong_to_i64_fbf9a9bc(
            &mut env,
            &timestamp_ntp64_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__timestamp_ntp64_val)
    } else {
        ::core::option::Option::None
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let congestion_control = if congestion_control_present != 0u8 {
        let __congestion_control_val = match jint_to_CongestionControl_62e38379(
            &mut env,
            &congestion_control_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__congestion_control_val)
    } else {
        ::core::option::Option::None
    };
    let priority = if priority_present != 0u8 {
        let __priority_val = match jint_to_Priority_447102d2(&mut env, &priority_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__priority_val)
    } else {
        ::core::option::Option::None
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let reliability = if reliability_present != 0u8 {
        let __reliability_val = match jint_to_Reliability_5d4a96c8(
            &mut env,
            &reliability_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        ::core::option::Option::Some(__reliability_val)
    } else {
        ::core::option::Option::None
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/sample/SampleBuilderRaw";
    const __CB_DESCR: &str = "(Ljava/lang/String;JILjava/lang/String;ILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)Ljava/lang/Object;";
    let __out = zenoh_flat::sample_new_put(
        __folded_key_expr,
        __folded_payload,
        __folded_encoding.as_ref(),
        timestamp_ntp64,
        __folded_attachment,
        congestion_control,
        priority,
        express,
        reliability,
    );
    let __obj0: jni::objects::JObject = {
        let __enc0 = match str_to_JString_7b77dc67(
            &mut env,
            zenoh_flat::keyexpr_get_str(zenoh_flat::sample_get_key_expr(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        __enc0.into()
    };
    let __obj2: jni::sys::jvalue = {
        let __enc2 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::encoding_get_id(zenoh_flat::sample_get_encoding(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc2 }
    };
    let __obj3: jni::objects::JObject = {
        let __enc3 = match Option_String_to_JString_56d5e304(
            &mut env,
            zenoh_flat::encoding_get_schema(zenoh_flat::sample_get_encoding(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        __enc3.into()
    };
    let __obj4: jni::sys::jvalue = {
        let __enc4 = match SampleKind_to_jint_d7ea75a8(
            &mut env,
            zenoh_flat::sample_get_kind(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc4 }
    };
    let __obj5: jni::objects::JObject = match zenoh_flat::sample_get_timestamp(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __enc5 = match i64_to_jlong_fbf9a9bc(
                &mut env,
                zenoh_flat::timestamp_get_ntp64(__n0),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e.to_string(),
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __enc5) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e,
                    );
                    return jni::objects::JObject::null().into();
                }
            }
        }
        ::core::option::Option::None => jni::objects::JObject::null(),
    };
    let __obj6: jni::sys::jvalue = {
        let __enc6 = match bool_to_jboolean_31306d98(
            &mut env,
            zenoh_flat::sample_get_express(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { z: __enc6 }
    };
    let __obj7: jni::sys::jvalue = {
        let __enc7 = match Priority_to_jint_447102d2(
            &mut env,
            zenoh_flat::sample_get_priority(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc7 }
    };
    let __obj8: jni::sys::jvalue = {
        let __enc8 = match CongestionControl_to_jint_62e38379(
            &mut env,
            zenoh_flat::sample_get_congestion_control(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc8 }
    };
    let __obj10: jni::sys::jvalue = {
        let __enc10 = match Reliability_to_jint_5d4a96c8(
            &mut env,
            zenoh_flat::sample_get_reliability(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc10 }
    };
    let __obj11: jni::objects::JObject = {
        let __enc11 = match Option_ZenohId_to_JByteArray_6880b2ba(
            &mut env,
            zenoh_flat::sample_get_source_zid(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        __enc11.into()
    };
    let __obj12: jni::sys::jvalue = {
        let __enc12 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::sample_get_source_eid(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc12 }
    };
    let __obj13: jni::sys::jvalue = {
        let __enc13 = match i64_to_jlong_fbf9a9bc(
            &mut env,
            zenoh_flat::sample_get_source_sn(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __enc13 }
    };
    let __obj1: jni::sys::jvalue = {
        let __h1: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
            &mut env,
            zenoh_flat::sample_get_payload(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h1 }
    };
    let __obj9: jni::objects::JObject = match zenoh_flat::sample_get_attachment(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __h9: jni::sys::jlong = match ZBytes_to_jlong_56134c74(&mut env, __n0) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e.to_string(),
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __h9) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    signal_binding_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        &__e.to_string(),
                    );
                    return jni::objects::JObject::null().into();
                }
            }
        }
        ::core::option::Option::None => jni::objects::JObject::null(),
    };
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[
                jni::sys::jvalue {
                    l: __obj0.as_raw(),
                },
                __obj1,
                __obj2,
                jni::sys::jvalue {
                    l: __obj3.as_raw(),
                },
                __obj4,
                jni::sys::jvalue {
                    l: __obj5.as_raw(),
                },
                __obj6,
                __obj7,
                __obj8,
                jni::sys::jvalue {
                    l: __obj9.as_raw(),
                },
                __obj10,
                jni::sys::jvalue {
                    l: __obj11.as_raw(),
                },
                __obj12,
                __obj13,
            ],
        )
    {
        ::core::result::Result::Ok(__o) => __o,
        ::core::result::Result::Err(__e) => {
            let _ = env.exception_describe();
            let __e2 = <__JniErr as ::core::convert::From<
                String,
            >>::from(__e.to_string());
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e2.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_scout<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    whatami: jni::sys::jint,
    config: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let whatami = match jint_to_i32_a3e3b6ef(&mut env, &whatami) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let config = match jlong_to_Option_Config_61908788(&mut env, &config) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match JObject_to_impl_Fn_Hello_Send_Sync_static_d937ec1a(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match JObject_to_impl_Fn_Send_Sync_static_90cfb0b9(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::scout(whatami, config.as_deref(), callback, on_close) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Scout_to_jlong_794eae84(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareKeyexpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let key_expr = match JString_to_String_c7f3ca43(&mut env, &key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_keyexpr(&session, key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match KeyExpr_to_jlong_5d6bcc5b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclarePublisher<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    congestion_control_present: jni::sys::jboolean,
    congestion_control_value: jni::sys::jint,
    priority_present: jni::sys::jboolean,
    priority_value: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    reliability_present: jni::sys::jboolean,
    reliability_value: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = if congestion_control_present != 0u8 {
        let __congestion_control_val = match jint_to_CongestionControl_62e38379(
            &mut env,
            &congestion_control_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__congestion_control_val)
    } else {
        ::core::option::Option::None
    };
    let priority = if priority_present != 0u8 {
        let __priority_val = match jint_to_Priority_447102d2(&mut env, &priority_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__priority_val)
    } else {
        ::core::option::Option::None
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let reliability = if reliability_present != 0u8 {
        let __reliability_val = match jint_to_Reliability_5d4a96c8(
            &mut env,
            &reliability_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__reliability_val)
    } else {
        ::core::option::Option::None
    };
    let __out = match zenoh_flat::session_declare_publisher(
        &session,
        __folded_key_expr,
        __folded_encoding.as_ref(),
        congestion_control,
        priority,
        express,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Publisher_to_jlong_7bfc8296(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareQuerier<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    target_present: jni::sys::jboolean,
    target_value: jni::sys::jint,
    consolidation_present: jni::sys::jboolean,
    consolidation_value: jni::sys::jint,
    congestion_control_present: jni::sys::jboolean,
    congestion_control_value: jni::sys::jint,
    priority_present: jni::sys::jboolean,
    priority_value: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    timeout_ms_present: jni::sys::jboolean,
    timeout_ms_value: jni::sys::jlong,
    accept_replies_present: jni::sys::jboolean,
    accept_replies_value: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let target = if target_present != 0u8 {
        let __target_val = match jint_to_QueryTarget_71d4db6a(&mut env, &target_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__target_val)
    } else {
        ::core::option::Option::None
    };
    let consolidation = if consolidation_present != 0u8 {
        let __consolidation_val = match jint_to_ConsolidationMode_dd4eaedc(
            &mut env,
            &consolidation_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__consolidation_val)
    } else {
        ::core::option::Option::None
    };
    let congestion_control = if congestion_control_present != 0u8 {
        let __congestion_control_val = match jint_to_CongestionControl_62e38379(
            &mut env,
            &congestion_control_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__congestion_control_val)
    } else {
        ::core::option::Option::None
    };
    let priority = if priority_present != 0u8 {
        let __priority_val = match jint_to_Priority_447102d2(&mut env, &priority_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__priority_val)
    } else {
        ::core::option::Option::None
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let timeout_ms = if timeout_ms_present != 0u8 {
        let __timeout_ms_val = match jlong_to_i64_fbf9a9bc(&mut env, &timeout_ms_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__timeout_ms_val)
    } else {
        ::core::option::Option::None
    };
    let accept_replies = if accept_replies_present != 0u8 {
        let __accept_replies_val = match jint_to_ReplyKeyExpr_0d9719f5(
            &mut env,
            &accept_replies_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__accept_replies_val)
    } else {
        ::core::option::Option::None
    };
    let __out = match zenoh_flat::session_declare_querier(
        &session,
        __folded_key_expr,
        target,
        consolidation,
        congestion_control,
        priority,
        express,
        timeout_ms,
        accept_replies,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Querier_to_jlong_9db85a56(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareQueryable<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    complete_present: jni::sys::jboolean,
    complete_value: jni::sys::jboolean,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let complete = if complete_present != 0u8 {
        let __complete_val = match jboolean_to_bool_31306d98(&mut env, &complete_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return 0 as jni::sys::jlong;
            }
        };
        ::core::option::Option::Some(__complete_val)
    } else {
        ::core::option::Option::None
    };
    let callback = match JObject_to_impl_Fn_Query_Send_Sync_static_6c353bcb(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match JObject_to_impl_Fn_Send_Sync_static_90cfb0b9(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_queryable(
        &session,
        __folded_key_expr,
        complete,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Queryable_to_jlong_f7f9bb6c(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareSubscriber<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_f7eec5be(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match JObject_to_impl_Fn_Sample_Send_Sync_static_a050ca1d(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match JObject_to_impl_Fn_Send_Sync_static_90cfb0b9(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_subscriber(
        &session,
        __folded_key_expr,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match Subscriber_to_jlong_73e1b4a2(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    congestion_control_present: jni::sys::jboolean,
    congestion_control_value: jni::sys::jint,
    priority_present: jni::sys::jboolean,
    priority_value: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    attachment: jni::objects::JByteArray<'a>,
    reliability_present: jni::sys::jboolean,
    reliability_value: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_d960fa7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let congestion_control = if congestion_control_present != 0u8 {
        let __congestion_control_val = match jint_to_CongestionControl_62e38379(
            &mut env,
            &congestion_control_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__congestion_control_val)
    } else {
        ::core::option::Option::None
    };
    let priority = if priority_present != 0u8 {
        let __priority_val = match jint_to_Priority_447102d2(&mut env, &priority_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__priority_val)
    } else {
        ::core::option::Option::None
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let reliability = if reliability_present != 0u8 {
        let __reliability_val = match jint_to_Reliability_5d4a96c8(
            &mut env,
            &reliability_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__reliability_val)
    } else {
        ::core::option::Option::None
    };
    let __out = match zenoh_flat::session_delete(
        &session,
        &__folded_key_expr,
        congestion_control,
        priority,
        express,
        __folded_attachment,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    parameters: jni::objects::JString<'a>,
    timeout_ms_present: jni::sys::jboolean,
    timeout_ms_value: jni::sys::jlong,
    target_present: jni::sys::jboolean,
    target_value: jni::sys::jint,
    consolidation_present: jni::sys::jboolean,
    consolidation_value: jni::sys::jint,
    accept_replies_present: jni::sys::jboolean,
    accept_replies_value: jni::sys::jint,
    congestion_control_present: jni::sys::jboolean,
    congestion_control_value: jni::sys::jint,
    priority_present: jni::sys::jboolean,
    priority_value: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_d960fa7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let parameters = match JString_to_Option_String_56d5e304(&mut env, &parameters) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let timeout_ms = if timeout_ms_present != 0u8 {
        let __timeout_ms_val = match jlong_to_i64_fbf9a9bc(&mut env, &timeout_ms_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__timeout_ms_val)
    } else {
        ::core::option::Option::None
    };
    let target = if target_present != 0u8 {
        let __target_val = match jint_to_QueryTarget_71d4db6a(&mut env, &target_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__target_val)
    } else {
        ::core::option::Option::None
    };
    let consolidation = if consolidation_present != 0u8 {
        let __consolidation_val = match jint_to_ConsolidationMode_dd4eaedc(
            &mut env,
            &consolidation_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__consolidation_val)
    } else {
        ::core::option::Option::None
    };
    let accept_replies = if accept_replies_present != 0u8 {
        let __accept_replies_val = match jint_to_ReplyKeyExpr_0d9719f5(
            &mut env,
            &accept_replies_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__accept_replies_val)
    } else {
        ::core::option::Option::None
    };
    let congestion_control = if congestion_control_present != 0u8 {
        let __congestion_control_val = match jint_to_CongestionControl_62e38379(
            &mut env,
            &congestion_control_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__congestion_control_val)
    } else {
        ::core::option::Option::None
    };
    let priority = if priority_present != 0u8 {
        let __priority_val = match jint_to_Priority_447102d2(&mut env, &priority_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__priority_val)
    } else {
        ::core::option::Option::None
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let __exp_payload = match JByteArray_to_Option_Vec_u8_6f4428ab(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match match __exp_payload {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let callback = match JObject_to_impl_Fn_Reply_Send_Sync_static_a5b82e2d(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match JObject_to_impl_Fn_Send_Sync_static_90cfb0b9(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::session_get(
        &session,
        &__folded_key_expr,
        parameters,
        timeout_ms,
        target,
        consolidation,
        accept_replies,
        congestion_control,
        priority,
        express,
        __folded_payload,
        __folded_encoding.as_ref(),
        __folded_attachment,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetPeersZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/config/ZenohIdFolderRaw";
    const __CB_DESCR: &str = "(Ljava/lang/Object;[B)Ljava/lang/Object;";
    let __vec = zenoh_flat::session_get_peers_zid(&session);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let __enc = match ZenohId_to_JByteArray_2caee6f1(&mut env, __elem) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj: jni::objects::JObject = __enc.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetRoutersZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/config/ZenohIdFolderRaw";
    const __CB_DESCR: &str = "(Ljava/lang/Object;[B)Ljava/lang/Object;";
    let __vec = zenoh_flat::session_get_routers_zid(&session);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let __enc = match ZenohId_to_JByteArray_2caee6f1(&mut env, __elem) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj: jni::objects::JObject = __enc.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::session_get_zid(&session);
    match ZenohId_to_JByteArray_2caee6f1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionPut<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JString<'a>,
    encoding_1: jni::sys::jlong,
    congestion_control_present: jni::sys::jboolean,
    congestion_control_value: jni::sys::jint,
    priority_present: jni::sys::jboolean,
    priority_value: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    attachment: jni::objects::JByteArray<'a>,
    reliability_present: jni::sys::jboolean,
    reliability_value: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match JString_to_Option_String_56d5e304(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match jlong_to_Option_KeyExpr_d960fa7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match jint_to_i32_a3e3b6ef(&mut env, &encoding_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: Option<i32> = if encoding_0_0_present != 0u8 {
        let __v = match jint_to_i32_a3e3b6ef(&mut env, &encoding_0_0_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__v)
    } else {
        ::core::option::Option::None
    };
    let __exp_encoding_0_1 = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match jlong_to_Option_Encoding_e89ec09d(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let congestion_control = if congestion_control_present != 0u8 {
        let __congestion_control_val = match jint_to_CongestionControl_62e38379(
            &mut env,
            &congestion_control_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__congestion_control_val)
    } else {
        ::core::option::Option::None
    };
    let priority = if priority_present != 0u8 {
        let __priority_val = match jint_to_Priority_447102d2(&mut env, &priority_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__priority_val)
    } else {
        ::core::option::Option::None
    };
    let express = if express_present != 0u8 {
        let __express_val = match jboolean_to_bool_31306d98(&mut env, &express_value) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__express_val)
    } else {
        ::core::option::Option::None
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let reliability = if reliability_present != 0u8 {
        let __reliability_val = match jint_to_Reliability_5d4a96c8(
            &mut env,
            &reliability_value,
        ) {
            ::core::result::Result::Ok(__v) => __v,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return ();
            }
        };
        ::core::option::Option::Some(__reliability_val)
    } else {
        ::core::option::Option::None
    };
    let __out = match zenoh_flat::session_put(
        &session,
        &__folded_key_expr,
        __folded_payload,
        __folded_encoding.as_ref(),
        congestion_control,
        priority,
        express,
        __folded_attachment,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionUndeclareKeyexpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    if key_expr == 0 || (key_expr & 1) == 1 {
        signal_binding_error(
            &mut env,
            &__error_sink,
            &__SINK_MID,
            __SINK_FQN,
            __SINK_DESCR,
            "Operation on a closed native handle.",
        );
        return ();
    }
    let key_expr: zenoh_flat::KeyExpr = unsafe {
        *std::boxed::Box::from_raw(key_expr as *mut zenoh_flat::KeyExpr)
    };
    let __out = match zenoh_flat::session_undeclare_keyexpr(&session, key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_timestampGetId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    t: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let t = match jlong_to_Timestamp_bfea2165(&mut env, &t) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::timestamp_get_id(&t);
    match Vec_u8_to_JByteArray_7936d5de(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_timestampGetNtp64<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    t: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let t = match jlong_to_Timestamp_bfea2165(&mut env, &t) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::timestamp_get_ntp64(&t);
    match i64_to_jlong_fbf9a9bc(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_tryInitZenohLogsFromEnv<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::try_init_zenoh_logs_from_env();
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zbytesAsBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match jlong_to_ZBytes_141dc9e1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zbytes_as_bytes(&z);
    match std_borrow_Cow_u8_to_JByteArray_c6a6bddf(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zbytesNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match jlong_to_ZBytes_141dc9e1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::zbytes_new_clone(&z);
    match ZBytes_to_jlong_141dc9e1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zbytesNewFromVec<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    bytes: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let bytes = match JByteArray_to_Vec_u8_7936d5de(&mut env, &bytes) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::zbytes_new_from_vec(bytes);
    match ZBytes_to_jlong_141dc9e1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zenohIdToBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match JByteArray_to_ZenohId_2caee6f1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zenoh_id_to_bytes(&z);
    match Vec_u8_to_JByteArray_7936d5de(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zenohIdToString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match JByteArray_to_ZenohId_2caee6f1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zenoh_id_to_string(&z);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
const _: () = {
    konst::assertc_eq!(
        zenoh_flat::FEATURES,
        "zenoh-flat/auth_pubkey zenoh-flat/auth_usrpwd zenoh-flat/transport_compression zenoh-flat/transport_multilink zenoh-flat/transport_quic zenoh-flat/transport_quic_datagram zenoh-flat/transport_tcp zenoh-flat/transport_tls zenoh-flat/transport_udp zenoh-flat/transport_unixsock-stream zenoh-flat/transport_ws zenoh-flat/unstable",
        "prebindgen: features mismatch between source crate and prebindgen generated file.\n\
                        This usually happens if source crate is compiled with different feature set\n\
                        for build dependencies and for library usage. You may need to explicitly set\n\
                        the necessary features."
    );
};
