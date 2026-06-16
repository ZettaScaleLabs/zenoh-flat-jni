//! Build script for the zenoh-flat JNI/Kotlin layer.
//!
//! Reads the `#[prebindgen]` items captured by `zenoh-flat` and drives them
//! through the [`prebindgen::lang::JniGen`] adapter to produce:
//!   * `src/generated_bindings.rs` — the Rust-side JNI wrappers (included by
//!     `src/lib.rs`), and
//!   * `kotlin/generated/**` — the matching typed Kotlin classes.
//!
//! ## Model
//!
//! This is the **flat tier**: every `#[prebindgen]` free function is declared
//! as a package function ([`JniGen::package_fun`]) under a module namespace
//! ([`JniGen::package`]). Opaque handles stay typed Kotlin classes derived from
//! `NativeHandle` (locked, closeable) via [`JniGen::ptr_class`].
//!
//! A type's CANONICAL representation — its input ([`JniGen::ptr_class_input`] /
//! `..._direct`) and output ([`JniGen::ptr_class_output`] / `..._direct`) — is
//! declared once on the `ptr_class` and AUTO-APPLIES to every matching param /
//! return. Read accessors are declared with [`JniGen::class_accessor`] (emitted
//! as instance methods of their class) and the per-fn `input_direct` /
//! `output_direct` overrides opt out.
//!
//! Errors are delivered through the per-call `onError` callback (no Rust-side
//! JVM throw): `Error` (zenoh's native error) is the `E` of every fallible
//! `Result<_, Error>`, and its canonical output (`error_get_message -> String`)
//! auto-applies to the `E` position so `onError` receives the message string.
//!
//! Names mirror zenoh-flat's de-prefixed Rust identifiers one-to-one
//! (`KeyExpr`, `Session`, `keyexpr_new_try_from`, `open`, …); the Kotlin-side
//! names are derived from them automatically.

use prebindgen::core::Registry;
use prebindgen::lang::JniGen;
use syn::parse_quote as pq;

fn fail(context: &str, err: impl std::fmt::Display) -> ! {
    eprintln!("error: prebindgen jnigen {context}: {err}");
    std::process::exit(1);
}

