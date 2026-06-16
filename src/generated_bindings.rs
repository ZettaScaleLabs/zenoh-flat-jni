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
pub(crate) fn signal_error(
    env: &mut jni::JNIEnv,
    sink: &jni::objects::JObject,
    mid: &::prebindgen::lang::CachedIfaceMethod,
    fqn: &str,
    descr: &str,
    je: ::core::option::Option<&str>,
    ze: &[jni::sys::jvalue],
) {
    if env.exception_check().unwrap_or(false) {
        return;
    }
    let __je: jni::objects::JObject = match je {
        ::core::option::Option::Some(__m) => {
            match env.new_string(__m) {
                Ok(s) => s.into(),
                Err(e) => {
                    tracing::error!("signal_error: new_string failed: {}", e);
                    return;
                }
            }
        }
        ::core::option::Option::None => jni::objects::JObject::null(),
    };
    let mut __args: ::std::vec::Vec<jni::sys::jvalue> = ::std::vec::Vec::with_capacity(
        1 + ze.len(),
    );
    __args
        .push(jni::sys::jvalue {
            l: __je.as_raw(),
        });
    __args.extend_from_slice(ze);
    if let Err(e) = mid.call_object(env, fqn, "run", descr, sink, &__args) {
        tracing::error!("signal_error: error-callback invoke failed: {}", e);
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_Encoding_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Encoding));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_ZBytes_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZBytes));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_config_Config_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Config));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_errors_Error_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Error));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_keyexpr_KeyExpr_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::KeyExpr));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_liveliness_LivelinessToken_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::LivelinessToken));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_Publisher_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Publisher));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_Subscriber_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Subscriber));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Querier_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Querier));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Query_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Query));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Queryable_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Queryable));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ReplyError_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ReplyError));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Reply_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Reply));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_sample_Sample_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Sample));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_Hello_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Hello));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_Scout_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Scout));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_session_Session_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Session));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_time_Timestamp_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Timestamp));
    }
}
const _: () = {
    const fn __assert_copy<T: ::core::marker::Copy>() {}
    __assert_copy::<zenoh_flat::ZenohId>();
};
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
pub(crate) unsafe fn Error_to_jlong_0740464d<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Error,
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
                "(JLjava/lang/String;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Integer;Ljava/lang/Long;IJ)V",
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
                    let __cb0_obj4: jni::objects::JObject = match zenoh_flat::query_get_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc4 = match i32_to_jint_a3e3b6ef(
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
                            match ::prebindgen::lang::box_jint(&mut env, __enc4) {
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
                    let __cb0_obj0: jni::sys::jvalue = {
                        let __h0: jni::sys::jlong = match KeyExpr_to_jlong_57109ee0(
                            &mut env,
                            zenoh_flat::query_get_keyexpr(&__cb_arg0),
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
                        jni::sys::jvalue { j: __h0 }
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
                    let __cb0_obj3: jni::objects::JObject = match zenoh_flat::query_get_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h3: jni::sys::jlong = match Encoding_to_jlong_072adb3b(
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
                            match ::prebindgen::lang::box_jlong(&mut env, __h3) {
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
                                __cb0_obj0,
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
                "([BIZLjava/lang/Long;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Integer;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Boolean;Ljava/lang/Integer;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Integer;[BLjava/lang/Integer;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Integer;)V",
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
                    let __cb0_obj6: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc6 = match i32_to_jint_a3e3b6ef(
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
                            match ::prebindgen::lang::box_jint(&mut env, __enc6) {
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
                    let __cb0_obj19: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc19 = match i32_to_jint_a3e3b6ef(
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
                            match ::prebindgen::lang::box_jint(&mut env, __enc19) {
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
                    let __cb0_obj3: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h3: jni::sys::jlong = match KeyExpr_to_jlong_57109ee0(
                                &mut env,
                                zenoh_flat::sample_get_key_expr(__n0),
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
                            match ::prebindgen::lang::box_jlong(&mut env, __h3) {
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
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::reply_get_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h5: jni::sys::jlong = match Encoding_to_jlong_072adb3b(
                                &mut env,
                                zenoh_flat::sample_get_encoding(__n0),
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
                    let __cb0_obj18: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h18: jni::sys::jlong = match Encoding_to_jlong_072adb3b(
                                &mut env,
                                zenoh_flat::reply_error_get_encoding(__n0),
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
                            match ::prebindgen::lang::box_jlong(&mut env, __h18) {
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
                "(JJJIILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)V",
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
                    let __cb0_obj3: jni::sys::jvalue = {
                        let __enc3 = match i32_to_jint_a3e3b6ef(
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
                        jni::sys::jvalue { i: __enc3 }
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
                    let __cb0_obj0: jni::sys::jvalue = {
                        let __h0: jni::sys::jlong = match KeyExpr_to_jlong_57109ee0(
                            &mut env,
                            zenoh_flat::sample_get_key_expr(&__cb_arg0),
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
                        jni::sys::jvalue { j: __h0 }
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
                    let __cb0_obj2: jni::sys::jvalue = {
                        let __h2: jni::sys::jlong = match Encoding_to_jlong_072adb3b(
                            &mut env,
                            zenoh_flat::sample_get_encoding(&__cb_arg0),
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
                        jni::sys::jvalue { j: __h2 }
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
                                __cb0_obj0,
                                __cb0_obj1,
                                __cb0_obj2,
                                __cb0_obj3,
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
            0 => CongestionControl::Drop,
            1 => CongestionControl::Block,
            2 => CongestionControl::BlockFirst,
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
            0 => ConsolidationMode::Auto,
            1 => ConsolidationMode::None,
            2 => ConsolidationMode::Monotonic,
            3 => ConsolidationMode::Latest,
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
            1 => Priority::RealTime,
            2 => Priority::InteractiveHigh,
            3 => Priority::InteractiveLow,
            4 => Priority::DataHigh,
            5 => Priority::Data,
            6 => Priority::DataLow,
            7 => Priority::Background,
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
            0 => QueryTarget::BestMatching,
            1 => QueryTarget::All,
            2 => QueryTarget::AllComplete,
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
            0 => Reliability::BestEffort,
            1 => Reliability::Reliable,
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
            0 => ReplyKeyExpr::Any,
            1 => ReplyKeyExpr::MatchingQuery,
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
            0 => SampleKind::Put,
            1 => SampleKind::Delete,
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
            0 => SetIntersectionLevel::Disjoint,
            1 => SetIntersectionLevel::Intersects,
            2 => SetIntersectionLevel::Includes,
            3 => SetIntersectionLevel::Equals,
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
            1 => WhatAmI::Router,
            2 => WhatAmI::Peer,
            4 => WhatAmI::Client,
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
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Config) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Encoding_e0e31e0d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Encoding>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Encoding) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Error_0740464d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Error>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Error) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Hello_bbd3fc65<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Hello>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Hello) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_KeyExpr_5d6bcc5b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::KeyExpr>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::KeyExpr) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_LivelinessToken_d3477f0e<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::LivelinessToken>, __JniErr> {
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
        } else {
            Some(*std::boxed::Box::from_raw(*v as *mut zenoh_flat::KeyExpr))
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_Sample_f6c35acc<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<zenoh_flat::Sample>, __JniErr> {
    Ok({
        if *v == 0 {
            None
        } else {
            Some(*std::boxed::Box::from_raw(*v as *mut zenoh_flat::Sample))
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
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Publisher) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Querier_9db85a56<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Querier>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Querier) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Query_3af47090<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Query>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Query) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Queryable_f7f9bb6c<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Queryable>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Queryable) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ReplyError_9db9d1a6<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ReplyError>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ReplyError) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Reply_8e506ce5<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Reply>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Reply) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Sample_f8134321<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Sample>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Sample) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Scout_794eae84<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Scout>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Scout) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Session_4d3982f6<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Session>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Session) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Subscriber_73e1b4a2<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Subscriber>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Subscriber) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Timestamp_bfea2165<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Timestamp>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Timestamp) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZBytes_141dc9e1<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZBytes>, __JniErr> {
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
pub(crate) unsafe fn static_Encoding_to_jlong_a909d874<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &'static zenoh_flat::Encoding,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
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
) -> jni::objects::JString<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let c = match jlong_to_Config_d1f60c7d(&mut env, &c) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let key = match JString_to_String_c7f3ca43(&mut env, &key) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::config_get_json(&c, &key);
    let __out_s0 = match Result_String_Error_to_String_1b7d13da(&mut env, __out) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    match String_to_JString_c7f3ca43(&mut env, __out_s0) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let mut c = match jlong_to_Config_d1f60c7d(&mut env, &c) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let key = match JString_to_String_c7f3ca43(&mut env, &key) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let value = match JString_to_String_c7f3ca43(&mut env, &value) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let c = match jlong_to_Config_d1f60c7d(&mut env, &c) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::config_new_clone(&c);
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::config_new_default();
    match Config_to_jlong_d1f60c7d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let path = match JString_to_String_c7f3ca43(&mut env, &path) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationCbor<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_cbor();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationCdr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_cdr();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationCoapPayload<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_coap_payload();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationJavaSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_java_serialized_object();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_json();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationJsonPatchJson<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_json_patch_json();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationJsonSeq<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_json_seq();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationJsonpath<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_jsonpath();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationJwt<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_jwt();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_mp4();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationOctetStream<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_octet_stream();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationOpenmetricsText<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_openmetrics_text();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationProtobuf<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_protobuf();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationPythonSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_python_serialized_object();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationSoapXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_soap_xml();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationSql<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_sql();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationXWwwFormUrlencoded<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_x_www_form_urlencoded();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_xml();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_yaml();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstApplicationYang<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_application_yang();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstAudioAac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_audio_aac();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstAudioFlac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_audio_flac();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstAudioMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_audio_mp4();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstAudioOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_audio_ogg();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstAudioVorbis<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_audio_vorbis();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstImageBmp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_image_bmp();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstImageGif<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_image_gif();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstImageJpeg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_image_jpeg();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstImagePng<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_image_png();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstImageWebp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_image_webp();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextCss<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_css();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextCsv<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_csv();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextHtml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_html();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextJavascript<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_javascript();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_json();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_json5();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextMarkdown<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_markdown();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextPlain<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_plain();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_xml();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstTextYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_text_yaml();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoH261<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_h261();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoH263<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_h263();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoH264<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_h264();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoH265<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_h265();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoH266<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_h266();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_mp4();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_ogg();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoRaw<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_raw();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoVp8<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_vp8();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstVideoVp9<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_video_vp9();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstZenohBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_zenoh_bytes();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstZenohSerialized<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_zenoh_serialized();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingConstZenohString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::encoding_const_zenoh_string();
    match static_Encoding_to_jlong_a909d874(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::encoding_get_id(&e);
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::encoding_get_schema(&e);
    match Option_String_to_JString_56d5e304(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_clone(&e);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let id = match jint_to_i32_a3e3b6ef(&mut env, &id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let schema = match JString_to_Option_String_56d5e304(&mut env, &schema) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_from_id(id, schema);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_from_string(s);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    e_id: jni::sys::jint,
    e_schema: jni::objects::JString<'a>,
    schema: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_e_id = match jint_to_i32_a3e3b6ef(&mut env, &e_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_e_schema = match JString_to_Option_String_56d5e304(&mut env, &e_schema) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_e = match ::core::result::Result::Ok(
        zenoh_flat::encoding_new_from_id(__exp_e_id, __exp_e_schema),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let schema = match JString_to_String_c7f3ca43(&mut env, &schema) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_with_schema(&__folded_e, schema);
    match Encoding_to_jlong_e0e31e0d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Encoding_e0e31e0d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::encoding_to_string(&e);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_errorGetMessage<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_Error_0740464d(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::error_get_message(&e);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match jlong_to_Hello_bbd3fc65(&mut env, &h) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::hello_get_locators(&h);
    match Vec_String_to_JObject_1e282499(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_helloGetWhatami<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    h: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match jlong_to_Hello_bbd3fc65(&mut env, &h) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::hello_get_whatami(&h);
    match WhatAmI_to_jint_4c5d5738(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match jlong_to_Hello_bbd3fc65(&mut env, &h) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::hello_get_zid(&h);
    match ZenohId_to_JByteArray_2caee6f1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let filter = match JString_to_String_c7f3ca43(&mut env, &filter) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __out = zenoh_flat::init_android_logs(&filter);
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let fallback_filter = match JString_to_String_c7f3ca43(&mut env, &fallback_filter) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __out = zenoh_flat::init_zenoh_logs_from_env_or(&fallback_filter);
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &ke) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::keyexpr_get_str(&ke);
    match str_to_JString_7b77dc67(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &a) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_sel = match jint_to_i32_a3e3b6ef(&mut env, &b_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_0 = match JString_to_Option_String_56d5e304(&mut env, &b_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &b_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::keyexpr_includes(&a, &__folded_b);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &a) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_sel = match jint_to_i32_a3e3b6ef(&mut env, &b_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_0 = match JString_to_Option_String_56d5e304(&mut env, &b_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &b_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::keyexpr_intersects(&a, &__folded_b);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &ke) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::keyexpr_new_clone(&ke);
    match KeyExpr_to_jlong_5d6bcc5b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_a_sel = match jint_to_i32_a3e3b6ef(&mut env, &a_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_0 = match JString_to_Option_String_56d5e304(&mut env, &a_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &a_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let b = match JString_to_String_c7f3ca43(&mut env, &b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_a_sel = match jint_to_i32_a3e3b6ef(&mut env, &a_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_0 = match JString_to_Option_String_56d5e304(&mut env, &a_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &a_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let b = match JString_to_String_c7f3ca43(&mut env, &b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let s = match JString_to_String_c7f3ca43(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &a) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_sel = match jint_to_i32_a3e3b6ef(&mut env, &b_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_0 = match JString_to_Option_String_56d5e304(&mut env, &b_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_1 = match jlong_to_Option_KeyExpr_d960fa7d(&mut env, &b_1) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::keyexpr_relation_to(&a, &__folded_b);
    match SetIntersectionLevel_to_jint_0e49fc84(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match jlong_to_KeyExpr_5d6bcc5b(&mut env, &ke) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::keyexpr_to_string(&ke);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let history = match jboolean_to_bool_31306d98(&mut env, &history) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let timeout_ms = match jlong_to_i64_fbf9a9bc(&mut env, &timeout_ms) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_publisherDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match jlong_to_Publisher_7bfc8296(&mut env, &publisher) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    encoding_present: jni::sys::jboolean,
    encoding_id: jni::sys::jint,
    encoding_schema: jni::objects::JString<'a>,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match jlong_to_Publisher_7bfc8296(&mut env, &publisher) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_present = match jboolean_to_bool_31306d98(
        &mut env,
        &encoding_present,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_id = match jint_to_i32_a3e3b6ef(&mut env, &encoding_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_schema = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_present {
        (::core::result::Result::Ok(
            zenoh_flat::encoding_new_from_id(__exp_encoding_id, __exp_encoding_schema),
        ))
            .map(::core::option::Option::Some)
    } else {
        ::core::result::Result::Ok(::core::option::Option::None)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    encoding_present: jni::sys::jboolean,
    encoding_id: jni::sys::jint,
    encoding_schema: jni::objects::JString<'a>,
    attachment: jni::objects::JByteArray<'a>,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let querier = match jlong_to_Querier_9db85a56(&mut env, &querier) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let parameters = match JString_to_Option_String_56d5e304(&mut env, &parameters) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Option_Vec_u8_6f4428ab(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_present = match jboolean_to_bool_31306d98(
        &mut env,
        &encoding_present,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_id = match jint_to_i32_a3e3b6ef(&mut env, &encoding_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_schema = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_present {
        (::core::result::Result::Ok(
            zenoh_flat::encoding_new_from_id(__exp_encoding_id, __exp_encoding_schema),
        ))
            .map(::core::option::Option::Some)
    } else {
        ::core::result::Result::Ok(::core::option::Option::None)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::query_get_accepts_replies(&q);
    match ReplyKeyExpr_to_jint_0d9719f5(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_attachment(&q);
    match Option_ZBytes_to_jlong_c521cd2f(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_encoding(&q);
    match Option_Encoding_to_jlong_e89ec09d(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_keyexpr(&q);
    match KeyExpr_to_jlong_57109ee0(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::query_get_parameters(&q);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match jlong_to_Query_3af47090(&mut env, &q) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_payload(&q);
    match Option_ZBytes_to_jlong_c521cd2f(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    timestamp_ntp64: jni::objects::JObject<'a>,
    attachment: jni::objects::JByteArray<'a>,
    express: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let timestamp_ntp64 = match JObject_to_Option_i64_2ba9a5ed(
        &mut env,
        &timestamp_ntp64,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    encoding_present: jni::sys::jboolean,
    encoding_id: jni::sys::jint,
    encoding_schema: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_present = match jboolean_to_bool_31306d98(
        &mut env,
        &encoding_present,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_id = match jint_to_i32_a3e3b6ef(&mut env, &encoding_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_schema = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_present {
        (::core::result::Result::Ok(
            zenoh_flat::encoding_new_from_id(__exp_encoding_id, __exp_encoding_schema),
        ))
            .map(::core::option::Option::Some)
    } else {
        ::core::result::Result::Ok(::core::option::Option::None)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    sample_sel: jni::sys::jint,
    sample_0: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_sample_sel = match jint_to_i32_a3e3b6ef(&mut env, &sample_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_sample_0 = match jlong_to_Option_Sample_f6c35acc(&mut env, &sample_0) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __folded_sample = match {
        match __exp_sample_sel {
            0i32 => {
                match __exp_sample_0 {
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __out = match zenoh_flat::query_reply_sample(&query, __folded_sample) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    encoding_present: jni::sys::jboolean,
    encoding_id: jni::sys::jint,
    encoding_schema: jni::objects::JString<'a>,
    timestamp_ntp64: jni::objects::JObject<'a>,
    attachment: jni::objects::JByteArray<'a>,
    express: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_Query_3af47090(&mut env, &query) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_present = match jboolean_to_bool_31306d98(
        &mut env,
        &encoding_present,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_id = match jint_to_i32_a3e3b6ef(&mut env, &encoding_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_schema = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_present {
        (::core::result::Result::Ok(
            zenoh_flat::encoding_new_from_id(__exp_encoding_id, __exp_encoding_schema),
        ))
            .map(::core::option::Option::Some)
    } else {
        ::core::result::Result::Ok(::core::option::Option::None)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let timestamp_ntp64 = match JObject_to_Option_i64_2ba9a5ed(
        &mut env,
        &timestamp_ntp64,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_ReplyError_9db9d1a6(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_error_get_encoding(&e);
    match Encoding_to_jlong_072adb3b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match jlong_to_ReplyError_9db9d1a6(&mut env, &e) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_error_get_payload(&e);
    match ZBytes_to_jlong_56134c74(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_get_err(&r);
    match Option_ReplyError_to_jlong_d3e8c438(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::reply_get_replier_eid(&r);
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::reply_get_replier_zid(&r);
    match Option_ZenohId_to_JByteArray_6880b2ba(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_get_sample(&r);
    match Option_Sample_to_jlong_e48d7024(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match jlong_to_Reply_8e506ce5(&mut env, &r) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::reply_is_ok(&r);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_attachment(&s);
    match Option_ZBytes_to_jlong_c521cd2f(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_congestion_control(&s);
    match CongestionControl_to_jint_62e38379(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_encoding(&s);
    match Encoding_to_jlong_072adb3b(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::sample_get_express(&s);
    match bool_to_jboolean_31306d98(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_key_expr(&s);
    match KeyExpr_to_jlong_57109ee0(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_kind(&s);
    match SampleKind_to_jint_d7ea75a8(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_payload(&s);
    match ZBytes_to_jlong_56134c74(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_priority(&s);
    match Priority_to_jint_447102d2(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_reliability(&s);
    match Reliability_to_jint_5d4a96c8(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_source_eid(&s);
    match i32_to_jint_a3e3b6ef(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_source_sn(&s);
    match i64_to_jlong_fbf9a9bc(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::sample_get_source_zid(&s);
    match Option_ZenohId_to_JByteArray_6880b2ba(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match jlong_to_Sample_f8134321(&mut env, &s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_timestamp(&s);
    match Option_Timestamp_to_jlong_880c755c(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    timestamp_ntp64: jni::objects::JObject<'a>,
    attachment: jni::objects::JByteArray<'a>,
    congestion_control: jni::objects::JObject<'a>,
    priority: jni::objects::JObject<'a>,
    express: jni::objects::JObject<'a>,
    reliability: jni::objects::JObject<'a>,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let timestamp_ntp64 = match JObject_to_Option_i64_2ba9a5ed(
        &mut env,
        &timestamp_ntp64,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let congestion_control = match JObject_to_Option_CongestionControl_7053bb49(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let priority = match JObject_to_Option_Priority_ad5cbb32(&mut env, &priority) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let reliability = match JObject_to_Option_Reliability_60b5e063(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/sample/SampleBuilderRaw";
    const __CB_DESCR: &str = "(JJJIILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)Ljava/lang/Object;";
    let __out = zenoh_flat::sample_new_delete(
        __folded_key_expr,
        timestamp_ntp64,
        __folded_attachment,
        congestion_control,
        priority,
        express,
        reliability,
    );
    let __obj3: jni::sys::jvalue = {
        let __enc3 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::encoding_get_id(zenoh_flat::sample_get_encoding(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc3 }
    };
    let __obj4: jni::sys::jvalue = {
        let __enc4 = match SampleKind_to_jint_d7ea75a8(
            &mut env,
            zenoh_flat::sample_get_kind(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e.to_string()),
                        &__zd,
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __enc5) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e),
                        &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __enc13 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match KeyExpr_to_jlong_57109ee0(
            &mut env,
            zenoh_flat::sample_get_key_expr(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h0 }
    };
    let __obj1: jni::sys::jvalue = {
        let __h1: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
            &mut env,
            zenoh_flat::sample_get_payload(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h1 }
    };
    let __obj2: jni::sys::jvalue = {
        let __h2: jni::sys::jlong = match Encoding_to_jlong_072adb3b(
            &mut env,
            zenoh_flat::sample_get_encoding(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h2 }
    };
    let __obj9: jni::objects::JObject = match zenoh_flat::sample_get_attachment(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __h9: jni::sys::jlong = match ZBytes_to_jlong_56134c74(&mut env, __n0) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e.to_string()),
                        &__zd,
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __h9) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e.to_string()),
                        &__zd,
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
                __obj0,
                __obj1,
                __obj2,
                __obj3,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e2.to_string()),
                &__zd,
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
    encoding_present: jni::sys::jboolean,
    encoding_id: jni::sys::jint,
    encoding_schema: jni::objects::JString<'a>,
    timestamp_ntp64: jni::objects::JObject<'a>,
    attachment: jni::objects::JByteArray<'a>,
    congestion_control: jni::objects::JObject<'a>,
    priority: jni::objects::JObject<'a>,
    express: jni::objects::JObject<'a>,
    reliability: jni::objects::JObject<'a>,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_encoding_present = match jboolean_to_bool_31306d98(
        &mut env,
        &encoding_present,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_encoding_id = match jint_to_i32_a3e3b6ef(&mut env, &encoding_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_encoding_schema = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __folded_encoding = match if __exp_encoding_present {
        (::core::result::Result::Ok(
            zenoh_flat::encoding_new_from_id(__exp_encoding_id, __exp_encoding_schema),
        ))
            .map(::core::option::Option::Some)
    } else {
        ::core::result::Result::Ok(::core::option::Option::None)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let timestamp_ntp64 = match JObject_to_Option_i64_2ba9a5ed(
        &mut env,
        &timestamp_ntp64,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __exp_attachment = match JByteArray_to_Option_Vec_u8_6f4428ab(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let congestion_control = match JObject_to_Option_CongestionControl_7053bb49(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let priority = match JObject_to_Option_Priority_ad5cbb32(&mut env, &priority) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let reliability = match JObject_to_Option_Reliability_60b5e063(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/sample/SampleBuilderRaw";
    const __CB_DESCR: &str = "(JJJIILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)Ljava/lang/Object;";
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
    let __obj3: jni::sys::jvalue = {
        let __enc3 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::encoding_get_id(zenoh_flat::sample_get_encoding(&__out)),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { i: __enc3 }
    };
    let __obj4: jni::sys::jvalue = {
        let __enc4 = match SampleKind_to_jint_d7ea75a8(
            &mut env,
            zenoh_flat::sample_get_kind(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e.to_string()),
                        &__zd,
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __enc5) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e),
                        &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
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
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __enc13 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match KeyExpr_to_jlong_57109ee0(
            &mut env,
            zenoh_flat::sample_get_key_expr(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h0 }
    };
    let __obj1: jni::sys::jvalue = {
        let __h1: jni::sys::jlong = match ZBytes_to_jlong_56134c74(
            &mut env,
            zenoh_flat::sample_get_payload(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h1 }
    };
    let __obj2: jni::sys::jvalue = {
        let __h2: jni::sys::jlong = match Encoding_to_jlong_072adb3b(
            &mut env,
            zenoh_flat::sample_get_encoding(&__out),
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                let __zd = __ze_defaults(&mut env);
                signal_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    ::core::option::Option::Some(&__e.to_string()),
                    &__zd,
                );
                return jni::objects::JObject::null().into();
            }
        };
        jni::sys::jvalue { j: __h2 }
    };
    let __obj9: jni::objects::JObject = match zenoh_flat::sample_get_attachment(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __h9: jni::sys::jlong = match ZBytes_to_jlong_56134c74(&mut env, __n0) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e.to_string()),
                        &__zd,
                    );
                    return jni::objects::JObject::null().into();
                }
            };
            match ::prebindgen::lang::box_jlong(&mut env, __h9) {
                ::core::result::Result::Ok(__o) => __o,
                ::core::result::Result::Err(__e) => {
                    let __zd = __ze_defaults(&mut env);
                    signal_error(
                        &mut env,
                        &__error_sink,
                        &__SINK_MID,
                        __SINK_FQN,
                        __SINK_DESCR,
                        ::core::option::Option::Some(&__e.to_string()),
                        &__zd,
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
                __obj0,
                __obj1,
                __obj2,
                __obj3,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e2.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let whatami = match jint_to_i32_a3e3b6ef(&mut env, &whatami) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let config = match jlong_to_Option_Config_61908788(&mut env, &config) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let key_expr = match JString_to_String_c7f3ca43(&mut env, &key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    congestion_control: jni::objects::JObject<'a>,
    priority: jni::objects::JObject<'a>,
    express: jni::objects::JObject<'a>,
    reliability: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = match JObject_to_Option_CongestionControl_7053bb49(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let priority = match JObject_to_Option_Priority_ad5cbb32(&mut env, &priority) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let reliability = match JObject_to_Option_Reliability_60b5e063(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_publisher(
        &session,
        __folded_key_expr,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    target: jni::objects::JObject<'a>,
    consolidation: jni::objects::JObject<'a>,
    congestion_control: jni::objects::JObject<'a>,
    priority: jni::objects::JObject<'a>,
    express: jni::objects::JObject<'a>,
    timeout_ms: jni::objects::JObject<'a>,
    accept_replies: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let target = match JObject_to_Option_QueryTarget_08d4f26d(&mut env, &target) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let consolidation = match JObject_to_Option_ConsolidationMode_25de8913(
        &mut env,
        &consolidation,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = match JObject_to_Option_CongestionControl_7053bb49(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let priority = match JObject_to_Option_Priority_ad5cbb32(&mut env, &priority) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let timeout_ms = match JObject_to_Option_i64_2ba9a5ed(&mut env, &timeout_ms) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let accept_replies = match JObject_to_Option_ReplyKeyExpr_91b36eb3(
        &mut env,
        &accept_replies,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    complete: jni::objects::JObject<'a>,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let complete = match JObject_to_Option_bool_5c82fffd(&mut env, &complete) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match JObject_to_impl_Fn_Query_Send_Sync_static_6c353bcb(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> jni::sys::jlong {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    congestion_control: jni::objects::JObject<'a>,
    priority: jni::objects::JObject<'a>,
    express: jni::objects::JObject<'a>,
    attachment: jni::objects::JByteArray<'a>,
    reliability: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let congestion_control = match JObject_to_Option_CongestionControl_7053bb49(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let priority = match JObject_to_Option_Priority_ad5cbb32(&mut env, &priority) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let reliability = match JObject_to_Option_Reliability_60b5e063(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    timeout_ms: jni::objects::JObject<'a>,
    target: jni::objects::JObject<'a>,
    consolidation: jni::objects::JObject<'a>,
    accept_replies: jni::objects::JObject<'a>,
    congestion_control: jni::objects::JObject<'a>,
    priority: jni::objects::JObject<'a>,
    express: jni::objects::JObject<'a>,
    payload: jni::objects::JByteArray<'a>,
    encoding_present: jni::sys::jboolean,
    encoding_id: jni::sys::jint,
    encoding_schema: jni::objects::JString<'a>,
    attachment: jni::objects::JByteArray<'a>,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let parameters = match JString_to_Option_String_56d5e304(&mut env, &parameters) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let timeout_ms = match JObject_to_Option_i64_2ba9a5ed(&mut env, &timeout_ms) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let target = match JObject_to_Option_QueryTarget_08d4f26d(&mut env, &target) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let consolidation = match JObject_to_Option_ConsolidationMode_25de8913(
        &mut env,
        &consolidation,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let accept_replies = match JObject_to_Option_ReplyKeyExpr_91b36eb3(
        &mut env,
        &accept_replies,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let congestion_control = match JObject_to_Option_CongestionControl_7053bb49(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let priority = match JObject_to_Option_Priority_ad5cbb32(&mut env, &priority) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Option_Vec_u8_6f4428ab(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_present = match jboolean_to_bool_31306d98(
        &mut env,
        &encoding_present,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_id = match jint_to_i32_a3e3b6ef(&mut env, &encoding_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_schema = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_present {
        (::core::result::Result::Ok(
            zenoh_flat::encoding_new_from_id(__exp_encoding_id, __exp_encoding_schema),
        ))
            .map(::core::option::Option::Some)
    } else {
        ::core::result::Result::Ok(::core::option::Option::None)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::session_get_peers_zid(&session);
    match Vec_ZenohId_to_JObject_cd7f8e6c(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetRoutersZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::session_get_routers_zid(&session);
    match Vec_ZenohId_to_JObject_cd7f8e6c(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::session_get_zid(&session);
    match ZenohId_to_JByteArray_2caee6f1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    encoding_present: jni::sys::jboolean,
    encoding_id: jni::sys::jint,
    encoding_schema: jni::objects::JString<'a>,
    congestion_control: jni::objects::JObject<'a>,
    priority: jni::objects::JObject<'a>,
    express: jni::objects::JObject<'a>,
    attachment: jni::objects::JByteArray<'a>,
    reliability: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match jint_to_i32_a3e3b6ef(&mut env, &key_expr_sel) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_payload = match JByteArray_to_Vec_u8_7936d5de(&mut env, &payload) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_present = match jboolean_to_bool_31306d98(
        &mut env,
        &encoding_present,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_id = match jint_to_i32_a3e3b6ef(&mut env, &encoding_id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __exp_encoding_schema = match JString_to_Option_String_56d5e304(
        &mut env,
        &encoding_schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_present {
        (::core::result::Result::Ok(
            zenoh_flat::encoding_new_from_id(__exp_encoding_id, __exp_encoding_schema),
        ))
            .map(::core::option::Option::Some)
    } else {
        ::core::result::Result::Ok(::core::option::Option::None)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let congestion_control = match JObject_to_Option_CongestionControl_7053bb49(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let priority = match JObject_to_Option_Priority_ad5cbb32(&mut env, &priority) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let express = match JObject_to_Option_bool_5c82fffd(&mut env, &express) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__je.to_string()),
                &__zd,
            );
            return ();
        }
    };
    let reliability = match JObject_to_Option_Reliability_60b5e063(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
) -> () {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![
            env.new_string("").map(| __s | jni::sys::jvalue { l : __s.into_raw() })
            .unwrap_or(jni::sys::jvalue { l : ::std::ptr::null_mut() })
        ]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_Session_4d3982f6(&mut env, &session) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return ();
        }
    };
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
                        let __zd = __ze_defaults(&mut env);
                        signal_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            ::core::option::Option::Some(&__e.to_string()),
                            &__zd,
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::None,
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
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let t = match jlong_to_Timestamp_bfea2165(&mut env, &t) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::timestamp_get_id(&t);
    match Vec_u8_to_JByteArray_7936d5de(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let t = match jlong_to_Timestamp_bfea2165(&mut env, &t) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::timestamp_get_ntp64(&t);
    match i64_to_jlong_fbf9a9bc(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::try_init_zenoh_logs_from_env();
    match unit_to_unit_9ecccf8e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match jlong_to_ZBytes_141dc9e1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zbytes_as_bytes(&z);
    match std_borrow_Cow_u8_to_JByteArray_c6a6bddf(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match jlong_to_ZBytes_141dc9e1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::zbytes_new_clone(&z);
    match ZBytes_to_jlong_141dc9e1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let bytes = match JByteArray_to_Vec_u8_7936d5de(&mut env, &bytes) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::zbytes_new_from_vec(bytes);
    match ZBytes_to_jlong_141dc9e1(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zbytesToBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match jlong_to_ZBytes_141dc9e1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zbytes_to_bytes(&z);
    match Vec_u8_to_JByteArray_7936d5de(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            jni::objects::JObject::null().into()
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match JByteArray_to_ZenohId_2caee6f1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zenoh_id_to_bytes(&z);
    match Vec_u8_to_JByteArray_7936d5de(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
    #[allow(unused_variables)]
    let __ze_defaults = |env: &mut jni::JNIEnv| -> ::std::vec::Vec<jni::sys::jvalue> {
        ::std::vec![]
    };
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match JByteArray_to_ZenohId_2caee6f1(&mut env, &z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zenoh_id_to_string(&z);
    match String_to_JString_c7f3ca43(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            let __zd = __ze_defaults(&mut env);
            signal_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                ::core::option::Option::Some(&__e.to_string()),
                &__zd,
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
