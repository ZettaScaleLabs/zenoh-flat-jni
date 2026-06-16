//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

package io.zenoh.jni

import java.io.File
import java.io.FileOutputStream
import java.io.InputStream
import java.util.zip.ZipInputStream

/**
 * Loads the `zenoh_flat_jni` native library exactly once for the whole process.
 *
 * This is the SINGLE owner of all native-loading concerns for this binding: the
 * library's base name, the per-platform file naming, the JAR resource layout,
 * and the actual `System.load*` calls all live here. Consumers (e.g. the
 * zenoh-java SDK) treat this module as an ordinary JVM library and never touch
 * the native library directly — every generated `JNINative` call and the
 * hand-written `JNIBytes` (de)serialization entry trigger [ensureLoaded] in
 * their static initializer, so the library is in place before any extern call.
 *
 * Loading strategy (first that succeeds wins):
 *  1. `System.loadLibrary("zenoh_flat_jni")` — covers Android (the `.so` is in
 *     the APK's `jniLibs`) and any host that put the library on
 *     `java.library.path`.
 *  2. The library bundled at the JAR root as `lib<name>.{dylib,so}` /
 *     `<name>.dll` — extracted to a temp file and `System.load`ed by absolute
 *     path (the local/single-platform build layout).
 *  3. The library bundled per-platform as `<target>/<target>.zip` — extracted
 *     and loaded (the multi-platform published layout).
 */
internal object NativeLibrary {
    private const val LIB_NAME = "zenoh_flat_jni"

    init {
        // 1) Android / java.library.path.
        if (runCatching { System.loadLibrary(LIB_NAME) }.isFailure) {
            // 2) Bundled at JAR root, else 3) per-target zip.
            if (tryLoadingBundledLibrary().isFailure) {
                tryLoadingTargetZip().getOrThrow()
            }
        }
    }

    /**
     * No-op whose sole purpose is to force this object's one-time, thread-safe
     * static initialization (the JVM guarantees `<clinit>` runs exactly once).
     * Call it from any JNI entry point's static initializer.
     */
    fun ensureLoaded() {
        // Referencing `NativeLibrary` (which the caller already did to reach
        // this method) has triggered `init {}` above; nothing to do here.
    }

    // ── Strategy 2: library bundled at the JAR root ──────────────────────────

    private fun tryLoadingBundledLibrary(): Result<Unit> = runCatching {
        val stream = findBundledLibraryStream()
            ?: throw UnsatisfiedLinkError("No bundled `$LIB_NAME` native library on the classpath.")
        loadFromStream(stream)
    }

    private fun findBundledLibraryStream(): InputStream? {
        val loader = NativeLibrary::class.java.classLoader
            ?: ClassLoader.getSystemClassLoader()
        for (ext in listOf(".dylib", ".so")) {
            loader.getResourceAsStream("lib$LIB_NAME$ext")?.let { return it }
        }
        // Windows: no `lib` prefix.
        loader.getResourceAsStream("$LIB_NAME.dll")?.let { return it }
        return null
    }

    // ── Strategy 3: per-target zipped library ────────────────────────────────

    private fun tryLoadingTargetZip(): Result<Unit> = runCatching {
        val target = determineTarget()
        val resource = "$target/$target.zip"
        val loader = NativeLibrary::class.java.classLoader
            ?: ClassLoader.getSystemClassLoader()
        val zipStream = loader.getResourceAsStream(resource)
            ?: throw UnsatisfiedLinkError("Unable to locate native library resource `$resource`.")
        val unzipped = unzipSingleEntry(zipStream)
        loadFromAbsolutePath(unzipped)
    }

    private fun determineTarget(): String {
        val osName = System.getProperty("os.name").lowercase()
        val osArch = System.getProperty("os.arch").lowercase()
        val isX64 = osArch.contains("x86_64") || osArch.contains("amd64") || osArch.contains("x64")
        val isArm64 = osArch.contains("aarch64") || osArch.contains("arm64")
        return when {
            osName.contains("win") -> when {
                isX64 -> "x86_64-pc-windows-msvc"
                isArm64 -> "aarch64-pc-windows-msvc"
                else -> unsupported(osName, osArch)
            }
            osName.contains("mac") || osName.contains("darwin") || osName.contains("os x") -> when {
                isX64 -> "x86_64-apple-darwin"
                isArm64 -> "aarch64-apple-darwin"
                else -> unsupported(osName, osArch)
            }
            osName.contains("nix") || osName.contains("nux") || osName.contains("aix") -> when {
                isX64 -> "x86_64-unknown-linux-gnu"
                isArm64 -> "aarch64-unknown-linux-gnu"
                else -> unsupported(osName, osArch)
            }
            else -> throw UnsupportedOperationException("Unsupported platform: $osName")
        }
    }

    private fun unsupported(osName: String, osArch: String): Nothing =
        throw UnsupportedOperationException("Unsupported architecture `$osArch` on `$osName`.")

    private fun unzipSingleEntry(compressed: InputStream): File {
        ZipInputStream(compressed).use { zip ->
            val entry = zip.nextEntry ?: throw IllegalStateException("Empty native-library archive.")
            val out = File.createTempFile(entry.name, ".tmp").apply { deleteOnExit() }
            out.parentFile?.mkdirs()
            FileOutputStream(out).use { zip.copyTo(it) }
            zip.closeEntry()
            return out
        }
    }

    // ── Shared: extract a stream to a temp file and load it ──────────────────

    @Suppress("UnsafeDynamicallyLoadedCode")
    private fun loadFromStream(stream: InputStream) {
        val tempLib = File.createTempFile("lib$LIB_NAME", ".tmp").apply { deleteOnExit() }
        FileOutputStream(tempLib).use { stream.copyTo(it) }
        loadFromAbsolutePath(tempLib)
    }

    @Suppress("UnsafeDynamicallyLoadedCode")
    private fun loadFromAbsolutePath(file: File) {
        System.load(file.absolutePath)
    }
}