fn main() {
    // The builder is type-state: each call returns a different `JniGen<S>`, so
    // the configuration is one fluent chain. It is split in two only to let the
    // long, modifier-free encoding-constant list collapse into a `for` loop
    // (see below); `package(...)` is available on every state, so the tail
    // chain resumes seamlessly afterwards.
    let mut jni = JniGen::new()
        .source_module(pq!(zenoh_flat)) // module prefix for prebindgen-marked items
        .package_prefix("io.zenoh.jni") // base package of the generated JNI bindings
        // ── Errors ────────────────────────────────────────────────────────
        // `Error` is the `E` of every fallible `Result<_, Error>`. Declaring it
        // as a `ptr_class` lets the `Result<T, Error>` resolver find its
        // converters; the handle never actually crosses (the extern peels the
        // `Result` and delivers the message via the error callback). Its
        // canonical output is the message string (1 leaf), auto-applied to the
        // `E` of every such `Result` — i.e. the `onError` callback's argument.
        // `error_get_message` is that output's accessor (a class method named
        // `message`, referenced by `.ptr_class_output("message")`).
        .package("errors")
        .ptr_class(pq!(Error))
        .class_accessor(pq!(error_get_message), "message")
        .ptr_class_output("message")
        // ── Key expressions ──────────────────────────────────────────────
        // Canonical input: a key-expr param accepts EITHER a String (built via
        // `keyexpr_new_try_from`) OR an existing handle (identity), selector-
        // dispatched. Canonical output: the handle only (identity, 1 raw jlong);
        // the string form stays an on-demand accessor method (`getStr`).
        .package("keyexpr")
        .ptr_class(pq!(KeyExpr))
        // Read accessors → instance methods on the KeyExpr class.
        .class_accessor(pq!(keyexpr_get_str), "getStr")
        .class_accessor(pq!(keyexpr_new_clone), "newClone")
        .class_accessor(pq!(keyexpr_to_string), "toStr")
        .ptr_class_input(pq!(keyexpr_new_try_from))
        .ptr_class_input_direct()
        .ptr_class_output_direct()
        // Factories returning `Result<KeyExpr, Error>` (handle return + error callback).
        .package_fun(pq!(keyexpr_new_try_from))
        .package_fun(pq!(keyexpr_new_autocanonize))
        .package_fun(pq!(keyexpr_new_join))
        .package_fun(pq!(keyexpr_new_concat))
        // Consumers: a/b key-expr params auto-constructed by the canonical input.
        .package_fun(pq!(keyexpr_intersects))
        .package_fun(pq!(keyexpr_includes))
        .package_fun(pq!(keyexpr_relation_to))
        .enum_class(pq!(SetIntersectionLevel))
        // ── Config + ZenohId ──────────────────────────────────────────────
        .package("config")
        .ptr_class(pq!(Config))
        .class_accessor(pq!(config_get_json), "getJson")
        .class_accessor(pq!(config_new_clone), "newClone")
        .package_fun(pq!(config_new_default))
        .package_fun(pq!(config_new_from_file))
        .package_fun(pq!(config_new_from_json))
        .package_fun(pq!(config_new_from_json5))
        .package_fun(pq!(config_new_from_yaml))
        .package_fun(pq!(config_insert_json5))
        .enum_class(pq!(WhatAmI))
        // `ZenohId` is a `Copy` value (zenoh's `ZenohId`, repr(transparent)), so
        // it crosses as a raw byte-blob `ByteArray` rather than a closeable jlong
        // handle. `Vec<ZenohId>` (session peers/routers) folds each element WHOLE
        // as the typed `ZenohId` value class. Its read accessors become methods
        // on the value class (receiver = `this.bytes`).
        .value_class(pq!(ZenohId))
        .class_accessor(pq!(zenoh_id_to_bytes), "toBytes")
        .class_accessor(pq!(zenoh_id_to_string), "toStr")
        // ── Scouting ──────────────────────────────────────────────────────
        // Canonical output: the scout callback decomposes a `Hello` into its
        // three read fields in ONE crossing (no handle — read-only). Auto-applies
        // to `scout`'s `Fn(Hello)`.
        .package("scouting")
        .ptr_class(pq!(Hello))
        .class_accessor(pq!(hello_get_whatami), "whatami") // WhatAmI enum -> Int
        .class_accessor(pq!(hello_get_zid), "zid") // ZenohId value class -> ByteArray
        .class_accessor(pq!(hello_get_locators), "locators") // Vec<String> -> List<String>
        .ptr_class_output("whatami")
        .ptr_class_output("zid")
        .ptr_class_output("locators")
        .ptr_class(pq!(Scout))
        .package_fun(pq!(scout))
        // ── Logger ────────────────────────────────────────────────────────
        .package("logger")
        .package_fun(pq!(init_android_logs))
        .package_fun(pq!(try_init_zenoh_logs_from_env))
        .package_fun(pq!(init_zenoh_logs_from_env_or))
        // ── QoS enums ─────────────────────────────────────────────────────
        .package("qos")
        .enum_class(pq!(Reliability))
        .enum_class(pq!(Priority))
        .enum_class(pq!(CongestionControl))
        // ── Bytes: ZBytes ─────────────────────────────────────────────────
        // Canonical input: `payload`/`attachment` params accept a `ByteArray`
        // (built via `zbytes_new_from_vec`). Canonical output: the handle only
        // (identity) — the bytes are heavy and fetched on demand via
        // `zbytesAsBytes` (one borrow-copy).
        .package("bytes")
        .ptr_class(pq!(ZBytes))
        .class_accessor(pq!(zbytes_as_bytes), "asBytes")
        .class_accessor(pq!(zbytes_to_bytes), "toBytes")
        .class_accessor(pq!(zbytes_new_clone), "newClone")
        .ptr_class_input(pq!(zbytes_new_from_vec))
        .ptr_class_output_direct()
        // Keep the factory exported with a raw-handle return (`output_direct`
        // opts out of the canonical ByteArray decomposition).
        .package_fun(pq!(zbytes_new_from_vec))
        .output_direct()
        // ── Bytes: Encoding ───────────────────────────────────────────────
        // Canonical input: encoding params cross as their decomposed value
        // `(id: i32, schema: Option<String>)` (built via `encoding_new_from_id`)
        // — cheap primitives, no per-call String parse, no native handle.
        // Canonical output: the handle (identity) + id (raw jint), both free
        // jvalue slots; schema and the canonical string stay on-demand accessors.
        .ptr_class(pq!(Encoding))
        .class_accessor(pq!(encoding_get_id), "id")
        .class_accessor(pq!(encoding_get_schema), "getSchema")
        .class_accessor(pq!(encoding_to_string), "toStr")
        .class_accessor(pq!(encoding_new_clone), "newClone")
        .class_accessor(pq!(encoding_new_with_schema), "withSchema")
        .ptr_class_input(pq!(encoding_new_from_id))
        .ptr_class_output_direct()
        .ptr_class_output("id")
        // Factories: keep the handle return (canonical output would decompose it).
        .package_fun(pq!(encoding_new_from_string))
        .output_direct()
        .package_fun(pq!(encoding_new_from_id))
        .output_direct();

    // Predefined encoding constants — each is a nullary `() -> &'static Encoding`
    // with no per-fn override, so they collapse into one loop instead of ~50
    // identical `.package_fun(...)` lines. `package_fun` on `Function` returns
    // `JniGen<Function>`, so the reassignment keeps a stable type.
    for ctor in [
        pq!(encoding_const_zenoh_bytes),
        pq!(encoding_const_zenoh_string),
        pq!(encoding_const_zenoh_serialized),
        pq!(encoding_const_application_octet_stream),
        pq!(encoding_const_text_plain),
        pq!(encoding_const_application_json),
        pq!(encoding_const_text_json),
        pq!(encoding_const_application_cdr),
        pq!(encoding_const_application_cbor),
        pq!(encoding_const_application_yaml),
        pq!(encoding_const_text_yaml),
        pq!(encoding_const_text_json5),
        pq!(encoding_const_application_python_serialized_object),
        pq!(encoding_const_application_protobuf),
        pq!(encoding_const_application_java_serialized_object),
        pq!(encoding_const_application_openmetrics_text),
        pq!(encoding_const_image_png),
        pq!(encoding_const_image_jpeg),
        pq!(encoding_const_image_gif),
        pq!(encoding_const_image_bmp),
        pq!(encoding_const_image_webp),
        pq!(encoding_const_application_xml),
        pq!(encoding_const_application_x_www_form_urlencoded),
        pq!(encoding_const_text_html),
        pq!(encoding_const_text_xml),
        pq!(encoding_const_text_css),
        pq!(encoding_const_text_javascript),
        pq!(encoding_const_text_markdown),
        pq!(encoding_const_text_csv),
        pq!(encoding_const_application_sql),
        pq!(encoding_const_application_coap_payload),
        pq!(encoding_const_application_json_patch_json),
        pq!(encoding_const_application_json_seq),
        pq!(encoding_const_application_jsonpath),
        pq!(encoding_const_application_jwt),
        pq!(encoding_const_application_mp4),
        pq!(encoding_const_application_soap_xml),
        pq!(encoding_const_application_yang),
        pq!(encoding_const_audio_aac),
        pq!(encoding_const_audio_flac),
        pq!(encoding_const_audio_mp4),
        pq!(encoding_const_audio_ogg),
        pq!(encoding_const_audio_vorbis),
        pq!(encoding_const_video_h261),
        pq!(encoding_const_video_h263),
        pq!(encoding_const_video_h264),
        pq!(encoding_const_video_h265),
        pq!(encoding_const_video_h266),
        pq!(encoding_const_video_mp4),
        pq!(encoding_const_video_ogg),
        pq!(encoding_const_video_raw),
        pq!(encoding_const_video_vp8),
        pq!(encoding_const_video_vp9),
    ] {
        jni = jni.package_fun(ctor);
    }

    let jni = jni
        // ── Time ──────────────────────────────────────────────────────────
        // Canonical output: a timestamp is its NTP64 value (`timestamp_get_ntp64`
        // -> i64 -> Long, 1 leaf); nested in a `Sample` it contributes that Long.
        .package("time")
        .ptr_class(pq!(Timestamp))
        .class_accessor(pq!(timestamp_get_ntp64), "ntp64")
        .class_accessor(pq!(timestamp_get_id), "getId")
        .ptr_class_output("ntp64")
        // ── Sample ────────────────────────────────────────────────────────
        // Canonical INPUT: identity only — a `Sample` param takes the owned
        // handle directly. (The full-options constructors carry `Option<ptr_class>`
        // params the recursive-input fold can't build through, so a `Sample` is
        // built via the `sample_new_*` constructors below and consumed by handle.)
        // Canonical OUTPUT: the full sample decomposed in ONE crossing. Each record
        // is unwrapped per its return type's own canonical output (key_expr ->
        // handle+String, payload/attachment -> ByteArray, encoding -> String,
        // timestamp -> Long?, kind/priority/congestion/reliability -> Int, express
        // -> Boolean, source_zid -> ByteArray?, source_eid -> Int, source_sn ->
        // Long). Auto-applies to every (non-Result) `Sample` return.
        .package("sample")
        .enum_class(pq!(SampleKind))
        .ptr_class(pq!(Sample))
        // All sample getters are record sources AND instance methods on the
        // Sample class; decomposition happens via the canonical output below.
        .class_accessor(pq!(sample_get_key_expr), "keyExpr")
        .class_accessor(pq!(sample_get_payload), "payload")
        .class_accessor(pq!(sample_get_encoding), "encoding")
        .class_accessor(pq!(sample_get_kind), "kind")
        .class_accessor(pq!(sample_get_timestamp), "timestamp")
        .class_accessor(pq!(sample_get_express), "express")
        .class_accessor(pq!(sample_get_priority), "priority")
        .class_accessor(pq!(sample_get_congestion_control), "congestionControl")
        .class_accessor(pq!(sample_get_attachment), "attachment")
        .class_accessor(pq!(sample_get_reliability), "reliability")
        .class_accessor(pq!(sample_get_source_zid), "sourceZid")
        .class_accessor(pq!(sample_get_source_eid), "sourceEid")
        .class_accessor(pq!(sample_get_source_sn), "sourceSn")
        .ptr_class_input_direct()
        .ptr_class_output("keyExpr")
        .ptr_class_output("payload")
        .ptr_class_output("encoding")
        .ptr_class_output("kind")
        .ptr_class_output("timestamp")
        .ptr_class_output("express")
        .ptr_class_output("priority")
        .ptr_class_output("congestionControl")
        .ptr_class_output("attachment")
        .ptr_class_output("reliability")
        .ptr_class_output("sourceZid")
        .ptr_class_output("sourceEid")
        .ptr_class_output("sourceSn")
        // Standalone sample constructors (callable from Kotlin); consumed by handle.
        .package_fun(pq!(sample_new_put))
        .package_fun(pq!(sample_new_delete))
        // ── Pub/Sub ───────────────────────────────────────────────────────
        // key_expr / payload / attachment / encoding params are auto-constructed
        // by their types' canonical inputs (no per-fn calls).
        .package("pubsub")
        .ptr_class(pq!(Publisher))
        .package_fun(pq!(publisher_put))
        .package_fun(pq!(publisher_delete))
        .ptr_class(pq!(Subscriber))
        // ── Query / Queryable / Querier ───────────────────────────────────
        .package("query")
        .ptr_class(pq!(Queryable))
        .ptr_class(pq!(Querier))
        .package_fun(pq!(querier_get))
        .enum_class(pq!(ReplyKeyExpr))
        .enum_class(pq!(QueryTarget))
        .enum_class(pq!(ConsolidationMode))
        // Canonical output: the queryable callback decomposes a `Query` into BOTH
        // its read fields AND the owned handle (identity) in ONE crossing — the
        // `output_direct` keeps the handle so the consumer can reply
        // (`query_reply_*`) after the callback returns; a query must outlive its
        // callback to be answered.
        //
        // ORDER MATTERS: `.ptr_class_output_direct()` MUST be LAST. The root
        // identity moves the owned query while the nested KeyExpr identity (from
        // `query_get_keyexpr`) clones from a borrow of the same query; identity
        // leaves are emitted in declaration order, so the root move must follow
        // the nested borrow to avoid a "use of moved value" error.
        .ptr_class(pq!(Query))
        .class_accessor(pq!(query_get_keyexpr), "keyExpr")
        .class_accessor(pq!(query_get_parameters), "parameters")
        .class_accessor(pq!(query_get_payload), "payload")
        .class_accessor(pq!(query_get_encoding), "encoding")
        .class_accessor(pq!(query_get_attachment), "attachment")
        .class_accessor(pq!(query_get_accepts_replies), "acceptsReplies")
        .ptr_class_output("keyExpr")
        .ptr_class_output("parameters")
        .ptr_class_output("payload")
        .ptr_class_output("encoding")
        .ptr_class_output("attachment")
        .ptr_class_output("acceptsReplies")
        .ptr_class_output_direct()
        // Reply ops on the owned/borrowed query handle.
        .package_fun(pq!(query_reply_success))
        .package_fun(pq!(query_reply_error))
        .package_fun(pq!(query_reply_delete))
        // `query_reply_sample` takes the sample by owned handle (Sample's
        // canonical input is identity — see the sample package above).
        .package_fun(pq!(query_reply_sample))
        // ── Reply / ReplyError ────────────────────────────────────────────
        // ReplyError canonical output: a failed reply's error decomposed in one
        // crossing — payload -> ByteArray, encoding -> String.
        .ptr_class(pq!(ReplyError))
        .class_accessor(pq!(reply_error_get_payload), "payload")
        .class_accessor(pq!(reply_error_get_encoding), "encoding")
        .ptr_class_output("payload")
        .ptr_class_output("encoding")
        // Reply canonical output: the whole reply decomposed in ONE crossing
        // (PRODUCT model — both arms' leaves always present, the not-taken arm's
        // are null). replier zid/eid + the is_ok discriminator, then the ok arm
        // splices the full sample and the err arm splices payload/encoding.
        // Auto-applies to the `Fn(Reply)` callbacks of `session_get` /
        // `querier_get` / liveliness get; no identity record, so no `Reply`
        // handle crosses.
        .ptr_class(pq!(Reply))
        // Record sources are class methods — `reply.sample()`'s standalone
        // export is therefore the cloned-handle form.
        .class_accessor(pq!(reply_get_replier_zid), "replierZid")
        .class_accessor(pq!(reply_get_replier_eid), "replierEid")
        .class_accessor(pq!(reply_is_ok), "isOk")
        .class_accessor(pq!(reply_get_sample), "sample")
        .class_accessor(pq!(reply_get_err), "err")
        .ptr_class_output("replierZid")
        .ptr_class_output("replierEid")
        .ptr_class_output("isOk")
        .ptr_class_output("sample")
        .ptr_class_output("err")
        // ── Liveliness + Session ──────────────────────────────────────────
        // `LivelinessToken` is just an opaque handle; the liveliness operations
        // (`liveliness_*`) are declared under the active `session` package below,
        // alongside the session API they extend.
        .package("liveliness")
        .ptr_class(pq!(LivelinessToken))
        .package("session")
        .ptr_class(pq!(Session))
        .class_accessor(pq!(session_get_zid), "getZid")
        .package_fun(pq!(open))
        .package_fun(pq!(session_declare_publisher))
        .package_fun(pq!(session_put))
        .package_fun(pq!(session_delete))
        .package_fun(pq!(session_declare_subscriber))
        .package_fun(pq!(session_declare_querier))
        .package_fun(pq!(session_declare_queryable))
        .package_fun(pq!(session_declare_keyexpr))
        // Undeclaring needs the declared handle, not a string — opt its key_expr
        // param out of the canonical (String-building) input.
        .package_fun(pq!(session_undeclare_keyexpr))
        .input_direct(pq!(key_expr))
        .package_fun(pq!(session_get))
        // `Vec<ZenohId>`: ZenohId is a value class, so these return `List<ZenohId>`
        // via the normal Vec converter.
        .package_fun(pq!(session_get_peers_zid))
        .package_fun(pq!(session_get_routers_zid))
        // Liveliness ops (key_expr params auto-constructed by the canonical input).
        .package_fun(pq!(liveliness_declare_token))
        .package_fun(pq!(liveliness_get))
        .package_fun(pq!(liveliness_declare_subscriber));

    // ── Outputs ───────────────────────────────────────────────────────────
    // Run the configured adapter over zenoh-flat's captured `#[prebindgen]`
    // items and write both generated artifacts.
    let source = prebindgen::Source::new(zenoh_flat::PREBINDGEN_OUT_DIR);
    let mut registry = match Registry::from_items(source.items_all()) {
        Ok(registry) => registry,
        Err(err) => fail("scan failed", err),
    };

    // Rust bindings → src/generated_bindings.rs. Absolute path so the file lands
    // in the source tree (committed to git and included by `src/lib.rs`).
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let rust_dest = std::path::Path::new(&crate_dir)
        .join("src")
        .join("generated_bindings.rs");
    let rust_path = match registry.write_rust(&jni, &rust_dest) {
        Ok(path) => path,
        Err(err) => fail("write_rust failed", err),
    };
    println!("cargo:warning=Generated bindings at: {}", rust_path.display());

    // ── Kotlin bindings → kotlin/generated/ ─────────────────────────────
    // The runtime module's Gradle source set picks these up via
    // `kotlin.srcDir("$rootDir/zenoh-flat-jni/kotlin/generated")`.
    let kotlin_root = std::path::Path::new(&crate_dir)
        .join("kotlin")
        .join("generated");
    // Remove stale generated files so package moves don't leave old classes
    // behind (e.g. io/zenoh/jni/* and io/zenoh/jni/<subpackage>/* side by side).
    if let Err(err) = std::fs::remove_dir_all(&kotlin_root) {
        if err.kind() != std::io::ErrorKind::NotFound {
            fail("cleanup kotlin/generated failed", err);
        }
    }
    for path in match jni.write_kotlin(&registry, &kotlin_root) {
        Ok(paths) => paths,
        Err(err) => fail("write_kotlin failed", err),
    } {
        println!("cargo:warning=Wrote {}", path.display());
    }
}
