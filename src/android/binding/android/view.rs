use jni_bind::{jboolean, jint};

use super::super::java::lang::Object;
use super::content::Context;

jni_bind::import_class! {
    "android/view/View";
    View;
    extends Object;

    fn getSuggestedMinimumHeight(&self) -> jint;
    fn getSuggestedMinimumWidth(&self) -> jint;
    fn getMeasuredWidth(&self) -> jint;
    fn getMeasuredHeight(&self) -> jint;

    fn measure(&self, widthMeasureSpec: jint, heightMeasureSpec: jint) -> ();

    fn getLayoutParams(&self) -> view_group::LayoutParams;
    fn setLayoutParams(&self, params: view_group::LayoutParams) -> ();

    fn setId(&self, id: jint) -> ();
    fn setVisibility(&self, visibility: jint) -> ();
    fn setEnabled(&self, enabled: jboolean) -> ();

    fn setClickable(&self, b: jboolean) -> ();
    fn setOnClickListener(&self, l: OnClickListener) -> ();
}

impl View {
    pub const VISIBLE: jint = 0;
    pub const INVISIBLE: jint = 4;
    pub const GONE: jint = 8;
}

jni_bind::import_class! {
    "android/view/View/AccessibilityDelegate";
    AccessibilityDelegate;

}

jni_bind::import_class! {
    "android/view/View/BaseSavedState";
    BaseSavedState;
}

jni_bind::import_class! {
    "android/view/View/DragShadowBuilder";
    DragShadowBuilder;
}

jni_bind::import_class! {
    "android/view/View/MeasureSpec";
    MeasureSpec;

    static fn makeMeasureSpec(size: jint, mode: jint) -> jint;
}

impl MeasureSpec {
    pub const AT_MOST: jint = 0x80000000;
    pub const EXACTLY: jint = 0x40000000;
    pub const UNSPECIFIED: jint = 0;
}

jni_bind::import_interface! {
    "android/view/View/OnClickListener";
    OnClickListener;
}

jni_bind::import_class! {
    "android/view/ViewGroup";
    ViewGroup;
    extends View;
    constructor(ctx: Context);
    fn addStatesFromChildren(&self) -> ();
    fn addView(&self, child: View, index: jint, params: view_group::LayoutParams) -> ();
    fn bringChildToFront(&self, child: View) -> ();
    fn childDrawableStateChanged(&self, child: View) -> ();
    fn childHasTransientStateChanged(&self, child: View, childHasTransientState: jboolean) -> ();
    fn clearChildFocus(&self, child: View) -> ();
    fn clearDisappearingChildren(&self) -> ();
    fn clearFocus(&self) -> ();
    fn getChildAt(&self, index: jint) -> View;
    fn getChildCount(&self) -> jint;
    fn indexOfChild(&self, child: View) -> jint;
    fn removeView(&self, view: View) -> ();
    fn removeViewAt(&self, index: jint) -> ();
}

pub mod view_group {
    use jni_bind::jint;

    use super::View;

    jni_bind::import_class! {
        "android/view/ViewGroup/LayoutParams";
        LayoutParams;
        extends View;
    }

    jni_bind::import_class! {
        "android/view/ViewGroup/MarginLayoutParams";
        MarginLayoutParams;
        extends LayoutParams;

        field bottomMargin: jint;
        field leftMargin: jint;
        field rightMargin: jint;
        field topMargin: jint;
    }
}
