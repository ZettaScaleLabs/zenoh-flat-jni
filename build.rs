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
//! as a package function ([`JniGen::fun`]) under a module namespace
//! ([`JniGen::package`]). Opaque handles stay typed Kotlin classes derived from
//! `NativeHandle` (locked, closeable) via [`JniGen::ptr_class`].
//!
//! A type's default flatten shape — its input ([`JniGen::flatten_input`]) and
//! output ([`JniGen::flatten_output`]) — is declared once on the `ptr_class` and
//! AUTO-APPLIES to every matching param / return. Class members are declared
//! with [`JniGen::accessor`] / [`JniGen::method`] / [`JniGen::constructor`]
//! (emitted as instance methods / companion factories of their class) and the
//! per-fn `flatten_input_suppress` / `flatten_output_suppress` overrides opt out.
//!
//! Errors are delivered through the per-call `onError` callback (no Rust-side
//! JVM throw): `Error` (zenoh's native error) is the `E` of every fallible
//! `Result<_, Error>`, and its default output flatten (`error_get_message ->
//! String`) auto-applies to the `E` position so `onError` receives the message.
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

/// Companion-factory name for a predefined `encoding_const_<x>` constant:
/// strip the `encoding_const_` prefix and lower-camelCase the rest
/// (`encoding_const_text_plain` → `textPlain`).
fn const_name(ident: &syn::Ident) -> String {
    let s = ident.to_string();
    let leaf = s.strip_prefix("encoding_const_").unwrap_or(&s);
    let mut out = String::new();
    let mut upper = false;
    for c in leaf.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
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
        // Every generated native call routes through `JNINative`; trigger our own
        // loader from its static initializer so the native library is loaded
        // transparently before any extern resolves (consumers never load it).
        .jni_native_init("io.zenoh.jni.NativeLibrary.ensureLoaded()")
        // ── Errors ────────────────────────────────────────────────────────
        // `Error` is the `E` of every fallible `Result<_, Error>`. Declaring it
        // as a `ptr_class` lets the `Result<T, Error>` resolver find its
        // converters; the handle never actually crosses (the extern peels the
        // `Result` and delivers the message via the error callback). Its
        // canonical output is the message string (1 leaf), auto-applied to the
        // `E` of every such `Result` — i.e. the `onError` callback's argument.
        // `error_get_message` is that output's accessor (a class method named
        // `message`, referenced by `.flatten_output().field("message")`).
        .package("errors")
        .ptr_class(pq!(Error))
        .accessor(pq!(error_get_message), "message")
        .flatten_output().field("message")
        // ── Key expressions ──────────────────────────────────────────────
        // Canonical input: a key-expr param accepts EITHER a String (built via
        // `keyexpr_new_try_from`) OR an existing handle (identity), selector-
        // dispatched. Canonical output: the handle only (identity, 1 raw jlong);
        // the string form stays an on-demand accessor method (`getStr`).
        .package("keyexpr")
        .ptr_class(pq!(KeyExpr))
        // Read accessors → instance methods on the KeyExpr class.
        .accessor(pq!(keyexpr_get_str), "getStr")
        .accessor(pq!(keyexpr_new_clone), "newClone")
        .accessor(pq!(keyexpr_to_string), "toStr")
        // Constructors → companion factories returning `Result<KeyExpr, Error>`;
        // `tryFrom` is also the build-from-String input variant.
        .constructor(pq!(keyexpr_new_try_from), "tryFrom")
        .constructor(pq!(keyexpr_new_autocanonize), "autocanonize")
        .constructor(pq!(keyexpr_new_join), "join")
        .constructor(pq!(keyexpr_new_concat), "concat")
        // Input flatten: a key-expr param accepts EITHER a String (built via
        // `tryFrom`) OR an existing handle (self). Output flatten: the handle
        // only (the string form stays the `getStr` accessor method).
        .flatten_input().variant("tryFrom").variant_self()
        .flatten_output().field_self()
        // Consumer methods: the receiver key-expr is `this`; the other param
        // accepts a String (built via the input flatten above).
        .method(pq!(keyexpr_intersects), "intersects")
        .method(pq!(keyexpr_includes), "includes")
        .method(pq!(keyexpr_relation_to), "relationTo")
        .enum_class(pq!(SetIntersectionLevel))
        // ── Config + ZenohId ──────────────────────────────────────────────
        .package("config")
        .ptr_class(pq!(Config))
        .accessor(pq!(config_get_json), "getJson")
        .accessor(pq!(config_new_clone), "newClone")
        // Factories → Config companion-object members.
        .constructor(pq!(config_new_default), "newDefault")
        .constructor(pq!(config_new_from_file), "fromFile")
        .constructor(pq!(config_new_from_json), "fromJson")
        .constructor(pq!(config_new_from_json5), "fromJson5")
        .constructor(pq!(config_new_from_yaml), "fromYaml")
        .fun(pq!(config_insert_json5))
        .enum_class(pq!(WhatAmI))
        // `ZenohId` is a `Copy` value (zenoh's `ZenohId`, repr(transparent)), so
        // it crosses as a raw byte-blob `ByteArray` rather than a closeable jlong
        // handle. `Vec<ZenohId>` (session peers/routers) folds each element WHOLE
        // as the typed `ZenohId` value class. Its read accessors become methods
        // on the value class (receiver = `this.bytes`).
        .value_class(pq!(ZenohId))
        .accessor(pq!(zenoh_id_to_bytes), "toBytes")
        .accessor(pq!(zenoh_id_to_string), "toStr")
        // ── Scouting ──────────────────────────────────────────────────────
        // Canonical output: the scout callback decomposes a `Hello` into its
        // three read fields in ONE crossing (no handle — read-only). Auto-applies
        // to `scout`'s `Fn(Hello)`.
        .package("scouting")
        .ptr_class(pq!(Hello))
        .accessor(pq!(hello_get_whatami), "whatami") // WhatAmI enum -> Int
        .accessor(pq!(hello_get_zid), "zid") // ZenohId value class -> ByteArray
        .accessor(pq!(hello_get_locators), "locators") // Vec<String> -> List<String>
        .flatten_output().field("whatami")
        .flatten_output().field("zid")
        .flatten_output().field("locators")
        .ptr_class(pq!(Scout))
        .fun(pq!(scout))
        // ── Logger ────────────────────────────────────────────────────────
        .package("logger")
        .fun(pq!(init_android_logs))
        .fun(pq!(try_init_zenoh_logs_from_env))
        .fun(pq!(init_zenoh_logs_from_env_or))
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
        .accessor(pq!(zbytes_as_bytes), "asBytes")
        .accessor(pq!(zbytes_to_bytes), "toBytes")
        .accessor(pq!(zbytes_new_clone), "newClone")
        // `fromVec` builds a ZBytes from a `ByteArray` — both the input-flatten
        // build variant AND a companion factory (a constructor never
        // output-flattens, so the factory keeps its raw-handle return).
        .constructor(pq!(zbytes_new_from_vec), "fromVec")
        .flatten_input().variant("fromVec")
        .flatten_output().field_self()
        // ── Bytes: Encoding ───────────────────────────────────────────────
        // Canonical input: encoding params cross as their decomposed value
        // `(id: i32, schema: Option<String>)` (built via `encoding_new_from_id`)
        // — cheap primitives, no per-call String parse, no native handle.
        // Canonical output: the handle (identity) + id (raw jint), both free
        // jvalue slots; schema and the canonical string stay on-demand accessors.
        .ptr_class(pq!(Encoding))
        .accessor(pq!(encoding_get_id), "id")
        .accessor(pq!(encoding_get_schema), "getSchema")
        .accessor(pq!(encoding_to_string), "toStr")
        .accessor(pq!(encoding_new_clone), "newClone")
        // `encoding_new_with_schema(&Encoding, schema) -> Encoding` derives a new
        // Encoding — a companion factory returning a raw handle (a constructor
        // never output-flattens, so the result stays a usable handle rather than
        // a decomposed builder).
        .constructor(pq!(encoding_new_with_schema), "withSchema")
        // Factories → companion members; `fromId` is also the input variant.
        .constructor(pq!(encoding_new_from_id), "fromId")
        .constructor(pq!(encoding_new_from_string), "fromString")
        // Input flatten: encoding params cross as the decomposed value
        // `(id, schema?)` (built via `fromId`). Output flatten: the handle +
        // its id (both free jvalue slots); schema/string stay accessors.
        .flatten_input().variant("fromId")
        .flatten_output().field_self().field("id");

    // Predefined encoding constants — each is a nullary `() -> &'static Encoding`
    // factory, declared as a companion `.constructor` of `Encoding` so they
    // collapse into one loop instead of ~50 identical lines. `.constructor`
    // returns `JniGen<PtrClass>` (the Encoding class is still live), so the
    // reassignment keeps a stable type. The factory name strips the
    // `encoding_const_` prefix and camelCases the rest (`text_plain` → `textPlain`).
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
        let name = const_name(&ctor);
        jni = jni.constructor(ctor, name);
    }

    let jni = jni
        // ── Time ──────────────────────────────────────────────────────────
        // Canonical output: a timestamp is its NTP64 value (`timestamp_get_ntp64`
        // -> i64 -> Long, 1 leaf); nested in a `Sample` it contributes that Long.
        .package("time")
        .ptr_class(pq!(Timestamp))
        .accessor(pq!(timestamp_get_ntp64), "ntp64")
        .accessor(pq!(timestamp_get_id), "getId")
        .flatten_output().field("ntp64")
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
        .accessor(pq!(sample_get_key_expr), "keyExpr")
        .accessor(pq!(sample_get_payload), "payload")
        .accessor(pq!(sample_get_encoding), "encoding")
        .accessor(pq!(sample_get_kind), "kind")
        .accessor(pq!(sample_get_timestamp), "timestamp")
        .accessor(pq!(sample_get_express), "express")
        .accessor(pq!(sample_get_priority), "priority")
        .accessor(pq!(sample_get_congestion_control), "congestionControl")
        .accessor(pq!(sample_get_attachment), "attachment")
        .accessor(pq!(sample_get_reliability), "reliability")
        .accessor(pq!(sample_get_source_zid), "sourceZid")
        .accessor(pq!(sample_get_source_eid), "sourceEid")
        .accessor(pq!(sample_get_source_sn), "sourceSn")
        .flatten_input().variant_self()
        .flatten_output().field("keyExpr")
        .flatten_output().field("payload")
        .flatten_output().field("encoding")
        .flatten_output().field("kind")
        .flatten_output().field("timestamp")
        .flatten_output().field("express")
        .flatten_output().field("priority")
        .flatten_output().field("congestionControl")
        .flatten_output().field("attachment")
        .flatten_output().field("reliability")
        .flatten_output().field("sourceZid")
        .flatten_output().field("sourceEid")
        .flatten_output().field("sourceSn")
        // Standalone sample constructors (callable from Kotlin); consumed by handle.
        .fun(pq!(sample_new_put))
        .fun(pq!(sample_new_delete))
        // ── Pub/Sub ───────────────────────────────────────────────────────
        // key_expr / payload / attachment / encoding params are auto-constructed
        // by their types' canonical inputs (no per-fn calls).
        .package("pubsub")
        .ptr_class(pq!(Publisher))
        .fun(pq!(publisher_put))
        .fun(pq!(publisher_delete))
        .ptr_class(pq!(Subscriber))
        // ── Query / Queryable / Querier ───────────────────────────────────
        .package("query")
        .ptr_class(pq!(Queryable))
        .ptr_class(pq!(Querier))
        .fun(pq!(querier_get))
        .enum_class(pq!(ReplyKeyExpr))
        .enum_class(pq!(QueryTarget))
        .enum_class(pq!(ConsolidationMode))
        // Canonical output: the queryable callback decomposes a `Query` into BOTH
        // its read fields AND the owned handle (identity) in ONE crossing — the
        // `output_direct` keeps the handle so the consumer can reply
        // (`query_reply_*`) after the callback returns; a query must outlive its
        // callback to be answered.
        //
        // ORDER MATTERS: `.flatten_output().field_self()` MUST be LAST. The root
        // identity moves the owned query while the nested KeyExpr identity (from
        // `query_get_keyexpr`) clones from a borrow of the same query; identity
        // leaves are emitted in declaration order, so the root move must follow
        // the nested borrow to avoid a "use of moved value" error.
        .ptr_class(pq!(Query))
        .accessor(pq!(query_get_keyexpr), "keyExpr")
        .accessor(pq!(query_get_parameters), "parameters")
        .accessor(pq!(query_get_payload), "payload")
        .accessor(pq!(query_get_encoding), "encoding")
        .accessor(pq!(query_get_attachment), "attachment")
        .accessor(pq!(query_get_accepts_replies), "acceptsReplies")
        .flatten_output().field("keyExpr")
        .flatten_output().field("parameters")
        .flatten_output().field("payload")
        .flatten_output().field("encoding")
        .flatten_output().field("attachment")
        .flatten_output().field("acceptsReplies")
        .flatten_output().field_self()
        // Reply ops on the owned/borrowed query handle.
        .fun(pq!(query_reply_success))
        .fun(pq!(query_reply_error))
        .fun(pq!(query_reply_delete))
        // `query_reply_sample` takes the sample by owned handle (Sample's
        // canonical input is identity — see the sample package above).
        .fun(pq!(query_reply_sample))
        // ── Reply / ReplyError ────────────────────────────────────────────
        // ReplyError canonical output: a failed reply's error decomposed in one
        // crossing — payload -> ByteArray, encoding -> String.
        .ptr_class(pq!(ReplyError))
        .accessor(pq!(reply_error_get_payload), "payload")
        .accessor(pq!(reply_error_get_encoding), "encoding")
        .flatten_output().field("payload")
        .flatten_output().field("encoding")
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
        .accessor(pq!(reply_get_replier_zid), "replierZid")
        .accessor(pq!(reply_get_replier_eid), "replierEid")
        .accessor(pq!(reply_is_ok), "isOk")
        .accessor(pq!(reply_get_sample), "sample")
        .accessor(pq!(reply_get_err), "err")
        .flatten_output().field("replierZid")
        .flatten_output().field("replierEid")
        .flatten_output().field("isOk")
        .flatten_output().field("sample")
        .flatten_output().field("err")
        // ── Liveliness + Session ──────────────────────────────────────────
        // `LivelinessToken` is just an opaque handle; the liveliness operations
        // (`liveliness_*`) are declared under the active `session` package below,
        // alongside the session API they extend.
        .package("liveliness")
        .ptr_class(pq!(LivelinessToken))
        .package("session")
        .ptr_class(pq!(Session))
        .accessor(pq!(session_get_zid), "getZid")
        .fun(pq!(open))
        .fun(pq!(session_declare_publisher))
        .fun(pq!(session_put))
        .fun(pq!(session_delete))
        .fun(pq!(session_declare_subscriber))
        .fun(pq!(session_declare_querier))
        .fun(pq!(session_declare_queryable))
        .fun(pq!(session_declare_keyexpr))
        // Undeclaring needs the declared handle, not a string — opt its key_expr
        // param out of the (String-building) input flatten.
        .fun(pq!(session_undeclare_keyexpr))
        .flatten_input_suppress(pq!(key_expr))
        .fun(pq!(session_get))
        // `Vec<ZenohId>`: ZenohId is a value class, so these return `List<ZenohId>`
        // via the normal Vec converter.
        .fun(pq!(session_get_peers_zid))
        .fun(pq!(session_get_routers_zid))
        // Liveliness ops (key_expr params auto-constructed by the canonical input).
        .fun(pq!(liveliness_declare_token))
        .fun(pq!(liveliness_get))
        .fun(pq!(liveliness_declare_subscriber));

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
