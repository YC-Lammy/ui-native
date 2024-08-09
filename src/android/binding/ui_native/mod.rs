use jni_bind::{import_class, jint};

use super::android::view::{OnClickListener, View};

import_class! {
    "com/uinative/ButtonOnClickListener";
    ButtonOnClickListener;
    implements OnClickListener;
    constructor(id: jint);
    fn setId(&self, id: jint) -> ();
    fn getId(&self) -> jint;
    fn onClick(&self, view: View) -> ();
    fn onDrop(&self) -> ();
}
