use std::sync::Arc;

use glib::subclass::types::ObjectSubclassExt;
use gtk4::prelude::PaintableExt;

use crate::{image::ImageSource, util::Comparable};

mod imp {
    use std::cell::RefCell;
    use std::ops::DerefMut;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;

    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};

    use gtk4::gdk::subclass::paintable::{PaintableImpl, PaintableImplExt};
    use gtk4::gdk::Texture;
    use gtk4::gdk::{Paintable, PaintableFlags, Snapshot};
    use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
    use gtk4::graphene::Rect;
    use gtk4::prelude::*;

    use crate::image::ImageSource;
    use crate::util::Comparable;

    #[derive(Default)]
    pub struct UINativeImage {
        pub(super) inner: RefCell<UINativeImageInner>,
        pub(super) aspect_radio: AtomicU64,
    }

    #[derive(Default)]
    pub(super) struct UINativeImageInner {
        pub(super) src: Option<Arc<Comparable<dyn ImageSource>>>,
        texture: Option<Texture>,
        buffer: Vec<u8>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UINativeImage {
        const NAME: &'static str = "UINativeImage";
        type Type = super::UINativeImage;
        type Interfaces = (Paintable,);
    }

    impl ObjectImpl for UINativeImage {}

    impl PaintableImpl for UINativeImage {
        #[doc(alias = "get_current_image")]
        fn current_image(&self) -> Paintable {
            self.parent_current_image()
        }

        #[doc(alias = "get_flags")]
        fn flags(&self) -> PaintableFlags {
            PaintableFlags::empty()
        }

        #[doc(alias = "get_intrinsic_width")]
        fn intrinsic_width(&self) -> i32 {
            let inner = self.inner.borrow();

            if let Some(src) = &inner.src {
                src.size().0 as i32
            } else {
                0
            }
        }

        #[doc(alias = "get_intrinsic_height")]
        fn intrinsic_height(&self) -> i32 {
            let inner = self.inner.borrow();

            if let Some(src) = &inner.src {
                src.size().1 as i32
            } else {
                0
            }
        }

        #[doc(alias = "get_intrinsic_aspect_ratio")]
        fn intrinsic_aspect_ratio(&self) -> f64 {
            let asp = self.aspect_radio.load(Ordering::Relaxed);

            if asp == 0 {
                let inner = self.inner.borrow();

                if let Some(src) = &inner.src {
                    let (w, h) = src.size();

                    return w as f64 / h as f64;
                }
            }

            return f64::from_bits(asp);
        }

        fn snapshot(&self, snapshot: &Snapshot, width: f64, height: f64) {
            let mut inner = self.inner.borrow_mut();
            let inner = inner.deref_mut();

            if let Some(src) = &inner.src {
                if src.is_up_to_date() {
                    if let Some(texture) = &inner.texture {
                        // append texture to the snapshot
                        snapshot.append_texture(
                            texture,
                            &Rect::new(0.0, 0.0, width as f32, height as f32),
                        );

                        return;
                    }
                }

                // freeze the source
                src.freeze();

                // get the buffer format of source
                let format = src.preferred_format();
                // get the width and height of source
                let (src_width, src_height) = src.size();

                // drop the texture
                inner.texture = None;

                unsafe {
                    // clear buffer
                    inner.buffer.set_len(0);
                    // reserve size
                    inner.buffer.reserve(
                        format.bit_per_sample() / 8 * src_width as usize * src_height as usize,
                    );
                    // set length
                    inner.buffer.set_len(
                        format.bit_per_sample() / 8 * src_width as usize * src_height as usize,
                    );
                }

                // snapshot the image
                src.snapshot(&mut inner.buffer);
                // unfreeze the source
                src.unfreeze();

                // create a new pixuf
                let pixbuf = Pixbuf::from_mut_slice(
                    &mut inner.buffer,
                    Colorspace::Rgb,
                    true,
                    8,
                    src_width as i32,
                    src_height as i32,
                    src_width as i32 * (format.bit_per_sample() as i32 / 8),
                );

                // create texture for pixbuf
                let tex = Texture::for_pixbuf(&pixbuf);

                // append texture to the snapshot
                snapshot.append_texture(&tex, &Rect::new(0.0, 0.0, width as f32, height as f32));

                // store the texture
                inner.texture = Some(tex);
            }
        }
    }
}

glib::wrapper! {
    pub struct UINativeImage(ObjectSubclass<imp::UINativeImage>)
    @implements gtk4::gdk::Paintable;
}

impl UINativeImage {
    pub fn new(src: Arc<Comparable<dyn ImageSource>>) -> Self {
        let obj = glib::Object::new();

        let img = imp::UINativeImage::from_obj(&obj);
        img.inner.borrow_mut().src.replace(src);

        return obj;
    }

    pub fn set_source(&self, src: Arc<Comparable<dyn ImageSource>>) {
        let img = imp::UINativeImage::from_obj(self);
        img.inner.borrow_mut().src.replace(src);

        self.invalidate_contents();
        self.invalidate_size();
    }

    pub fn check_update(&self) {
        let img = imp::UINativeImage::from_obj(self);
        let inner = img.inner.borrow();

        if let Some(src) = &inner.src {
            if !src.is_up_to_date() {
                self.invalidate_contents();
            }
        }
    }
}
