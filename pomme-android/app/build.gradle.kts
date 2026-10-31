plugins {
    id("com.android.application")
}

android {
    namespace = "com.pommemc.client"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.pommemc.client"
        // Android 10, the first with a Vulkan 1.1 loader everywhere.
        minSdk = 29
        targetSdk = 35
        versionCode = 3
        versionName = "0.3"
        ndk {
            abiFilters += "arm64-v8a"
        }
    }
}
