use std::sync::atomic::{AtomicU64, Ordering};

use alloc::{borrow::Cow, vec::Vec};

pub enum MaybeInherit<T>{
    Inherit,
    Value(T)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
}

/// RGBA8
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl StyleValue for Colour{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    #[default]
    Inherit,
    Solid,
    Dotted,
    Dashed,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PointEvents {
    #[default]
    Auto,
    None,
    BoxNone,
    BoxOnly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlignContent {
    #[default]
    Inherit,
    FlexStart,
    FlexEnd,
    Centre,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    #[default]
    Stretch,
    FlexStart,
    FlexEnd,
    Centre,
    Baseline,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Dimension {
    #[default]
    Auto,
    Points(f32),
    Percent(f32),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    #[default]
    None,
    Flex,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PositionType {
    /// the properties `top`, `bottom`, `left`, `right`, `z-index` have no effect
    #[default]
    Static,
    /// the element is offset rlative to parent
    Relative,
    /// the element has absolote offset
    Absolute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Inherit,
    LTR,
    RTL,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Number(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Centre,
    Justified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecorationLine {
    None,
    Underline,
    LineThrough,
    UnderlineLineThrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalise,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Style {
    /////////////////////////////////////////
    ///////  common to all components ///////
    /////////////////////////////////////////
    pub display: Display,
    pub direction: Direction,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,
    pub align_items: AlignItems,
    pub align_self: Option<AlignItems>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<JustifyContent>,

    pub position: PositionType,

    pub top: Option<Dimension>,
    pub bottom: Option<Dimension>,
    pub left: Option<Dimension>,
    pub right: Option<Dimension>,
    /// when direction is `ltr`, `start` is equivalant to `left`.
    /// when direction is `rtl`, `start` is equivalant to `right`.
    pub start: Option<Dimension>,
    /// when direction is `ltr`, `end` is equivalant to `right`.
    pub end: Option<Dimension>,

    /// setting margin set top, bottom, left, right
    pub margin: Option<Dimension>,

    /// setting `margin_vertical` sets both `margin_bottom` and `margin_top`
    pub margin_vertical: Option<Dimension>,
    pub margin_bottom: Option<Dimension>,
    pub margin_top: Option<Dimension>,

    /// setting `margin_horizontal` sets both `margin_left` and `margin_right`
    pub margin_horizontal: Option<Dimension>,
    pub margin_left: Option<Dimension>,
    pub margin_right: Option<Dimension>,

    /// when direction is `ltr`, `margin_end` is equivalant to `margin_right`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_left`
    pub margin_end: Option<Dimension>,
    /// when direction is `ltr`, `margin_end` is equivalant to `margin_left`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_right`
    pub margin_start: Option<Dimension>,

    pub padding: Option<Dimension>,
    /// setting `margin_vertical` sets both `margin_bottom` and `margin_top`
    pub padding_vertical: Option<Dimension>,
    pub padding_bottom: Option<Dimension>,
    pub padding_top: Option<Dimension>,
    /// setting `margin_horizontal` sets both `margin_left` and `margin_right`
    pub padding_horizontal: Option<Dimension>,
    pub padding_left: Option<Dimension>,
    pub padding_right: Option<Dimension>,
    /// when direction is `ltr`, `margin_end` is equivalant to `margin_right`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_left`
    pub padding_end: Option<Dimension>,
    /// when direction is `ltr`, `margin_end` is equivalant to `margin_left`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_right`
    pub padding_start: Option<Dimension>,

    pub border_width: Option<f64>,

    pub border_top_width: Option<f64>,
    pub border_bottom_width: Option<f64>,
    pub border_left_width: Option<f64>,
    pub border_right_width: Option<f64>,

    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,

    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub min_width: Option<Dimension>,
    pub min_height: Option<Dimension>,
    pub max_width: Option<Dimension>,
    pub max_height: Option<Dimension>,
    pub aspect_ratio: Option<Dimension>,

    //////////////////////////////////////////
    ///////  only used by view widget  ///////
    //////////////////////////////////////////
    pub backface_visible: Option<Visibility>,
    pub background_colour: Option<Colour>,

    pub border_radius: Option<f64>,

    pub border_top_start_radius: Option<f64>,
    pub border_top_end_radius: Option<f64>,
    pub border_top_left_radius: Option<f64>,
    pub border_top_right_radius: Option<f64>,

    pub border_bottom_start_radius: Option<f64>,
    pub border_bottom_end_radius: Option<f64>,
    pub border_bottom_left_radius: Option<f64>,
    pub border_bottom_right_radius: Option<f64>,

    pub border_start_start_radius: Option<f64>,
    pub border_start_end_radius: Option<f64>,
    pub border_end_end_radius: Option<f64>,
    pub border_end_start_radius: Option<f64>,

    pub border_colour: Option<Colour>,
    pub border_start_colour: Option<Colour>,
    pub border_end_colour: Option<Colour>,
    pub border_top_colour: Option<Colour>,
    pub border_bottom_colour: Option<Colour>,
    pub border_left_colour: Option<Colour>,
    pub border_right_colour: Option<Colour>,

    pub border_style: Option<BorderStyle>,

    pub opacity: Option<f64>,
    pub point_events: Option<PointEvents>,

    //////////////////////////////////////////
    ///////  only used by text widget  ///////
    //////////////////////////////////////////
    pub colour: Option<Colour>,
    pub font_family: Option<Cow<'static, str>>,
    pub font_size: Option<f64>,
    pub font_style: Option<FontStyle>,
    pub font_weight: Option<FontWeight>,
    pub font_varient: Option<Vec<Cow<'static, str>>>,
    pub letter_spacing: Option<f64>,
    pub line_height: Option<f64>,
    pub text_align: Option<TextAlign>,
    pub text_decoration_line: Option<TextDecorationLine>,
    pub text_shadow_colour: Option<Colour>,
    // pub text_shadow_offset: ?,
    pub text_shadow_radius: Option<f64>,
    pub text_transform: Option<TextTransform>,
}

macro_rules! imp_inherit_from {
    (
            $(
                pub $field:ident : $ty:ty
            ),*

    ) => {
        impl Style{
            pub fn inherit_from(&mut self, parent: &Self){
                $(
                    self.$field = Some(self.$field.clone().unwrap_or(parent.$field.clone().unwrap()))
                );*
            }
        }
    };
}

imp_inherit_from!(
        pub align_self: Option<AlignItems>,
        pub align_content: Option<AlignContent>,
        pub justify_content: Option<JustifyContent>,

        pub top: Option<Dimension>,
        pub bottom: Option<Dimension>,
        pub left: Option<Dimension>,
        pub right: Option<Dimension>,
        pub start: Option<Dimension>,
        pub end: Option<Dimension>,

        pub margin: Option<Dimension>,

        pub margin_vertical: Option<Dimension>,
        pub margin_bottom: Option<Dimension>,
        pub margin_top: Option<Dimension>,

        pub margin_horizontal: Option<Dimension>,
        pub margin_left: Option<Dimension>,
        pub margin_right: Option<Dimension>,

        pub margin_end: Option<Dimension>,
        pub margin_start: Option<Dimension>,

        pub padding: Option<Dimension>,
        pub padding_vertical: Option<Dimension>,
        pub padding_bottom: Option<Dimension>,
        pub padding_top: Option<Dimension>,
        pub padding_horizontal: Option<Dimension>,
        pub padding_left: Option<Dimension>,
        pub padding_right: Option<Dimension>,
        pub padding_end: Option<Dimension>,
        pub padding_start: Option<Dimension>,

        pub border_width: Option<f64>,

        pub border_top_width: Option<f64>,
        pub border_bottom_width: Option<f64>,
        pub border_left_width: Option<f64>,
        pub border_right_width: Option<f64>,

        pub width: Option<Dimension>,
        pub height: Option<Dimension>,
        pub min_width: Option<Dimension>,
        pub min_height: Option<Dimension>,
        pub max_width: Option<Dimension>,
        pub max_height: Option<Dimension>,
        pub aspect_ratio: Option<Dimension>,


        //////////////////////////////////////////
        ///////  only used by view widget  ///////
        //////////////////////////////////////////

        pub backface_visible: Option<Visibility>,
        pub background_colour: Option<Colour>,

        pub border_radius: Option<f64>,

        pub border_top_start_radius: Option<f64>,
        pub border_top_end_radius: Option<f64>,
        pub border_top_left_radius: Option<f64>,
        pub border_top_right_radius: Option<f64>,

        pub border_bottom_start_radius: Option<f64>,
        pub border_bottom_end_radius: Option<f64>,
        pub border_bottom_left_radius: Option<f64>,
        pub border_bottom_right_radius: Option<f64>,

        pub border_start_start_radius: Option<f64>,
        pub border_start_end_radius: Option<f64>,
        pub border_end_end_radius: Option<f64>,
        pub border_end_start_radius: Option<f64>,

        pub border_colour: Option<Colour>,
        pub border_start_colour: Option<Colour>,
        pub border_end_colour: Option<Colour>,
        pub border_top_colour: Option<Colour>,
        pub border_bottom_colour: Option<Colour>,
        pub border_left_colour: Option<Colour>,
        pub border_right_colour: Option<Colour>,

        pub border_style: Option<BorderStyle>,

        pub opacity: Option<f64>,
        pub point_events: Option<PointEvents>,

        //////////////////////////////////////////
        ///////  only used by text widget  ///////
        //////////////////////////////////////////
        pub colour: Option<Colour>,
        pub font_family: Option<Cow<'static, str>>,
        pub font_size: Option<f64>,
        pub font_style: Option<FontStyle>,
        pub font_weight: Option<FontWeight>,
        pub font_varient: Option<Vec<Cow<'static, str>>>,
        pub letter_spacing: Option<f64>,
        pub line_height: Option<f64>,
        pub text_align: Option<TextAlign>,
        pub text_decoration_line: Option<TextDecorationLine>,
        pub text_shadow_colour: Option<Colour>,
        // pub text_shadow_offset: ?,
        pub text_shadow_radius: Option<f64>,
        pub text_transform: Option<TextTransform>
);

impl Style {
    pub fn normalise(&mut self) {}

    pub fn fill_defaults(&mut self) {}
}

pub struct StyleSheetProvider{

}

struct FieldMask([AtomicU64;4]);

impl FieldMask{
    pub fn is_set(&self, idx: usize) -> bool{
        let mask = self.0[idx / 64].load(Ordering::Relaxed);
        let offset = idx % 64;

        let mask_true = 0x1u64 << offset;

        return (mask & mask_true) != 0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyleSheetID(u32);

pub struct StyleSheetHeader{
    /// parent style sheet
    parent: Option<StyleSheetID>,
    /// mask for active fields.
    /// style is inherited from parent if not active
    owned: FieldMask,
    /// the rest is data
    data: [u8;0]
}

pub trait StyleValue: Clone + 'static{}



macro_rules! gen_dyn_stylesheet {
    ($(
        $field:tt : $ty:ty
    ),*) => {
        paste::paste!{
            impl StyleSheetHeader{
                pub fn get<T: StyleValue>(&self, name: &str) -> Option<T>{
                    let mut idx = 0;
                    let mut ptr = self.data.as_ptr();

                    unsafe{
                        $(
                            if self.owned.is_set(idx){
                                if name == stringify!($field){
                                    assert!(core::any::TypeId::of::<T>() == core::any::TypeId::of::<$ty>());

                                    return Some((ptr as *const T).as_ref().unwrap().clone())
                                };
                                ptr = ptr.add(core::mem::size_of::<$ty>());
                            }
                            idx += 1;
                        )*
                    }
                    return None;
                }

                pub fn set<T: StyleValue>(&self, name: &str, value: T) -> bool{
                    let mut idx = 0;
                    let mut ptr = self.data.as_ptr();

                    unsafe{
                        $(
                            if self.owned.is_set(idx){
                                assert!(core::any::TypeId::of::<T>() == core::any::TypeId::of::<$ty>());

                                if name == stringify!($field){
                                    let r = (ptr as *mut T).as_mut().unwrap();
                                    *r = value;
                                    return true;
                                };
                                ptr = ptr.add(core::mem::size_of::<$ty>());
                            }
                            idx += 1;
                        )*
                    }
                    return false;
                }

                $(
                    pub fn [<get_ $field:snake>](&self) -> $ty{
                        self.get(stringify!($field)).unwrap()
                    }
                )*

                $(
                    pub fn [<set_ $field:snake>](&self, value: $ty) -> bool{
                        self.set(stringify!($field), value)
                    }
                )*
            }
        }
        
    };
}

gen_dyn_stylesheet!{
    color: Colour
}