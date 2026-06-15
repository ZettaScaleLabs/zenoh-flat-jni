pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
    }
}

rootProject.name = "zenoh-flat-jni"

// Read version from gradle.properties.
val versionName = providers.gradleProperty("version").get()
println("Zenoh Flat JNI version: $versionName")
