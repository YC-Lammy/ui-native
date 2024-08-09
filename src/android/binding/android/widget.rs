use jni_bind::jint;

use crate::android::binding::android::graphics::Bitmap;

use super::content::Context;
use super::view::{View, ViewGroup};

use super::super::java;

jni_bind::import_class! {
    "android/widget/RelativeLayout";
    RelativeLayout;
    extends ViewGroup;
    constructor(context: Context);

    fn requestLayout(&self) -> ();
}

pub mod relative_layout {
    use crate::android::binding::android::view::view_group;
    use jni_bind::jint;

    jni_bind::import_class! {
        "android/widget/RelativeLayout/LayoutParams";
        LayoutParams;
        extends view_group::MarginLayoutParams;
        constructor(w: jint, h: jint);
    }
}

jni_bind::import_class! {
    "android/widget/TextView";
    TextView;
    extends View;
    constructor(context: Context);

    fn setText(&self, text: java::lang::CharSequence) -> ();
}

jni_bind::import_class! {
    "android/widget/Button";
    Button;
    extends TextView;
    constructor(context: Context);
}

jni_bind::import_class! {
    "android/widget/ImageView";
    ImageView;
    extends View;
    constructor(context: Context);

    fn clearColorFilter(&self) -> ();
    fn setImageAlpha(&self, alpha: jint) -> ();
    fn setImageBitmap(&self, bitmap: Bitmap) -> ();
}
