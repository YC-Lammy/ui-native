use jni_bind::{jboolean, jchar, jint, jlong};

jni_bind::import_class! {
    "java/lang/Object";
    Object;
    constructor();
    /// Indicates whether some other object is "equal to" this one.
    fn equals(&self, other: Object) ->  jboolean;
    /// Returns a hash code value for the object.
    fn hashCode(&self) -> jint;
    /// Wakes up a single thread that is waiting on this object's monitor.
    fn notify(&self) -> ();
    /// Wakes up all threads that are waiting on this object's monitor.
    fn notifyAll(&self) -> ();
    /// Returns a string representation of the object.
    fn toString(&self) -> String;
    /// Causes the current thread to wait until it is awakened,
    /// typically by being notified or interrupted,
    /// or until a certain amount of real time has elapsed.
    fn wait(&self, timeout_millis: jlong, nanos: jint) -> ();
}

jni_bind::import_interface! {
    "java/lang/CharSequence";
    CharSequence;
    //static fn compare(cs1: CharSequence, cs2: CharSequence) -> jint;
    fn charAt(&self, index: jint) -> jchar;
    fn chars(&self) -> ();
    fn codePoints(&self) -> ();
    fn isEmpty(&self) -> jboolean;
    fn length(&self) -> jint;
    fn toString(&self) -> String;
}

jni_bind::import_class! {
    "java/lang/String";
    String;
    extends Object;
    implements CharSequence;
}

impl String {
    pub fn from_str(env: &mut jni::JNIEnv, s: &str) -> String {
        let s = env.new_string(s).expect("");
        let r = env.new_global_ref(s).expect("");

        return String { _obj: r };
    }
}
