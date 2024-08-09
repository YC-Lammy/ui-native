use core::marker::PhantomData;

#[cfg(target_os = "android")]
use jni_bind::JNIEnv;

pub(crate) struct Context<'a> {
    #[cfg(target_os = "android")]
    pub jni_env: JNIEnv<'a>,
    #[cfg(target_os = "android")]
    pub android_activity: crate::android::binding::android::app::NativeActivity,

    pub _mark: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    #[cfg(not(target_os = "android"))]
    pub fn dummy() -> Self {
        Self { _mark: PhantomData }
    }
}
