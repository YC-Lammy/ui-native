use std::{borrow::Cow, sync::{atomic::{AtomicU64, Ordering}, Arc}};

use parking_lot::RwLock;


trait StyleValue: Clone + Default + 'static {
    // cheat to impl From<Style> easier
    fn unwrap(self) -> Self{
        self
    }
}

impl StyleValue for f64{}

pub enum MaybeInherit<T> {
    Inherit,
    Owned(T),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
}

impl StyleValue for Visibility{}

/// RGBA8
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl StyleValue for Colour {}



#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    #[default]
    Solid,
    Dotted,
    Dashed,
}

impl StyleValue for BorderStyle{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PointEvents {
    #[default]
    Auto,
    None,
    BoxNone,
    BoxOnly,
}

impl StyleValue for PointEvents{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlignContent {
    #[default]
    Normal,
    Start,
    End,
    FlexStart,
    FlexEnd,
    Centre,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl StyleValue for AlignContent{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    #[default]
    Normal,
    Stretch,
    Start,
    End,
    FlexStart,
    FlexEnd,
    Centre,
    Baseline,
}

impl StyleValue for AlignItems{}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Dimension {
    #[default]
    Auto,
    Points(f32),
    Percent(f32),
}

impl StyleValue for Dimension{}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarginDimension{
    Auto,
    Points(f32),
    Percent(f32),
}

impl Default for MarginDimension{
    fn default() -> Self {
        Self::Points(0.0)
    }
}

impl StyleValue for MarginDimension{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    #[default]
    Flex,
    Block,
    Grid,
    None,
}

impl StyleValue for Display{}

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

impl StyleValue for PositionType{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    LTR,
    RTL,
}

impl StyleValue for Direction{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl StyleValue for FlexDirection{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexShrink(pub f32);

impl Default for FlexShrink{
    fn default() -> Self {
        Self(1.0)
    }
}

impl StyleValue for FlexShrink{}

impl StyleValue for FlexWrap{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    #[default]
    Normal,
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Start,
    End,
    Stretch
}

impl StyleValue for JustifyContent{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
}

impl StyleValue for Overflow{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

impl StyleValue for FontStyle{}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum FontWeight {
    #[default]
    Normal,
    Bold,
    Number(f64),
}

impl StyleValue for FontWeight{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Auto,
    Left,
    Right,
    Centre,
    Justified,
}

impl StyleValue for TextAlign{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TextDecorationLine {
    #[default]
    None,
    Underline,
    LineThrough,
    UnderlineLineThrough,
}

impl StyleValue for TextDecorationLine{}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TextTransform {
    #[default]
    None,
    Uppercase,
    Lowercase,
    Capitalise,
}

impl StyleValue for TextTransform{}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AspectRatio{
    #[default]
    Auto,
    Ratio(f32)
}

impl StyleValue for AspectRatio{}

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

    pub justify_items: AlignItems,
    pub justify_content: JustifyContent,
    pub justify_self: AlignItems,

    pub visible: Visibility,
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
    pub margin: MarginDimension,

    /// setting `margin_vertical` sets both `margin_bottom` and `margin_top`
    pub margin_vertical: MarginDimension,
    pub margin_bottom: MarginDimension,
    pub margin_top: MarginDimension,

    /// setting `margin_horizontal` sets both `margin_left` and `margin_right`
    pub margin_horizontal: MarginDimension,
    pub margin_left: MarginDimension,
    pub margin_right: MarginDimension,

    /// when direction is `ltr`, `margin_end` is equivalant to `margin_right`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_left`
    pub margin_end: MarginDimension,
    /// when direction is `ltr`, `margin_end` is equivalant to `margin_left`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_right`
    pub margin_start: MarginDimension,

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

    pub border_width: Dimension,

    pub border_top_width: Dimension,
    pub border_bottom_width: Dimension,
    pub border_left_width: Dimension,
    pub border_right_width: Dimension,

    pub column_gap: Dimension,
    pub row_gap: Dimension,

