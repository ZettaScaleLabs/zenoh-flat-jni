pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
    }
}

rootProject.name = "zenoh-flat-jni"

// Read version from gradle.properties
val zfVersion: String by settings
println("Zenoh Flat JNI version: $zfVersion")
