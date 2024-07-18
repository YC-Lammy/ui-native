use jni::errors::Result;
use jni::objects::{JClass, JMethodID, JObject, JValue, JValueOwned};

use crate::platform::android::{ANDROID_ACTIVITY, JNI_ENV};

use lazy_static::lazy_static;

use crate::Element;

macro_rules! bind_class {
    (class $name:ident -> $jvm_class:expr =>
        $(static $static_method:tt : $static_sig:expr;)*
        $($method:tt : $sig:expr;)*
    ) => {
        paste::paste!{
            #[allow(unused)]
            #[allow(non_snake_case)]
            pub struct [<Android $name Cache>]{
                class: JClass<'static>,
                $(
                    $method : JMethodID,
                )*
            }

            #[allow(non_snake_case)]
            impl [<Android $name Cache>]{
                pub fn new() -> Self{
                    JNI_ENV.with(|env|{
                        let mut env = env.borrow_mut();
                        let class = env.find_class($jvm_class).expect("");
                        $(
                            let $method = env.get_method_id(&class, stringify!($method), $sig).expect(stringify!(require method $method));
                        )*
                        Self{
                            class,
                            $($method),*
                        }
                    })
                }
            }

            lazy_static::lazy_static!{
                static ref [<CACHE_ $name:upper>]: [<Android $name Cache>] = [<Android $name Cache>]::new();
            }

            pub struct [<Android $name>]{
                obj: JObject<'static>
            }

            #[allow(non_snake_case)]
            impl [<Android $name>]{
                pub fn new_object(sig: &str, args: &[JValue]) -> Self{

                    JNI_ENV.with(|env|{
                        let obj = env.borrow_mut().new_object(&[<CACHE_ $name:upper>].class, sig, args).expect("failed");
                        Self{
                            obj: obj
                        }
                    })
                }
                $(
                    pub fn $static_method(&args: &[JValue]) -> Result<JValueOwned<'static>>{
                        JNI_ENV.with(|env|{
                            env.borrow_mut().call_static_method(&CACHE.class, stringify!($static_method), $static_sig, args)
                        })
                    }
                )*
                $(
                    pub fn $method(&self, args: &[JValue]) -> Result<JValueOwned<'static>>{
                        JNI_ENV.with(|env|{
                            env.borrow_mut().call_method(&self.obj, stringify!($method), $sig, args)
                        })
                    }
                )*
            }
        }
    };
}

bind_class! {
    class FlexboxLayoutManager -> "com/google/android/flexbox/FlexboxLayoutManager" =>
    isAutoMeasureEnabled : "()Z";
    getFlexDirection: "()I";
    setFlexDirection: "(I)V";
    getFlexWrap: "()I";
    setFlexWrap: "(I)V";
    getJustifyContent: "()I";
    setJustifyContent: "(I)V";
    getAlignItems: "()I";
    setAlignItems: "(I)V";
    getWidth: "()I";
    setWidth: "(I)V";
    getHeight: "()I";
    setHeight: "(I)V";
    getFlexGrow: "()F";
    setFlexGrow: "(F)V";
    getFlexShrink: "()F";
    setFlexShrink: "(F)V";
    getAlignSelf: "()I";
    setAlignSelf: "(I)V";
    getMinWidth: "()I";
    setMinWidth: "(I)V";
    getMaxWidth: "()I";
    setMaxWidth: "(I)V";
    getMinHeight: "()I";
    setMinHeight: "(I)V";
    getMaxHeight: "()I";
    setMaxHeight: "(I)V";
}

bind_class! {
    class FlexboxLayout -> "com/google/android/flexbox/FlexboxLayout" =>
    getFlexItemCount: "()I";
    getFlexItemAt: "(I)Landroid/view/View";
    getReorderedChildAt: "(I)Landroid/view/View";
    getReorderedFlexItemAt: "(I)Landroid/view/View";
    addView: "(Landroid/view/View)V";
    getLayoutParams: "()Landroid/view/ViewGroup/LayoutParams";
    setLayoutParams: "(Landroid/view/ViewGroup/LayoutParams)V";
}

bind_class! {
    class FlexboxLayoutParams -> "com/google/android/flexbox/FlexboxLayout/LayoutParams" =>
    getWidth: "()I";
    setWidth: "(I)V";
    getHeight: "()I";
    setHeight: "(I)V";
    getOrder: "()I";
    setOrder: "(I)V";
    getFlexGrow: "()F";
    setFlexGrow: "(F)V";
    getFlexShrink: "()F";
    setFlexShrink: "(F)V";
    getAlignSelf: "()I";
    setAlignSelf: "(I)V";
    getMinWidth: "()I";
    setMinWidth: "(I)V";
    getMinHeight: "()I";
    setMinHeight: "(I)V";
    getMaxWidth: "()I";
    setMaxWidth: "(I)V";
    getMaxHeight: "()I";
    setMaxHeight: "(I)V";
    isWrapBefore: "()Z";
    setWrapBefore: "(Z)V";
    getFlexBasisPercent: "()F";
    setFlexBasisPercent: "(F)V";
    getMarginLeft: "()I";
    getMarginTop: "()I";
    getMarginRight: "()I";
    getMarginBottom: "()I";
}

pub trait NativeElement {
    fn as_object(&self) -> &JObject;
}

pub struct NativeView {
    view: AndroidFlexboxLayout,
}

impl NativeView {
    #[allow(static_mut_refs)]
    pub fn new() -> Self {
        Self {
            view: AndroidFlexboxLayout::new_object(
                "(Landroid/content/Context)V",
                &[JValue::Object(unsafe { &ANDROID_ACTIVITY })],
            ),
        }
    }
    pub fn add_child<T>(&self, child: &T)
    where
        T: NativeElement,
    {
        let _ = self.view.addView(&[JValue::Object(child.as_object())]);
    }
}

impl NativeElement for NativeView {
    fn as_object(&self) -> &JObject {
        &self.view.obj
    }
}
