use std::sync::Arc;

use parking_lot::RwLock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageFormat {
    #[default]
    Rgba8,
}

impl ImageFormat {
    pub fn bit_per_sample(&self) -> usize {
        match self {
            Self::Rgba8 => 8 * 4,
        }
    }
}

pub trait ImageSource: Sync + Send {
    /// returns the width and height of the current image
    fn size(&self) -> (u32, u32);
    /// the preferred format to snapshot the image
    fn preferred_format(&self) -> ImageFormat {
        ImageFormat::Rgba8
    }
    /// snapshot the image,
    /// writes image into the preallocated buffer using the preffered format
    fn snapshot(&self, buf: &mut [u8]);
    /// freeze the image, image should not change during a freeze
    fn freeze(&self);
    /// unfreeze the image, image can be changed
    fn unfreeze(&self);
    /// true if the image is up to date since the last snapshot.
    ///
    /// should always return true for static image
    fn is_up_to_date(&self) -> bool {
        return true;
    }
}

impl<T> ImageSource for &'static T
where
    T: ImageSource,
{
    fn size(&self) -> (u32, u32) {
        <T as ImageSource>::size(&self)
    }
    fn preferred_format(&self) -> ImageFormat {
        <T as ImageSource>::preferred_format(&self)
    }
    fn freeze(&self) {
        <T as ImageSource>::freeze(&self)
    }
    fn unfreeze(&self) {
        <T as ImageSource>::unfreeze(&self)
    }
    fn snapshot(&self, buf: &mut [u8]) {
        <T as ImageSource>::snapshot(&self, buf)
    }
    fn is_up_to_date(&self) -> bool {
        <T as ImageSource>::is_up_to_date(&self)
    }
}

impl<T> ImageSource for &'static RwLock<T>
where
    T: ImageSource,
{
    fn size(&self) -> (u32, u32) {
        <T as ImageSource>::size(&self.read())
    }
    fn preferred_format(&self) -> ImageFormat {
        <T as ImageSource>::preferred_format(&self.read())
    }
    fn freeze(&self) {
        <T as ImageSource>::freeze(&self.read())
    }
    fn unfreeze(&self) {
        <T as ImageSource>::unfreeze(&self.read())
    }
    fn snapshot(&self, buf: &mut [u8]) {
        <T as ImageSource>::snapshot(&self.read(), buf)
    }
    fn is_up_to_date(&self) -> bool {
        <T as ImageSource>::is_up_to_date(&self.read())
    }
}

impl<T> ImageSource for Arc<T>
where
    T: ImageSource,
{
    fn size(&self) -> (u32, u32) {
        <T as ImageSource>::size(&self)
    }
    fn preferred_format(&self) -> ImageFormat {
        <T as ImageSource>::preferred_format(&self)
    }
    fn freeze(&self) {
        <T as ImageSource>::freeze(&self)
    }
    fn unfreeze(&self) {
        <T as ImageSource>::unfreeze(&self)
    }
    fn snapshot(&self, buf: &mut [u8]) {
        <T as ImageSource>::snapshot(&self, buf)
    }
    fn is_up_to_date(&self) -> bool {
        <T as ImageSource>::is_up_to_date(&self)
    }
}

impl<T> ImageSource for Arc<RwLock<T>>
where
    T: ImageSource,
{
    fn size(&self) -> (u32, u32) {
        <T as ImageSource>::size(&self.read())
    }
    fn preferred_format(&self) -> ImageFormat {
        <T as ImageSource>::preferred_format(&self.read())
    }
    fn freeze(&self) {
        <T as ImageSource>::freeze(&self.read())
    }
    fn unfreeze(&self) {
        <T as ImageSource>::unfreeze(&self.read())
    }
    fn snapshot(&self, buf: &mut [u8]) {
        <T as ImageSource>::snapshot(&self.read(), buf)
    }
    fn is_up_to_date(&self) -> bool {
        <T as ImageSource>::is_up_to_date(&self.read())
    }
}

pub struct RGBA8Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl RGBA8Image {
    pub fn new(width: u32, height: u32) -> Self {
        let mut v = Vec::with_capacity((width * height * 4) as usize);
        unsafe { v.set_len((width * height * 4) as usize) };
        v.fill(0);

        Self {
            width: width,
            height: height,
            data: v,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
}

impl ImageSource for RGBA8Image {
    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    fn preferred_format(&self) -> ImageFormat {
        ImageFormat::Rgba8
    }
    fn freeze(&self) {}
    fn unfreeze(&self) {}
    fn snapshot(&self, buf: &mut [u8]) {
        // buffer length should be the same
        debug_assert!(buf.len() == self.data.len());

        buf.copy_from_slice(&self.data);
    }
}

impl ImageSource for image::RgbaImage {
    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    fn preferred_format(&self) -> ImageFormat {
        ImageFormat::Rgba8
    }
    fn is_up_to_date(&self) -> bool {
        true
    }
    fn freeze(&self) {}
    fn unfreeze(&self) {}
    fn snapshot(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self)
    }
}
