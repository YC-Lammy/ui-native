use jni_bind::jint;

use crate::android::binding::java::lang::Object;

jni_bind::import_class! {
    "android/graphics/Bitmap";
    Bitmap;
    extends Object;
    static fn createBitmap(width: jint, height: jint, config: bitmap::Config) -> Bitmap;
}

pub mod bitmap {
    use jni::errors::Error;
    use jni::JNIEnv;

    use crate::android::binding::java;

    jni_bind::import_class! {
        "android/graphics/Bitmap$Config";
        Config;
        extends java::lang::Object;
        static fn valueOf(s: java::lang::String) -> Config;
    }

    impl Config {
        /// **get** public static final [ALPHA_8](https://developer.android.com/reference/android/graphics/Bitmap.Config.html#ALPHA_8)
        pub fn ALPHA_8<'env>(env: &mut JNIEnv) -> Result<Config, Error> {
            let value = env.get_static_field(
                "android/graphics/Bitmap$Config\0",
                "ALPHA_8\0",
                "Landroid/graphics/Bitmap$Config;\0",
            )?;

            let r = env.new_global_ref(value.l()?)?;

            return Ok(Config { _obj: r });
        }

        /// **get** public static final [RGB_565](https://developer.android.com/reference/android/graphics/Bitmap.Config.html#RGB_565)
        pub fn RGB_565<'env>(env: &mut JNIEnv) -> Result<Config, Error> {
            let value = env.get_static_field(
                "android/graphics/Bitmap$Config\0",
                "RGB_565\0",
                "Landroid/graphics/Bitmap$Config;\0",
            )?;
            let r = env.new_global_ref(value.l()?)?;

            return Ok(Config { _obj: r });
        }

        /// **get** public static final [ARGB_4444](https://developer.android.com/reference/android/graphics/Bitmap.Config.html#ARGB_4444)
        pub fn ARGB_4444<'env>(env: &mut JNIEnv) -> Result<Config, Error> {
            let value = env.get_static_field(
                "android/graphics/Bitmap$Config\0",
                "ARGB_4444\0",
                "Landroid/graphics/Bitmap$Config;\0",
            )?;
            let r = env.new_global_ref(value.l()?)?;

            return Ok(Config { _obj: r });
        }

        /// **get** public static final [ARGB_8888](https://developer.android.com/reference/android/graphics/Bitmap.Config.html#ARGB_8888)
        pub fn ARGB_8888<'env>(env: &mut JNIEnv) -> Result<Config, Error> {
            let value = env.get_static_field(
                "android/graphics/Bitmap$Config\0",
                "ARGB_8888\0",
                "Landroid/graphics/Bitmap$Config;\0",
            )?;
            let r = env.new_global_ref(value.l()?)?;

            return Ok(Config { _obj: r });
        }

        /// **get** public static final [RGBA_F16](https://developer.android.com/reference/android/graphics/Bitmap.Config.html#RGBA_F16)
        pub fn RGBA_F16<'env>(env: &mut JNIEnv) -> Result<Config, Error> {
            let value = env.get_static_field(
                "android/graphics/Bitmap$Config\0",
                "RGBA_F16\0",
                "Landroid/graphics/Bitmap$Config;\0",
            )?;
            let r = env.new_global_ref(value.l()?)?;

            return Ok(Config { _obj: r });
        }

        /// **get** public static final [HARDWARE](https://developer.android.com/reference/android/graphics/Bitmap.Config.html#HARDWARE)
        pub fn HARDWARE(env: &mut JNIEnv) -> Result<Config, Error> {
            let value = env.get_static_field(
                "android/graphics/Bitmap$Config\0",
                "HARDWARE\0",
                "Landroid/graphics/Bitmap$Config;\0",
            )?;
            let r = env.new_global_ref(value.l()?)?;

            return Ok(Config { _obj: r });
        }
    }
}