    pub flex_grow: f64,
    pub flex_shrink: FlexShrink,
    pub flex_basis: Dimension,

    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub min_width: Option<Dimension>,
    pub min_height: Option<Dimension>,
    pub max_width: Option<Dimension>,
    pub max_height: Option<Dimension>,
    pub aspect_ratio: AspectRatio,

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

impl Style {
}

pub struct StyleSheetProvider {}

#[derive(Debug, Default)]
struct FieldMask([AtomicU64; 4]);

impl FieldMask {
    pub const fn new() -> Self{
        Self([AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)])
    }
    pub fn is_set(&self, idx: usize) -> bool {
        let offset = idx % 64;
        let mask_true = 0x1u64 << offset;

        let mask = self.0[idx / 64].load(Ordering::Relaxed);

        return (mask & mask_true) != 0;
    }

    pub fn set(&self, idx: usize, b: bool){
        let offset = idx % 64;

        let mask_loc = 0x1u64 << offset;

        if b{
            self.0[idx / 64].fetch_or(mask_loc, Ordering::Relaxed);
        } else{
            self.0[idx / 64].fetch_and(!mask_loc, Ordering::Relaxed);
        };
    }
}

lazy_static::lazy_static!{
    pub(crate) static ref DEFAULT_STYLESHEET_ARC: Arc<StyleSheet> = Arc::new(StyleSheet::new());
}

/// `StyleSheet` is similar to a CSS stylesheet.
/// 
/// Creating and cloning `StyleSheet` is expensive. It should be reused
/// whenever possible.
/// 
/// It is optimised for memory usage, only properties 
/// that are set explicitly will be stored in the stylesheet.
/// Since properties are allocated dynamically, they cannot be accessed
/// like a field. Instead `get_*` and `set_*` must be called 
/// to access or modify a property. `get_*` methods that returns an `Option` type
/// means that the property inherits from the parent if not set explicitly.
/// 
/// For convenience, `StyleSheet` can be constructed from `Style` structure
/// which contains all the fields on the style sheet, using `From::from` method.
#[derive(Debug, Default)]
pub struct StyleSheet {
    /// mask for active fields.
    /// style is inherited from parent if not active
    owned: FieldMask,
    /// mask for modified fields
    /// 
    modified: FieldMask,
    /// the rest is data
    data: RwLock<Vec<u8>>,
}

unsafe impl Send for StyleSheet{}
unsafe impl Sync for StyleSheet{}

macro_rules! gen_dyn_stylesheet {
    ($(
        $(#[doc=$doc:expr])*
        $field:ident : $ty:ty $(: $inherit:ident)?
    ),*) => {
        paste::paste!{
            impl StyleSheet{
                #[allow(unused)]
                #[allow(unused_assignments)]
                #[inline]
                fn get<T: StyleValue>(&self, name: &str) -> Option<T>{
                    let data = self.data.read();
                    let mut idx = 0;
                    let mut ptr = data.as_ptr();

                    unsafe{
                        $(
                            let inherit = inherit_bool!($($inherit)?);
                            if name == stringify!($field){
                                assert!(core::any::TypeId::of::<T>() == core::any::TypeId::of::<$ty>());

                                if self.owned.is_set(idx){
                                    return Some((ptr as *const T).as_ref().unwrap().clone())
                                } else{
                                    if inherit{
                                        return None;
                                    } else{
                                        return Some(T::default())
                                    }
                                }
                            }
                            if self.owned.is_set(idx){
                                ptr = ptr.add(core::mem::size_of::<$ty>());
                            }
                            
                            idx += 1;
                        )*
                    }
                    unreachable!()
                }

                #[allow(unused_assignments)]
                #[inline]
                fn set<T: StyleValue>(&self, name: &str, value: T){
                    let mut data = self.data.write();
                    let mut idx = 0;
                    let mut offset = 0;

                    unsafe{
                        $(
                            if name == stringify!($field){
                                assert!(core::any::TypeId::of::<T>() == core::any::TypeId::of::<$ty>());

                                let ptr = data.as_mut_ptr().add(offset);

                                if self.owned.is_set(idx){
                                    let r = (ptr as *mut T).as_mut().unwrap();
                                    *r = value;
                                } else{
                                    
                                    let value_ptr = &value as *const T as *const u8;
                                    let value_size = core::mem::size_of::<T>();
                                    // reserve length
                                    data.reserve(value_size);
                                    // get the new pointer
                                    let ptr = data.as_mut_ptr().add(offset);

                                    let need_copy = data.len() - offset;

                                    // copy the old bytes
                                    core::ptr::copy(ptr, ptr.add(value_size), need_copy);
                                    core::ptr::copy_nonoverlapping(value_ptr, ptr, value_size);

                                    let new_len = data.len() + value_size;
                                    // set new len
                                    data.set_len(new_len);

                                    // set field as owned
                                    self.owned.set(idx, true);
                                };

                                self.modified.set(idx, true);

                                return;
                            }
                            if self.owned.is_set(idx){
                                offset += core::mem::size_of::<$ty>();
                            }
                            
                            idx += 1;
                        )*
                    };

                    drop(data);
                    unreachable!()
                }

                #[allow(unused_assignments)]
                #[inline]
                fn updated(&self, name: &str) -> bool{
                    let mut idx = 0;
                    $(
                        if name == stringify!($field){
                            return self.modified.is_set(idx)
                        }
                        idx += 1;
                    )*
                    unreachable!()
                }

                #[allow(unused_assignments)]
                #[inline]
                fn owned(&self, name: &str) -> bool{
                    let mut idx = 0;
                    $(
                        if name == stringify!($field){
                            return self.owned.is_set(idx)
                        }
                        idx += 1;
                    )*
                    unreachable!()
                }

                $(
                    #[allow(unused)]
                    $(#[doc=$doc])*
                    pub fn [<get_ $field:snake>](&self) -> get_return_ty!($($inherit)? $ty)
                    {
                        let inherit = false;
                        $(
                            let $inherit = true;
                        )?
                        let value = self.get(stringify!($field));

                        get_return_value!($($inherit)? value)
                    }
                )*

                $(
                    $(#[doc=$doc])*
                    pub fn [<set_ $field:snake>](&self, value: $ty){
                        self.set(stringify!($field), value)
                    }
                )*

                $(
                    pub fn [<$field:snake _updated>](&self) -> bool{
                        self.updated(stringify!($field))
                    }
                )*

                $(
                    pub fn [<$field:snake _owned>](&self) -> bool{
                        self.owned(stringify!($field))
                    }
                )*
            }

            impl From<&Style> for StyleSheet{
                fn from(style: &Style) -> StyleSheet{
                    let stylesheet = StyleSheet::new();
                    $(
                        if style.$field != Default::default(){
                            stylesheet.[<set_ $field:snake>](style.$field.unwrap());
                        }
                    )*
                    return stylesheet
                }
            }
        }

    };
}

macro_rules! inherit_bool {
    () => {
        false
    };
    (inherit) => {
        true
    }
}

macro_rules! get_return_ty {
    (inherit $ty: ty) => {
        Option<$ty>
    };
    ($ty: ty) => {
        $ty
    }
}

macro_rules! get_return_value {
    (inherit $value:expr) => {
        $value
    };
    ($value:expr) => {
        $value.unwrap_or_default()
    }
}

gen_dyn_stylesheet! {
    visible: Visibility,
    colour: Colour :inherit,
    display: Display,
    direction: Direction : inherit,
    flex_direction: FlexDirection,
    flex_wrap: FlexWrap,
    
    overflow: Overflow,

    align_items: AlignItems,
    align_self: AlignItems : inherit,
    align_content: AlignContent,

    justify_items: AlignItems,
    justify_content: JustifyContent,
    justify_self: AlignItems,

    position: PositionType,

    column_gap: Dimension,
    row_gap: Dimension,

    top: Dimension,
    bottom: Dimension,
    left: Dimension,
    right: Dimension,
    /// when direction is `ltr`, `start` is equivalant to `left`.
    /// when direction is `rtl`, `start` is equivalant to `right`.
    start: Dimension,
    /// when direction is `ltr`, `end` is equivalant to `right`.
    end: Dimension,

    /// setting margin set top, bottom, left, right
    margin: MarginDimension,

    /// setting `margin_vertical` sets both `margin_bottom` and `margin_top`
    margin_vertical: MarginDimension,
    margin_bottom: MarginDimension,
    margin_top: MarginDimension,

    /// setting `margin_horizontal` sets both `margin_left` and `margin_right`
    margin_horizontal: MarginDimension,
    margin_left: MarginDimension,
    margin_right: MarginDimension,

    /// when direction is `ltr`, `margin_end` is equivalant to `margin_right`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_left`
    margin_end: MarginDimension,
    /// when direction is `ltr`, `margin_end` is equivalant to `margin_left`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_right`
    margin_start: MarginDimension,

    padding: Dimension,
    /// setting `margin_vertical` sets both `margin_bottom` and `margin_top`
    padding_vertical: Dimension,
    padding_bottom: Dimension,
    padding_top: Dimension,
    /// setting `margin_horizontal` sets both `margin_left` and `margin_right`
    padding_horizontal: Dimension,
    padding_left: Dimension,
    padding_right: Dimension,
    /// when direction is `ltr`, `margin_end` is equivalant to `margin_right`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_left`
    padding_end: Dimension,
    /// when direction is `ltr`, `margin_end` is equivalant to `margin_left`.
    /// when direction is `rtl`, `margin_end` is equivalant to `margin_right`
    padding_start: Dimension,

    border_width: Dimension,

    border_top_width: Dimension,
    border_bottom_width: Dimension,
    border_left_width: Dimension,
    border_right_width: Dimension,

    flex_grow: f64,
    flex_shrink: FlexShrink,
    flex_basis: Dimension,

    width: Dimension,
    height: Dimension,
    min_width: Dimension,
    min_height: Dimension,
    max_width: Dimension,
    max_height: Dimension,
    aspect_ratio: AspectRatio
}

impl StyleSheet{
    pub const fn new() -> Self{
        Self {
            owned: FieldMask::new(), 
            modified: FieldMask::new(), 
            data: RwLock::new(Vec::new())
        }
    }

    pub(crate) fn set_owned_as_modified(&self){
        self.modified.0[0].store(self.owned.0[0].load(Ordering::Relaxed), Ordering::Relaxed);
        self.modified.0[1].store(self.owned.0[1].load(Ordering::Relaxed), Ordering::Relaxed);
        self.modified.0[2].store(self.owned.0[2].load(Ordering::Relaxed), Ordering::Relaxed);
        self.modified.0[3].store(self.owned.0[3].load(Ordering::Relaxed), Ordering::Relaxed);
    }

    pub(crate) fn to_taffy_style(&self) -> taffy::Style{
        let position_type = self.get_position();

        taffy::Style{
            display: match self.get_display(){
                Display::None => taffy::Display::None,
                Display::Flex => taffy::Display::Flex,
                Display::Block => taffy::Display::Block,
                Display::Grid => taffy::Display::Grid,
            },
            overflow: {
                let f = match self.get_overflow(){
                    Overflow::Hidden => taffy::Overflow::Hidden,
                    Overflow::Visible => taffy::Overflow::Visible
                };
                taffy::Point{
                    x: f,
                    y: f,
                }
            },
            scrollbar_width: 0.0,
            position: match position_type{
                PositionType::Absolute => taffy::Position::Absolute,
                PositionType::Relative => taffy::Position::Relative,
                PositionType::Static => taffy::Position::Relative,
            },
            inset: taffy::Rect::auto(),
            size: 
                taffy::Size { 
                    width: match self.get_width(){
                        Dimension::Auto => taffy::Dimension::Auto,
                        Dimension::Percent(p) => taffy::Dimension::Percent(p.max(1.0)),
                        Dimension::Points(p) => taffy::Dimension::Length(p),
                    }, 
                    height: match self.get_height(){
                        Dimension::Auto => taffy::Dimension::Auto,
                        Dimension::Percent(p) => taffy::Dimension::Percent(p.max(1.0)),
                        Dimension::Points(p) => taffy::Dimension::Length(p),
                    }, 
                },
            min_size: taffy::Size { 
                width: match self.get_min_width(){
                    Dimension::Auto => taffy::Dimension::Auto,
                    Dimension::Percent(p) => taffy::Dimension::Percent(p),
                    Dimension::Points(p) => taffy::Dimension::Length(p),
                }, 
                height: match self.get_min_height(){
                    Dimension::Auto => taffy::Dimension::Auto,
                    Dimension::Percent(p) => taffy::Dimension::Percent(p),
                    Dimension::Points(p) => taffy::Dimension::Length(p),
                }, 
            },
            max_size: taffy::Size { 
                width: match self.get_max_width(){
                    Dimension::Auto => taffy::Dimension::Auto,
                    Dimension::Percent(p) => taffy::Dimension::Percent(p),
                    Dimension::Points(p) => taffy::Dimension::Length(p),
                },
                height: match self.get_max_height(){
                    Dimension::Auto => taffy::Dimension::Auto,
                    Dimension::Percent(p) => taffy::Dimension::Percent(p),
                    Dimension::Points(p) => taffy::Dimension::Length(p),
                }, 
            },
            aspect_ratio: match self.get_aspect_ratio(){
                AspectRatio::Auto => None,
                AspectRatio::Ratio(r) => Some(r)
            },
            margin: taffy::Rect { 
                left: match self.get_margin_left(){
                    MarginDimension::Auto => taffy::LengthPercentageAuto::Auto,
                    MarginDimension::Percent(p) => taffy::LengthPercentageAuto::Percent(p),
                    MarginDimension::Points(p) => taffy::LengthPercentageAuto::Length(p),
                }, 
                right: match self.get_margin_right(){
                    MarginDimension::Auto => taffy::LengthPercentageAuto::Auto,
                    MarginDimension::Percent(p) => taffy::LengthPercentageAuto::Percent(p),
                    MarginDimension::Points(p) => taffy::LengthPercentageAuto::Length(p),
                }, 
                top: match self.get_margin_top(){
                    MarginDimension::Auto => taffy::LengthPercentageAuto::Auto,
                    MarginDimension::Percent(p) => taffy::LengthPercentageAuto::Percent(p),
                    MarginDimension::Points(p) => taffy::LengthPercentageAuto::Length(p),
                }, 
                bottom: match self.get_margin_bottom(){
                    MarginDimension::Auto => taffy::LengthPercentageAuto::Auto,
                    MarginDimension::Percent(p) => taffy::LengthPercentageAuto::Percent(p),
                    MarginDimension::Points(p) => taffy::LengthPercentageAuto::Length(p),
                } 
            },
            padding: taffy::Rect { 
                left: match self.get_padding_left(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p)
                }, 
                right: match self.get_padding_right(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p)
                }, 
                top: match self.get_padding_top(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p)
                }, 
                bottom: match self.get_padding_bottom(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p)
                } 
            },
            border: taffy::Rect { 
                left: match self.get_border_left_width(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p),
                }, 
                right: match self.get_border_right_width(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p),
                }, 
                top: match self.get_border_top_width(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p),
                }, 
                bottom: match self.get_border_bottom_width(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p),
                }
            },
            align_items: match self.get_align_items(){
                AlignItems::Normal => None,
                AlignItems::Baseline => Some(taffy::AlignItems::Baseline),
                AlignItems::Centre => Some(taffy::AlignItems::Center),
                AlignItems::Start => Some(taffy::AlignItems::Start),
                AlignItems::End => Some(taffy::AlignItems::End),
                AlignItems::FlexStart => Some(taffy::AlignItems::FlexStart),
                AlignItems::FlexEnd => Some(taffy::AlignItems::FlexEnd),
                AlignItems::Stretch => Some(taffy::AlignItems::Stretch)
            },
            align_self: match self.get_align_self(){
                None => None,
                Some(AlignItems::Normal) => Some(taffy::AlignItems::Stretch),
                Some(AlignItems::Baseline) => Some(taffy::AlignItems::Baseline),
                Some(AlignItems::Centre) => Some(taffy::AlignItems::Center),
                Some(AlignItems::Start) => Some(taffy::AlignItems::Start),
                Some(AlignItems::End) => Some(taffy::AlignItems::End),
                Some(AlignItems::FlexStart) => Some(taffy::AlignItems::FlexStart),
                Some(AlignItems::FlexEnd) => Some(taffy::AlignItems::FlexEnd),
                Some(AlignItems::Stretch) => Some(taffy::AlignItems::Stretch)
            },
            justify_items: match self.get_justify_items(){
                AlignItems::Normal => None,
                AlignItems::Baseline => Some(taffy::AlignItems::Baseline),
                AlignItems::Centre => Some(taffy::AlignItems::Center),
                AlignItems::Start => Some(taffy::AlignItems::Start),
                AlignItems::End => Some(taffy::AlignItems::End),
                AlignItems::FlexStart => Some(taffy::AlignItems::FlexStart),
                AlignItems::FlexEnd => Some(taffy::AlignItems::FlexEnd),
                AlignItems::Stretch => Some(taffy::AlignItems::Stretch)
            },
            justify_content: match self.get_justify_content(){
                JustifyContent::Normal => None,
                JustifyContent::Center => Some(taffy::AlignContent::Center),
                JustifyContent::FlexEnd => Some(taffy::AlignContent::FlexEnd),
                JustifyContent::FlexStart => Some(taffy::AlignContent::FlexEnd),
                JustifyContent::SpaceAround => Some(taffy::AlignContent::SpaceAround),
                JustifyContent::SpaceBetween => Some(taffy::AlignContent::SpaceBetween),
                JustifyContent::SpaceEvenly => Some(taffy::AlignContent::SpaceEvenly),
                JustifyContent::Start => Some(taffy::AlignContent::Start),
                JustifyContent::End =>  Some(taffy::AlignContent::End),
                JustifyContent::Stretch => Some(taffy::AlignContent::Stretch),
            },
            justify_self: match self.get_justify_self(){
                AlignItems::Normal => None,
                AlignItems::Baseline => Some(taffy::AlignItems::Baseline),
                AlignItems::Centre => Some(taffy::AlignItems::Center),
                AlignItems::Start => Some(taffy::AlignItems::Start),
                AlignItems::End => Some(taffy::AlignItems::End),
                AlignItems::FlexStart => Some(taffy::AlignItems::FlexStart),
                AlignItems::FlexEnd => Some(taffy::AlignItems::FlexEnd),
                AlignItems::Stretch => Some(taffy::AlignItems::Stretch)
            },
            align_content: match self.get_align_content(){
                AlignContent::Normal => None,
                AlignContent::Centre => Some(taffy::AlignContent::Center),
                AlignContent::Start => Some(taffy::AlignContent::Start),
                AlignContent::End => Some(taffy::AlignContent::End),
                AlignContent::FlexStart => Some(taffy::AlignContent::FlexStart),
                AlignContent::FlexEnd => Some(taffy::AlignContent::FlexEnd),
                AlignContent::SpaceAround => Some(taffy::AlignContent::SpaceAround),
                AlignContent::SpaceBetween => Some(taffy::AlignContent::SpaceBetween),
                AlignContent::SpaceEvenly => Some(taffy::AlignContent::SpaceEvenly),
                AlignContent::Stretch => Some(taffy::AlignContent::Stretch),
            },
            gap: taffy::Size { 
                width: match self.get_column_gap(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p),
                }, 
                height: match self.get_row_gap(){
                    Dimension::Auto => taffy::LengthPercentage::Length(0.0),
                    Dimension::Percent(p) => taffy::LengthPercentage::Percent(p),
                    Dimension::Points(p) => taffy::LengthPercentage::Length(p),
                }
            },
            flex_direction: match self.get_flex_direction(){
                FlexDirection::Column => taffy::FlexDirection::Column,
                FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
                FlexDirection::Row => taffy::FlexDirection::Row,
                FlexDirection::RowReverse => taffy::FlexDirection::RowReverse
            },
            flex_wrap: match self.get_flex_wrap(){
                FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
                FlexWrap::Wrap => taffy::FlexWrap::Wrap,
                FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse
            },
            flex_basis: match self.get_flex_basis(){
                Dimension::Auto => taffy::Dimension::Auto,
                Dimension::Percent(p) => taffy::Dimension::Percent(p),
                Dimension::Points(p) => taffy::Dimension::Length(p)        
            },
            flex_grow: self.get_flex_grow() as f32,
            flex_shrink: self.get_flex_shrink().0 as f32,
            ..Default::default()
        }
    }
}

#[test]
fn test_stylesheet(){
    let sheet = StyleSheet::new();

    sheet.set_align_content(AlignContent::Centre);
    sheet.set_aspect_ratio(AspectRatio::Ratio(98.0));

    assert!(sheet.get_align_content() == AlignContent::Centre);
    assert!(sheet.get_aspect_ratio() == AspectRatio::Ratio(98.0));
    assert!(sheet.get_align_self() == None);
    
    assert!(sheet.align_content_updated());
    assert!(sheet.aspect_ratio_updated());
}