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
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_ZEncoding_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZEncoding));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_ZZBytes_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZZBytes));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_config_ZConfig_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZConfig));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_errors_ZError_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZError));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_keyexpr_ZKeyExpr_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZKeyExpr));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_liveliness_ZLivelinessToken_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZLivelinessToken));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_ZPublisher_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZPublisher));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_ZSubscriber_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZSubscriber));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ZQuerier_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZQuerier));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ZQuery_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZQuery));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ZQueryable_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZQueryable));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ZReplyError_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZReplyError));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ZReply_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZReply));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_sample_ZSample_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZSample));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_ZHello_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZHello));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_ZScout_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZScout));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_session_ZSession_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZSession));
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_time_ZTimestamp_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZTimestamp));
    }
}
const _: () = {
    const fn __assert_copy<T: ::core::marker::Copy>() {}
    __assert_copy::<zenoh_flat::ZZenohId>();
};
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
pub(crate) unsafe fn JByteArray_to_ZZenohId_2e8b4538<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JByteArray<'v>,
) -> ::core::result::Result<zenoh_flat::ZZenohId, __JniErr> {
    Ok({
        let __bytes = env
            .convert_byte_array(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("value-blob decode: {}", e))
            })?;
        if __bytes.len() != ::core::mem::size_of::<zenoh_flat::ZZenohId>() {
            return ::core::result::Result::Err(
                <__JniErr as ::core::convert::From<
                    String,
                >>::from("value-blob decode: wrong byte length".to_string()),
            );
        }
        unsafe {
            ::core::ptr::read_unaligned(__bytes.as_ptr() as *const zenoh_flat::ZZenohId)
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
pub(crate) unsafe fn JObject_to_impl_Fn_ZHello_Send_Sync_static_10862af4<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::ZHello) + Send + Sync + 'static,
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
                format!("Unable to get callback class for {}: {}", "Fn(ZHello)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(&__invoke_class, "run", "(I[BLjava/util/List;)V")
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(ZHello)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::ZHello| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(ZHello)", e)))?;
                env.push_local_frame(16)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(ZHello)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::sys::jvalue = {
                        let __enc0 = match WhatAmI_to_jint_4c5d5738(
                            &mut env,
                            zenoh_flat::z_hello_whatami(&__cb_arg0),
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
                        let __enc1 = match ZZenohId_to_JByteArray_2e8b4538(
                            &mut env,
                            zenoh_flat::z_hello_zid(&__cb_arg0),
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
                            zenoh_flat::z_hello_locators(&__cb_arg0),
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
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(ZHello)"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_ZQuery_Send_Sync_static_119148c0<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::ZQuery) + Send + Sync + 'static,
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
                format!("Unable to get callback class for {}: {}", "Fn(ZQuery)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(JLjava/lang/String;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Integer;Ljava/lang/Long;IJ)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(ZQuery)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::ZQuery| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(ZQuery)", e)))?;
                env.push_local_frame(22)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(ZQuery)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj1: jni::objects::JObject = {
                        let __enc1 = match String_to_JString_c7f3ca43(
                            &mut env,
                            zenoh_flat::z_query_parameters(&__cb_arg0),
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
                    let __cb0_obj4: jni::objects::JObject = match zenoh_flat::z_query_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc4 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::z_encoding_id(__n0),
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
                            zenoh_flat::z_query_accepts_replies(&__cb_arg0),
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
                        let __h0: jni::sys::jlong = match ZKeyExpr_to_jlong_fbfa2238(
                            &mut env,
                            zenoh_flat::z_query_keyexpr(&__cb_arg0),
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
                    let __cb0_obj2: jni::objects::JObject = match zenoh_flat::z_query_payload(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h2: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
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
                    let __cb0_obj3: jni::objects::JObject = match zenoh_flat::z_query_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h3: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(
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
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::z_query_attachment(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h5: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
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
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(ZQuery)"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_ZReply_Send_Sync_static_bd874b12<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::ZReply) + Send + Sync + 'static,
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
                format!("Unable to get callback class for {}: {}", "Fn(ZReply)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "([BIZLjava/lang/Long;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Integer;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Boolean;Ljava/lang/Integer;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Integer;[BLjava/lang/Integer;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Long;Ljava/lang/Integer;)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(ZReply)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::ZReply| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(ZReply)", e)))?;
                env.push_local_frame(46)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(ZReply)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::objects::JObject = {
                        let __enc0 = match Option_ZZenohId_to_JByteArray_20202e50(
                            &mut env,
                            zenoh_flat::z_reply_replier_zid(&__cb_arg0),
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
                            zenoh_flat::z_reply_replier_eid(&__cb_arg0),
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
                            zenoh_flat::z_reply_is_ok(&__cb_arg0),
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
                    let __cb0_obj6: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc6 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::z_encoding_id(
                                    zenoh_flat::z_sample_encoding(__n0),
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
                    let __cb0_obj7: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc7 = match SampleKind_to_jint_d7ea75a8(
                                &mut env,
                                zenoh_flat::z_sample_kind(__n0),
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
                    let __cb0_obj8: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            match zenoh_flat::z_sample_timestamp(__n0) {
                                ::core::option::Option::Some(__n1) => {
                                    let __enc8 = match i64_to_jlong_fbf9a9bc(
                                        &mut env,
                                        zenoh_flat::z_timestamp_ntp64(__n1),
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
                    let __cb0_obj9: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc9 = match bool_to_jboolean_31306d98(
                                &mut env,
                                zenoh_flat::z_sample_express(__n0),
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
                    let __cb0_obj10: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc10 = match Priority_to_jint_447102d2(
                                &mut env,
                                zenoh_flat::z_sample_priority(__n0),
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
                    let __cb0_obj11: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc11 = match CongestionControl_to_jint_62e38379(
                                &mut env,
                                zenoh_flat::z_sample_congestion_control(__n0),
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
                    let __cb0_obj13: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc13 = match Reliability_to_jint_5d4a96c8(
                                &mut env,
                                zenoh_flat::z_sample_reliability(__n0),
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
                    let __cb0_obj14: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc14 = match Option_ZZenohId_to_JByteArray_20202e50(
                                &mut env,
                                zenoh_flat::z_sample_source_zid(__n0),
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
                    let __cb0_obj15: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc15 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::z_sample_source_eid(__n0),
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
                    let __cb0_obj16: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc16 = match i64_to_jlong_fbf9a9bc(
                                &mut env,
                                zenoh_flat::z_sample_source_sn(__n0),
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
                    let __cb0_obj19: jni::objects::JObject = match zenoh_flat::z_reply_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc19 = match i32_to_jint_a3e3b6ef(
                                &mut env,
                                zenoh_flat::z_encoding_id(
                                    zenoh_flat::z_reply_error_encoding(__n0),
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
                    let __cb0_obj3: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h3: jni::sys::jlong = match ZKeyExpr_to_jlong_fbfa2238(
                                &mut env,
                                zenoh_flat::z_sample_key_expr(__n0),
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
                    let __cb0_obj4: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h4: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
                                &mut env,
                                zenoh_flat::z_sample_payload(__n0),
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
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h5: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(
                                &mut env,
                                zenoh_flat::z_sample_encoding(__n0),
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
                    let __cb0_obj12: jni::objects::JObject = match zenoh_flat::z_reply_sample(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            match zenoh_flat::z_sample_attachment(__n0) {
                                ::core::option::Option::Some(__n1) => {
                                    let __h12: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
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
                    let __cb0_obj17: jni::objects::JObject = match zenoh_flat::z_reply_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h17: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
                                &mut env,
                                zenoh_flat::z_reply_error_payload(__n0),
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
                    let __cb0_obj18: jni::objects::JObject = match zenoh_flat::z_reply_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h18: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(
                                &mut env,
                                zenoh_flat::z_reply_error_encoding(__n0),
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
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(ZReply)"));
        })
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn JObject_to_impl_Fn_ZSample_Send_Sync_static_24e97b15<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::ZSample) + Send + Sync + 'static,
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
                format!("Unable to get callback class for {}: {}", "Fn(ZSample)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(JJJIILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(ZSample)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::ZSample| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(ZSample)", e)))?;
                env.push_local_frame(34)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(ZSample)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj3: jni::sys::jvalue = {
                        let __enc3 = match i32_to_jint_a3e3b6ef(
                            &mut env,
                            zenoh_flat::z_encoding_id(
                                zenoh_flat::z_sample_encoding(&__cb_arg0),
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
                            zenoh_flat::z_sample_kind(&__cb_arg0),
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
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::z_sample_timestamp(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc5 = match i64_to_jlong_fbf9a9bc(
                                &mut env,
                                zenoh_flat::z_timestamp_ntp64(__n0),
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
                            zenoh_flat::z_sample_express(&__cb_arg0),
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
                            zenoh_flat::z_sample_priority(&__cb_arg0),
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
                            zenoh_flat::z_sample_congestion_control(&__cb_arg0),
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
                            zenoh_flat::z_sample_reliability(&__cb_arg0),
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
                        let __enc11 = match Option_ZZenohId_to_JByteArray_20202e50(
                            &mut env,
                            zenoh_flat::z_sample_source_zid(&__cb_arg0),
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
                            zenoh_flat::z_sample_source_eid(&__cb_arg0),
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
                            zenoh_flat::z_sample_source_sn(&__cb_arg0),
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
                        let __h0: jni::sys::jlong = match ZKeyExpr_to_jlong_fbfa2238(
                            &mut env,
                            zenoh_flat::z_sample_key_expr(&__cb_arg0),
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
                        let __h1: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
                            &mut env,
                            zenoh_flat::z_sample_payload(&__cb_arg0),
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
                        let __h2: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(
                            &mut env,
                            zenoh_flat::z_sample_encoding(&__cb_arg0),
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
                    let __cb0_obj9: jni::objects::JObject = match zenoh_flat::z_sample_attachment(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h9: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
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
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(ZSample)"));
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
pub(crate) unsafe fn Option_ZEncoding_to_jlong_28edc9d4<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::ZEncoding>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => ZEncoding_to_jlong_77133a31(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ZReplyError_to_jlong_5fea0243<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::ZReplyError>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => ZReplyError_to_jlong_a51fa0c0(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ZSample_to_jlong_d70cb420<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::ZSample>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => ZSample_to_jlong_fea0199a(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ZTimestamp_to_jlong_804b81ce<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::ZTimestamp>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => ZTimestamp_to_jlong_6e790947(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ZZBytes_to_jlong_947bea0f<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<&zenoh_flat::ZZBytes>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok({
        match v {
            Some(value) => ZZBytes_to_jlong_f37597b8(env, value)?,
            None => 0i64,
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Option_ZZenohId_to_JByteArray_20202e50<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Option<zenoh_flat::ZZenohId>,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        match v {
            Some(value) => ZZenohId_to_JByteArray_2e8b4538(env, value)?,
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
pub(crate) unsafe fn QueryTarget_to_jint_71d4db6a<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::QueryTarget,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Reliability_to_jint_5d4a96c8<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Reliability,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ReplyKeyExpr_to_jint_0d9719f5<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ReplyKeyExpr,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_String_ZError_to_String_57946770<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<String, zenoh_flat::ZError>,
) -> ::core::result::Result<String, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZConfig_ZError_to_ZConfig_f73b555c<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZConfig, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZConfig, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZError_to_unit_78f28373<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<(), zenoh_flat::ZError>,
) -> ::core::result::Result<(), zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZKeyExpr_ZError_to_ZKeyExpr_934eec32<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZKeyExpr, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZKeyExpr, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZLivelinessToken_ZError_to_ZLivelinessToken_f6df71c1<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZLivelinessToken, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZLivelinessToken, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZPublisher_ZError_to_ZPublisher_158f73af<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZPublisher, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZPublisher, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZQuerier_ZError_to_ZQuerier_3be150df<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZQuerier, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZQuerier, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZQueryable_ZError_to_ZQueryable_29fd3638<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZQueryable, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZQueryable, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZScout_ZError_to_ZScout_76fdefcd<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZScout, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZScout, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZSession_ZError_to_ZSession_67e5df17<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZSession, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZSession, zenoh_flat::ZError> {
    v
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn Result_ZSubscriber_ZError_to_ZSubscriber_78daa560<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Result<zenoh_flat::ZSubscriber, zenoh_flat::ZError>,
) -> ::core::result::Result<zenoh_flat::ZSubscriber, zenoh_flat::ZError> {
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
pub(crate) unsafe fn Vec_ZZenohId_to_JObject_ef32ce89<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: Vec<zenoh_flat::ZZenohId>,
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
            let __elem_wire = ZZenohId_to_JByteArray_2e8b4538(env, __elem)?;
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
pub(crate) unsafe fn ZConfig_to_jlong_92724b10<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZConfig,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZEncoding_to_jlong_5667ec6b<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZEncoding,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZEncoding_to_jlong_77133a31<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZEncoding,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZError_to_jlong_a71a127e<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZError,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZHello_to_jlong_5eb6fe2b<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZHello,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZKeyExpr_to_jlong_37f9dc18<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZKeyExpr,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZKeyExpr_to_jlong_fbfa2238<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZKeyExpr,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZLivelinessToken_to_jlong_64cf7e16<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZLivelinessToken,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZPublisher_to_jlong_68421aa7<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZPublisher,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZQuerier_to_jlong_1b809fcd<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZQuerier,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZQuery_to_jlong_19e50934<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZQuery,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZQueryable_to_jlong_ca27040b<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZQueryable,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZReplyError_to_jlong_1d6cceb0<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZReplyError,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZReplyError_to_jlong_a51fa0c0<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZReplyError,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZReply_to_jlong_6eac4758<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZReply,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZSample_to_jlong_757bca9c<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZSample,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZSample_to_jlong_fea0199a<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZSample,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZScout_to_jlong_fad6332d<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZScout,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZSession_to_jlong_71741b2c<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZSession,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZSubscriber_to_jlong_6f91088e<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZSubscriber,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZTimestamp_to_jlong_6e790947<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZTimestamp,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZTimestamp_to_jlong_d34508ae<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZTimestamp,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZZBytes_to_jlong_8156b044<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZZBytes,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZZBytes_to_jlong_f37597b8<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZZBytes,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn ZZenohId_to_JByteArray_2e8b4538<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZZenohId,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        let __bytes: &[u8] = unsafe {
            ::core::slice::from_raw_parts(
                (&v as *const zenoh_flat::ZZenohId) as *const u8,
                ::core::mem::size_of::<zenoh_flat::ZZenohId>(),
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
pub(crate) unsafe fn jlong_to_Option_ZConfig_9937e91b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::ZConfig>>, __JniErr> {
    Ok({ if *v == 0 { None } else { Some(jlong_to_ZConfig_92724b10(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_ZEncoding_28edc9d4<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::ZEncoding>>, __JniErr> {
    Ok({ if *v == 0 { None } else { Some(jlong_to_ZEncoding_5667ec6b(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_ZKeyExpr_1e208261<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::ZKeyExpr>>, __JniErr> {
    Ok({ if *v == 0 { None } else { Some(jlong_to_ZKeyExpr_37f9dc18(env, v)?) } })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_ZKeyExpr_ffed55c9<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<zenoh_flat::ZKeyExpr>, __JniErr> {
    Ok({
        if *v == 0 {
            None
        } else {
            Some(*std::boxed::Box::from_raw(*v as *mut zenoh_flat::ZKeyExpr))
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_ZSample_f905b773<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<zenoh_flat::ZSample>, __JniErr> {
    Ok({
        if *v == 0 {
            None
        } else {
            Some(*std::boxed::Box::from_raw(*v as *mut zenoh_flat::ZSample))
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_Option_ZZBytes_afe91fd1<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<zenoh_flat::ZZBytes>, __JniErr> {
    Ok({
        if *v == 0 {
            None
        } else {
            Some(*std::boxed::Box::from_raw(*v as *mut zenoh_flat::ZZBytes))
        }
    })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZConfig_92724b10<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZConfig>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZConfig) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZEncoding_5667ec6b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZEncoding>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZEncoding) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZError_a71a127e<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZError>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZError) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZHello_5eb6fe2b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZHello>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZHello) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZKeyExpr_37f9dc18<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZKeyExpr>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZKeyExpr) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZLivelinessToken_64cf7e16<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZLivelinessToken>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZLivelinessToken) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZPublisher_68421aa7<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZPublisher>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZPublisher) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZQuerier_1b809fcd<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZQuerier>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZQuerier) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZQuery_19e50934<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZQuery>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZQuery) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZQueryable_ca27040b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZQueryable>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZQueryable) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZReplyError_1d6cceb0<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZReplyError>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZReplyError) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZReply_6eac4758<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZReply>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZReply) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZSample_757bca9c<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZSample>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZSample) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZScout_fad6332d<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZScout>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZScout) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZSession_71741b2c<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZSession>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZSession) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZSubscriber_6f91088e<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZSubscriber>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZSubscriber) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZTimestamp_d34508ae<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZTimestamp>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZTimestamp) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_ZZBytes_8156b044<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZZBytes>, __JniErr> {
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZZBytes) })
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn jlong_to_i64_fbf9a9bc<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<i64, __JniErr> {
    Ok(*v)
}
#[allow(non_snake_case, unused_mut, unused_variables, unused_braces, dead_code)]
pub(crate) unsafe fn static_ZEncoding_to_jlong_6f6d7e84<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: &'static zenoh_flat::ZEncoding,
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigClone<'a>(
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
    let c = match jlong_to_ZConfig_92724b10(&mut env, &c) {
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
    let __out = zenoh_flat::z_config_clone(&c);
    match ZConfig_to_jlong_92724b10(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigDefault<'a>(
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
    let __out = zenoh_flat::z_config_default();
    match ZConfig_to_jlong_92724b10(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigFromFile<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __out = match zenoh_flat::z_config_from_file(&path) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZConfig_to_jlong_92724b10(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigFromJson<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __out = match zenoh_flat::z_config_from_json(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZConfig_to_jlong_92724b10(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigFromJson5<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __out = match zenoh_flat::z_config_from_json5(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZConfig_to_jlong_92724b10(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigFromYaml<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __out = match zenoh_flat::z_config_from_yaml(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZConfig_to_jlong_92724b10(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigGetJson<'a>(
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
    let c = match jlong_to_ZConfig_92724b10(&mut env, &c) {
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
    let __out = zenoh_flat::z_config_get_json(&c, &key);
    let __out_s0 = match Result_String_ZError_to_String_57946770(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zConfigInsertJson5<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let mut c = match jlong_to_ZConfig_92724b10(&mut env, &c) {
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
    let __out = match zenoh_flat::z_config_insert_json5(&mut c, &key, &value) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationCbor<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_cbor();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationCdr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_cdr();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationCoapPayload<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_coap_payload();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationJavaSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_java_serialized_object();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_json();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationJsonPatchJson<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_json_patch_json();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationJsonSeq<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_json_seq();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationJsonpath<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_jsonpath();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationJwt<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_jwt();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_mp4();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationOctetStream<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_octet_stream();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationOpenmetricsText<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_openmetrics_text();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationProtobuf<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_protobuf();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationPythonSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_python_serialized_object();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationSoapXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_soap_xml();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationSql<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_sql();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationXWwwFormUrlencoded<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_x_www_form_urlencoded();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_xml();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_yaml();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingApplicationYang<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_application_yang();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingAudioAac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_audio_aac();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingAudioFlac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_audio_flac();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingAudioMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_audio_mp4();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingAudioOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_audio_ogg();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingAudioVorbis<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_audio_vorbis();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingClone<'a>(
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
    let e = match jlong_to_ZEncoding_5667ec6b(&mut env, &e) {
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
    let __out = zenoh_flat::z_encoding_clone(&e);
    match ZEncoding_to_jlong_5667ec6b(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingFromId<'a>(
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
    let __out = zenoh_flat::z_encoding_from_id(id, schema);
    match ZEncoding_to_jlong_5667ec6b(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingFromString<'a>(
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
    let __out = zenoh_flat::z_encoding_from_string(s);
    match ZEncoding_to_jlong_5667ec6b(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingId<'a>(
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
    let e = match jlong_to_ZEncoding_5667ec6b(&mut env, &e) {
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
    let __out = zenoh_flat::z_encoding_id(&e);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingImageBmp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_image_bmp();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingImageGif<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_image_gif();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingImageJpeg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_image_jpeg();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingImagePng<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_image_png();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingImageWebp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_image_webp();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingSchema<'a>(
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
    let e = match jlong_to_ZEncoding_5667ec6b(&mut env, &e) {
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
    let __out = zenoh_flat::z_encoding_schema(&e);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextCss<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_css();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextCsv<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_csv();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextHtml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_html();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextJavascript<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_javascript();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_json();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_json5();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextMarkdown<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_markdown();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextPlain<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_plain();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_xml();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingTextYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_text_yaml();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingToString<'a>(
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
    let e = match jlong_to_ZEncoding_5667ec6b(&mut env, &e) {
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
    let __out = zenoh_flat::z_encoding_to_string(&e);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoH261<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_h261();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoH263<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_h263();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoH264<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_h264();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoH265<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_h265();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoH266<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_h266();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_mp4();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_ogg();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoRaw<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_raw();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoVp8<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_vp8();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingVideoVp9<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_video_vp9();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingWithSchema<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
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
    let e = match jlong_to_ZEncoding_5667ec6b(&mut env, &e) {
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
    let __out = zenoh_flat::z_encoding_with_schema(&e, schema);
    match ZEncoding_to_jlong_5667ec6b(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingZenohBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_zenoh_bytes();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingZenohSerialized<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_zenoh_serialized();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zEncodingZenohString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
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
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen::lang::CachedIfaceMethod = ::prebindgen::lang::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/bytes/ZEncodingBuilderRaw";
    const __CB_DESCR: &str = "(JI)Ljava/lang/Object;";
    let __out = zenoh_flat::z_encoding_zenoh_string();
    let __obj1: jni::sys::jvalue = {
        let __enc1 = match i32_to_jint_a3e3b6ef(
            &mut env,
            zenoh_flat::z_encoding_id(__out),
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
        jni::sys::jvalue { i: __enc1 }
    };
    let __obj0: jni::sys::jvalue = {
        let __h0: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1],
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zErrorMessage<'a>(
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
    let e = match jlong_to_ZError_a71a127e(&mut env, &e) {
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
    let __out = zenoh_flat::z_error_message(&e);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zHelloLocators<'a>(
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
    let h = match jlong_to_ZHello_5eb6fe2b(&mut env, &h) {
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
    let __out = zenoh_flat::z_hello_locators(&h);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zHelloWhatami<'a>(
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
    let h = match jlong_to_ZHello_5eb6fe2b(&mut env, &h) {
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
    let __out = zenoh_flat::z_hello_whatami(&h);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zHelloZid<'a>(
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
    let h = match jlong_to_ZHello_5eb6fe2b(&mut env, &h) {
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
    let __out = zenoh_flat::z_hello_zid(&h);
    match ZZenohId_to_JByteArray_2e8b4538(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprAsStr<'a>(
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
    let ke = match jlong_to_ZKeyExpr_37f9dc18(&mut env, &ke) {
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
    let __out = zenoh_flat::z_keyexpr_as_str(&ke);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprAutocanonize<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __out = match zenoh_flat::z_keyexpr_autocanonize(s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZKeyExpr_to_jlong_37f9dc18(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprClone<'a>(
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
    let ke = match jlong_to_ZKeyExpr_37f9dc18(&mut env, &ke) {
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
    let __out = zenoh_flat::z_keyexpr_clone(&ke);
    match ZKeyExpr_to_jlong_37f9dc18(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprConcat<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __exp_a_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &a_1) {
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = match zenoh_flat::z_keyexpr_concat(&__folded_a, b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZKeyExpr_to_jlong_37f9dc18(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprIncludes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a_sel: jni::sys::jint,
    a_0: jni::objects::JString<'a>,
    a_1: jni::sys::jlong,
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
            return 0 as jni::sys::jboolean;
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
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_a_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &a_1) {
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
    let __folded_a = match {
        match __exp_a_sel {
            0i32 => {
                match __exp_a_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __exp_b_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &b_1) {
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = zenoh_flat::z_keyexpr_includes(&__folded_a, &__folded_b);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprIntersects<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a_sel: jni::sys::jint,
    a_0: jni::objects::JString<'a>,
    a_1: jni::sys::jlong,
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
            return 0 as jni::sys::jboolean;
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
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_a_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &a_1) {
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
    let __folded_a = match {
        match __exp_a_sel {
            0i32 => {
                match __exp_a_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __exp_b_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &b_1) {
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = zenoh_flat::z_keyexpr_intersects(&__folded_a, &__folded_b);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprJoin<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __exp_a_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &a_1) {
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = match zenoh_flat::z_keyexpr_join(&__folded_a, b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZKeyExpr_to_jlong_37f9dc18(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprRelationTo<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a_sel: jni::sys::jint,
    a_0: jni::objects::JString<'a>,
    a_1: jni::sys::jlong,
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
            return 0 as jni::sys::jint;
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
            return 0 as jni::sys::jint;
        }
    };
    let __exp_a_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &a_1) {
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
    let __folded_a = match {
        match __exp_a_sel {
            0i32 => {
                match __exp_a_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __exp_b_1 = match jlong_to_Option_ZKeyExpr_1e208261(&mut env, &b_1) {
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = zenoh_flat::z_keyexpr_relation_to(&__folded_a, &__folded_b);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprToString<'a>(
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
    let ke = match jlong_to_ZKeyExpr_37f9dc18(&mut env, &ke) {
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
    let __out = zenoh_flat::z_keyexpr_to_string(&ke);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zKeyexprTryFrom<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let __out = match zenoh_flat::z_keyexpr_try_from(s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZKeyExpr_to_jlong_37f9dc18(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zLivelinessDeclareSubscriber<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let callback = match JObject_to_impl_Fn_ZSample_Send_Sync_static_24e97b15(
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
    let __out = match zenoh_flat::z_liveliness_declare_subscriber(
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
                    zenoh_flat::z_error_message(&__de),
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
    match ZSubscriber_to_jlong_6f91088e(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zLivelinessDeclareToken<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = match zenoh_flat::z_liveliness_declare_token(
        &session,
        __folded_key_expr,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZLivelinessToken_to_jlong_64cf7e16(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zLivelinessGet<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_1e208261(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let callback = match JObject_to_impl_Fn_ZReply_Send_Sync_static_bd874b12(
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
    let __out = match zenoh_flat::z_liveliness_get(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zOpen<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let config: zenoh_flat::ZConfig = unsafe {
        *std::boxed::Box::from_raw(config as *mut zenoh_flat::ZConfig)
    };
    let __out = match zenoh_flat::z_open(config) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZSession_to_jlong_71741b2c(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zPublisherDelete<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match jlong_to_ZPublisher_68421aa7(&mut env, &publisher) {
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let __out = match zenoh_flat::z_publisher_delete(&publisher, __folded_attachment) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zPublisherPut<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match jlong_to_ZPublisher_68421aa7(&mut env, &publisher) {
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
        zenoh_flat::z_zbytes_from_vec(__exp_payload),
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
            zenoh_flat::z_encoding_from_id(__exp_encoding_id, __exp_encoding_schema),
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let __out = match zenoh_flat::z_publisher_put(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQuerierGet<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let querier = match jlong_to_ZQuerier_1b809fcd(&mut env, &querier) {
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
            zenoh_flat::z_encoding_from_id(__exp_encoding_id, __exp_encoding_schema),
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let callback = match JObject_to_impl_Fn_ZReply_Send_Sync_static_bd874b12(
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
    let __out = match zenoh_flat::z_querier_get(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryAcceptsReplies<'a>(
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
    let q = match jlong_to_ZQuery_19e50934(&mut env, &q) {
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
    let __out = zenoh_flat::z_query_accepts_replies(&q);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryAttachment<'a>(
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
    let q = match jlong_to_ZQuery_19e50934(&mut env, &q) {
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
    let __out = zenoh_flat::z_query_attachment(&q);
    match Option_ZZBytes_to_jlong_947bea0f(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryEncoding<'a>(
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
    let q = match jlong_to_ZQuery_19e50934(&mut env, &q) {
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
    let __out = zenoh_flat::z_query_encoding(&q);
    match Option_ZEncoding_to_jlong_28edc9d4(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryKeyexpr<'a>(
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
    let q = match jlong_to_ZQuery_19e50934(&mut env, &q) {
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
    let __out = zenoh_flat::z_query_keyexpr(&q);
    match ZKeyExpr_to_jlong_fbfa2238(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryParameters<'a>(
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
    let q = match jlong_to_ZQuery_19e50934(&mut env, &q) {
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
    let __out = zenoh_flat::z_query_parameters(&q);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryPayload<'a>(
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
    let q = match jlong_to_ZQuery_19e50934(&mut env, &q) {
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
    let __out = zenoh_flat::z_query_payload(&q);
    match Option_ZZBytes_to_jlong_947bea0f(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryReplyDelete<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_ZQuery_19e50934(&mut env, &query) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_1e208261(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let __out = match zenoh_flat::z_query_reply_delete(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryReplyError<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_ZQuery_19e50934(&mut env, &query) {
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
        zenoh_flat::z_zbytes_from_vec(__exp_payload),
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
            zenoh_flat::z_encoding_from_id(__exp_encoding_id, __exp_encoding_schema),
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
    let __out = match zenoh_flat::z_query_reply_error(
        &query,
        __folded_payload,
        __folded_encoding.as_ref(),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryReplySample<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_ZQuery_19e50934(&mut env, &query) {
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
    let __exp_sample_0 = match jlong_to_Option_ZSample_f905b773(&mut env, &sample_0) {
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
    let __out = match zenoh_flat::z_query_reply_sample(&query, __folded_sample) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zQueryReplySuccess<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let query = match jlong_to_ZQuery_19e50934(&mut env, &query) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_1e208261(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
        zenoh_flat::z_zbytes_from_vec(__exp_payload),
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
            zenoh_flat::z_encoding_from_id(__exp_encoding_id, __exp_encoding_schema),
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let __out = match zenoh_flat::z_query_reply_success(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zReplyErr<'a>(
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
    let r = match jlong_to_ZReply_6eac4758(&mut env, &r) {
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
    let __out = zenoh_flat::z_reply_err(&r);
    match Option_ZReplyError_to_jlong_5fea0243(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zReplyErrorEncoding<'a>(
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
    let e = match jlong_to_ZReplyError_1d6cceb0(&mut env, &e) {
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
    let __out = zenoh_flat::z_reply_error_encoding(&e);
    match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zReplyErrorPayload<'a>(
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
    let e = match jlong_to_ZReplyError_1d6cceb0(&mut env, &e) {
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
    let __out = zenoh_flat::z_reply_error_payload(&e);
    match ZZBytes_to_jlong_f37597b8(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zReplyIsOk<'a>(
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
    let r = match jlong_to_ZReply_6eac4758(&mut env, &r) {
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
    let __out = zenoh_flat::z_reply_is_ok(&r);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zReplyReplierEid<'a>(
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
    let r = match jlong_to_ZReply_6eac4758(&mut env, &r) {
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
    let __out = zenoh_flat::z_reply_replier_eid(&r);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zReplyReplierZid<'a>(
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
    let r = match jlong_to_ZReply_6eac4758(&mut env, &r) {
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
    let __out = zenoh_flat::z_reply_replier_zid(&r);
    match Option_ZZenohId_to_JByteArray_20202e50(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zReplySample<'a>(
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
    let r = match jlong_to_ZReply_6eac4758(&mut env, &r) {
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
    let __out = zenoh_flat::z_reply_sample(&r);
    match Option_ZSample_to_jlong_d70cb420(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleAttachment<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_attachment(&s);
    match Option_ZZBytes_to_jlong_947bea0f(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleCongestionControl<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_congestion_control(&s);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleDelete<'a>(
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    const __CB_FQN: &str = "io/zenoh/jni/sample/ZSampleBuilderRaw";
    const __CB_DESCR: &str = "(JJJIILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)Ljava/lang/Object;";
    let __out = zenoh_flat::z_sample_delete(
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
            zenoh_flat::z_encoding_id(zenoh_flat::z_sample_encoding(&__out)),
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
            zenoh_flat::z_sample_kind(&__out),
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
    let __obj5: jni::objects::JObject = match zenoh_flat::z_sample_timestamp(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __enc5 = match i64_to_jlong_fbf9a9bc(
                &mut env,
                zenoh_flat::z_timestamp_ntp64(__n0),
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
            zenoh_flat::z_sample_express(&__out),
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
            zenoh_flat::z_sample_priority(&__out),
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
            zenoh_flat::z_sample_congestion_control(&__out),
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
            zenoh_flat::z_sample_reliability(&__out),
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
        let __enc11 = match Option_ZZenohId_to_JByteArray_20202e50(
            &mut env,
            zenoh_flat::z_sample_source_zid(&__out),
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
            zenoh_flat::z_sample_source_eid(&__out),
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
            zenoh_flat::z_sample_source_sn(&__out),
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
        let __h0: jni::sys::jlong = match ZKeyExpr_to_jlong_fbfa2238(
            &mut env,
            zenoh_flat::z_sample_key_expr(&__out),
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
        let __h1: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
            &mut env,
            zenoh_flat::z_sample_payload(&__out),
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
        let __h2: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(
            &mut env,
            zenoh_flat::z_sample_encoding(&__out),
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
    let __obj9: jni::objects::JObject = match zenoh_flat::z_sample_attachment(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __h9: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(&mut env, __n0) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleEncoding<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_encoding(&s);
    match ZEncoding_to_jlong_77133a31(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleExpress<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_express(&s);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleKeyExpr<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_key_expr(&s);
    match ZKeyExpr_to_jlong_fbfa2238(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleKind<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_kind(&s);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSamplePayload<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_payload(&s);
    match ZZBytes_to_jlong_f37597b8(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSamplePriority<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_priority(&s);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSamplePut<'a>(
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
        zenoh_flat::z_zbytes_from_vec(__exp_payload),
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
            zenoh_flat::z_encoding_from_id(__exp_encoding_id, __exp_encoding_schema),
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    const __CB_FQN: &str = "io/zenoh/jni/sample/ZSampleBuilderRaw";
    const __CB_DESCR: &str = "(JJJIILjava/lang/Long;ZIILjava/lang/Long;I[BIJ)Ljava/lang/Object;";
    let __out = zenoh_flat::z_sample_put(
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
            zenoh_flat::z_encoding_id(zenoh_flat::z_sample_encoding(&__out)),
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
            zenoh_flat::z_sample_kind(&__out),
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
    let __obj5: jni::objects::JObject = match zenoh_flat::z_sample_timestamp(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __enc5 = match i64_to_jlong_fbf9a9bc(
                &mut env,
                zenoh_flat::z_timestamp_ntp64(__n0),
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
            zenoh_flat::z_sample_express(&__out),
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
            zenoh_flat::z_sample_priority(&__out),
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
            zenoh_flat::z_sample_congestion_control(&__out),
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
            zenoh_flat::z_sample_reliability(&__out),
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
        let __enc11 = match Option_ZZenohId_to_JByteArray_20202e50(
            &mut env,
            zenoh_flat::z_sample_source_zid(&__out),
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
            zenoh_flat::z_sample_source_eid(&__out),
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
            zenoh_flat::z_sample_source_sn(&__out),
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
        let __h0: jni::sys::jlong = match ZKeyExpr_to_jlong_fbfa2238(
            &mut env,
            zenoh_flat::z_sample_key_expr(&__out),
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
        let __h1: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(
            &mut env,
            zenoh_flat::z_sample_payload(&__out),
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
        let __h2: jni::sys::jlong = match ZEncoding_to_jlong_77133a31(
            &mut env,
            zenoh_flat::z_sample_encoding(&__out),
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
    let __obj9: jni::objects::JObject = match zenoh_flat::z_sample_attachment(&__out) {
        ::core::option::Option::Some(__n0) => {
            let __h9: jni::sys::jlong = match ZZBytes_to_jlong_f37597b8(&mut env, __n0) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleReliability<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_reliability(&s);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleSourceEid<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_source_eid(&s);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleSourceSn<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_source_sn(&s);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleSourceZid<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_source_zid(&s);
    match Option_ZZenohId_to_JByteArray_20202e50(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSampleTimestamp<'a>(
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
    let s = match jlong_to_ZSample_757bca9c(&mut env, &s) {
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
    let __out = zenoh_flat::z_sample_timestamp(&s);
    match Option_ZTimestamp_to_jlong_804b81ce(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zScout<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
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
    let config = match jlong_to_Option_ZConfig_9937e91b(&mut env, &config) {
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
    let callback = match JObject_to_impl_Fn_ZHello_Send_Sync_static_10862af4(
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
    let __out = match zenoh_flat::z_scout(
        whatami,
        config.as_deref(),
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZScout_to_jlong_fad6332d(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionDeclareKeyexpr<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __out = match zenoh_flat::z_session_declare_keyexpr(&session, key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
    match ZKeyExpr_to_jlong_37f9dc18(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionDeclarePublisher<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = match zenoh_flat::z_session_declare_publisher(
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
                    zenoh_flat::z_error_message(&__de),
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
    match ZPublisher_to_jlong_68421aa7(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionDeclareQuerier<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let __out = match zenoh_flat::z_session_declare_querier(
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
                    zenoh_flat::z_error_message(&__de),
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
    match ZQuerier_to_jlong_1b809fcd(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionDeclareQueryable<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let callback = match JObject_to_impl_Fn_ZQuery_Send_Sync_static_119148c0(
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
    let __out = match zenoh_flat::z_session_declare_queryable(
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
                    zenoh_flat::z_error_message(&__de),
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
    match ZQueryable_to_jlong_ca27040b(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionDeclareSubscriber<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_ffed55c9(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
    let callback = match JObject_to_impl_Fn_ZSample_Send_Sync_static_24e97b15(
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
    let __out = match zenoh_flat::z_session_declare_subscriber(
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
                    zenoh_flat::z_error_message(&__de),
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
    match ZSubscriber_to_jlong_6f91088e(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionDelete<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_1e208261(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let __out = match zenoh_flat::z_session_delete(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionGet<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_1e208261(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
            zenoh_flat::z_encoding_from_id(__exp_encoding_id, __exp_encoding_schema),
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let callback = match JObject_to_impl_Fn_ZReply_Send_Sync_static_bd874b12(
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
    let __out = match zenoh_flat::z_session_get(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionPeersZid<'a>(
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
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __out = zenoh_flat::z_session_peers_zid(&session);
    match Vec_ZZenohId_to_JObject_ef32ce89(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionPut<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __exp_key_expr_1 = match jlong_to_Option_ZKeyExpr_1e208261(
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
                        zenoh_flat::z_keyexpr_try_from(__p0)
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
        zenoh_flat::z_zbytes_from_vec(__exp_payload),
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
            zenoh_flat::z_encoding_from_id(__exp_encoding_id, __exp_encoding_schema),
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
            (::core::result::Result::Ok(zenoh_flat::z_zbytes_from_vec(__inner)))
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
    let __out = match zenoh_flat::z_session_put(
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
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionRoutersZid<'a>(
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
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __out = zenoh_flat::z_session_routers_zid(&session);
    match Vec_ZZenohId_to_JObject_ef32ce89(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionUndeclareKeyexpr<'a>(
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
    const __SINK_FQN: &str = "io/zenoh/jni/errors/ZErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;";
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let key_expr: zenoh_flat::ZKeyExpr = unsafe {
        *std::boxed::Box::from_raw(key_expr as *mut zenoh_flat::ZKeyExpr)
    };
    let __out = match zenoh_flat::z_session_undeclare_keyexpr(&session, key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match String_to_JString_c7f3ca43(
                    &mut env,
                    zenoh_flat::z_error_message(&__de),
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zSessionZid<'a>(
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
    let session = match jlong_to_ZSession_71741b2c(&mut env, &session) {
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
    let __out = zenoh_flat::z_session_zid(&session);
    match ZZenohId_to_JByteArray_2e8b4538(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zTimestampId<'a>(
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
    let t = match jlong_to_ZTimestamp_d34508ae(&mut env, &t) {
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
    let __out = zenoh_flat::z_timestamp_id(&t);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zTimestampNtp64<'a>(
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
    let t = match jlong_to_ZTimestamp_d34508ae(&mut env, &t) {
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
    let __out = zenoh_flat::z_timestamp_ntp64(&t);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zZbytesAsBytes<'a>(
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
    let z = match jlong_to_ZZBytes_8156b044(&mut env, &z) {
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
    let __out = zenoh_flat::z_zbytes_as_bytes(&z);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zZbytesClone<'a>(
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
    let z = match jlong_to_ZZBytes_8156b044(&mut env, &z) {
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
    let __out = zenoh_flat::z_zbytes_clone(&z);
    match ZZBytes_to_jlong_8156b044(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zZbytesFromVec<'a>(
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
    let __out = zenoh_flat::z_zbytes_from_vec(bytes);
    match ZZBytes_to_jlong_8156b044(&mut env, __out) {
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zZbytesToBytes<'a>(
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
    let z = match jlong_to_ZZBytes_8156b044(&mut env, &z) {
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
    let __out = zenoh_flat::z_zbytes_to_bytes(&z);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zZenohIdToBytes<'a>(
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
    let z = match JByteArray_to_ZZenohId_2e8b4538(&mut env, &z) {
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
    let __out = zenoh_flat::z_zenoh_id_to_bytes(&z);
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zZenohIdToString<'a>(
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
    let z = match JByteArray_to_ZZenohId_2e8b4538(&mut env, &z) {
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
    let __out = zenoh_flat::z_zenoh_id_to_string(&z);
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
