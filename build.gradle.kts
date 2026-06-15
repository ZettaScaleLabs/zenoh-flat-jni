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

plugins {
    `maven-publish`
    signing
}

group = "org.eclipse.zenoh"
version = rootProject.version

// ============================================================================
// Build Configuration
// ============================================================================

val isRemotePublication = project.findProperty("remotePublication")?.toString()?.toBoolean() == true
val release = project.findProperty("release")?.toString()?.toBoolean() == true

enum class BuildMode {
    DEBUG, RELEASE
}

val buildMode = if (release) BuildMode.RELEASE else BuildMode.DEBUG

// ============================================================================
// Rust Build Configuration
// ============================================================================

tasks.register("buildZenohFlatJni") {
    description = "Build zenoh-flat-jni Rust library"
    doLast {
        val cargoCommand = mutableListOf("cargo", "build")
        if (buildMode == BuildMode.RELEASE) {
            cargoCommand.add("--release")
        }
        
        val result = project.exec {
            commandLine(*(cargoCommand.toTypedArray()))
            isIgnoreExitValue = true
        }
        
        if (result.exitValue != 0) {
            throw GradleException("Failed to build zenoh-flat-jni. Exit code: ${result.exitValue}")
        }
    }
}

// ============================================================================
// JAR Packaging
// ============================================================================

val jarTarget = if (buildMode == BuildMode.RELEASE) "target/release" else "target/debug"
val dylibExt = when {
    System.getProperty("os.name").lowercase().contains("mac") -> "dylib"
    System.getProperty("os.name").lowercase().contains("win") -> "dll"
    else -> "so"
}
val dylibName = when {
    System.getProperty("os.name").lowercase().contains("win") -> "zenoh_flat_jni.dll"
    System.getProperty("os.name").lowercase().contains("mac") -> "libzenoh_flat_jni.dylib"
    else -> "libzenoh_flat_jni.so"
}

tasks.register<Jar>("packageJar") {
    description = "Package zenoh-flat-jni library into JAR with native libraries and Kotlin sources"
    dependsOn("buildZenohFlatJni")
    
    archiveBaseName.set("zenoh-flat-jni")
    archiveVersion.set(project.version.toString())
    
    // Include Kotlin sources
    from("kotlin") {
        into("io/zenoh/jni")
    }
    
    from("generated-kotlin") {
        into("io/zenoh/jni")
    }
    
    // Include native library
    // Multi-platform JAR would include libs for all platforms
    // For now, include current platform's library
    val platformPath = "META-INF/lib"
    from(jarTarget) {
        include(dylibName)
        into(platformPath)
    }
}

// ============================================================================
// Maven Publishing Configuration
// ============================================================================

publishing {
    repositories {
        if (isRemotePublication) {
            // Publish to Maven Central via OSSRH
            maven("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/") {
                name = "ossrh"
                credentials {
                    username = System.getenv("OSSRH_USERNAME") ?: ""
                    password = System.getenv("OSSRH_PASSWORD") ?: ""
                }
            }
        } else {
            // Local Maven repository for development
            mavenLocal()
        }
    }

    publications {
        register<MavenPublication>("maven") {
            groupId = "org.eclipse.zenoh"
            artifactId = "zenoh-flat-jni"
            version = project.version.toString()

            // Include the JAR artifact
            artifact("build/libs/zenoh-flat-jni-${project.version}.jar") {
                builtBy("packageJar")
            }

            // Include sources JAR
            val sourcesJar = tasks.register<Jar>("sourcesJar") {
                archiveBaseName.set("zenoh-flat-jni")
                archiveVersion.set(project.version.toString())
                archiveClassifier.set("sources")
                
                from("src")
                from("kotlin")
                from("build.rs")
                from("Cargo.toml")
            }
            artifact(sourcesJar)

            pom {
                name.set("Zenoh Flat JNI")
                description.set("Zenoh JNI bindings and Kotlin wrappers - generated from zenoh-flat via prebindgen")
                url.set("https://zenoh.io/")

                licenses {
                    license {
                        name.set("Eclipse Public License 2.0 OR Apache License 2.0")
                        url.set("http://www.eclipse.org/legal/epl-2.0")
                    }
                }

                developers {
                    developer {
                        id.set("ZettaScale")
                        name.set("ZettaScale Zenoh Team")
                        email.set("zenoh@zettascale.tech")
                    }
                }

                scm {
                    connection.set("scm:git:https://github.com/ZettaScaleLabs/zenoh-flat-jni.git")
                    developerConnection.set("scm:git:https://github.com/ZettaScaleLabs/zenoh-flat-jni.git")
                    url.set("https://github.com/ZettaScaleLabs/zenoh-flat-jni")
                }
            }
        }
    }
}

// ============================================================================
// Signing Configuration
// ============================================================================

signing {
    isRequired = isRemotePublication
    if (isRemotePublication) {
        useInMemoryPgpKeys(
            System.getenv("ORG_GPG_SUBKEY_ID"),
            System.getenv("ORG_GPG_PRIVATE_KEY"),
            System.getenv("ORG_GPG_PASSPHRASE")
        )
    }
    sign(publishing.publications)
}

// Ensure signing happens after artifact generation
tasks.withType<Sign>().configureEach {
    dependsOn("packageJar")
}

// ============================================================================
// Task Dependencies
// ============================================================================

tasks.named("publishMavenPublicationTo" + if (isRemotePublication) "Ossrh" else "MavenLocal" + "Repository") {
    dependsOn(tasks.withType<Sign>())
}
