use super::content::Context;

jni_bind::import_class! {
    "android/app/Activity";
    Activity;
    extends Context;
}

jni_bind::import_class! {
    "android/app/NativeActivity";
    NativeActivity;
    extends Activity;
}
