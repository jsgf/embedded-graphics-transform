//! Add simple coordinate transforms for embedded graphics displays
//!
//! This crate adds [`DrawTarget`] implementations which apply various simple
//! transformations to coordinates as they're being written. This allows
//! graphics output to be rotated or mirrored to display correctly on a specific
//! display device.
//!
//! Specifically, it implements:
//! - rotation by 90/180/270 degrees (and 0, for consistency)
//! - mirroring
//! - transposition
//!
//! Note that these transformations can be composed if needed.
//! 
//! Because this is a completely generic implementation, it cannot take
//! advantage of any hardware or driver specific specializations. In particular,
//! [`DrawTarget::fill_contiguous`] must fall back to a generic implementation
//! using [`draw_iter`](DrawTarget::draw_iter). ([`fill_solid`](DrawTarget::fill_solid) and
//! [`clear`](DrawTarget::clear) can use specialized implementations, however.)
#![no_std]

use embedded_graphics_core::{prelude::*, primitives::Rectangle};

macro_rules! xform_type {
    ($inner:ident , ) => { $inner };
    ($inner:ident , $xform: ident $($rest:ident)*) => {
        r#impl::$xform < xform_type!($inner, $($rest)*) >
    };
}

macro_rules! xform_new {
    ($inner:ident , ) => {
        $inner
    };
    ($inner:ident , $xform:ident $($rest:ident)*) => {
        r#impl::$xform::new(xform_new!($inner, $($rest)*))
    };
}

macro_rules! impl_xform {
    ($(#[$attr:meta])* $name:ident : $($xforms:ident)*) => {
        $(#[$attr])*
        pub struct $name<D> {
            target: xform_type!(D, $($xforms)*)
        }

        impl<D> $name<D> {
            /// Apply a transformation to display implementing [`DrawTarget`].
            pub fn new(target: D) -> Self {
                $name {
                    target: xform_new!(target, $($xforms)*)
                }
            }
        }

        impl<D: Dimensions> Dimensions for $name<D> {
            #[inline]
            fn bounding_box(&self) -> Rectangle {
                self.target.bounding_box()
            }
        }

        impl<D: DrawTarget> DrawTarget for $name<D> {
            type Color = D::Color;
            type Error = D::Error;

            #[inline]
            fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
            where
                I: IntoIterator<Item = Pixel<Self::Color>>,
            {
                self.target.draw_iter(pixels)
            }

            #[inline]
            fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
            where
                I: IntoIterator<Item = Self::Color>,
            {
                self.target.fill_contiguous(area, colors)
            }

            #[inline]
            fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
                self.target.fill_solid(area, color)
            }

            #[inline]
            fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
                self.target.clear(color)
            }
        }
    }
}

// Define rotations in terms of transpose and flip. Note that transforms are
// applied in order from last to first.
impl_xform! {
    /// No-op (identity) rotation for completeness.
    Rotate0:
}
impl_xform! {
    /// Rotate image 90 degrees to the right.
    Rotate90: MirrorX TransposeXY
}
impl_xform! {
    /// Rotate image 90 degrees to the left.
    Rotate270: MirrorY TransposeXY
}
impl_xform! {
    /// Rotate image 180 degrees.
    Rotate180: MirrorX MirrorY
}

// Transpose and Mirror
impl_xform! {
    /// Transpose X and Y coordinates.
    Transpose: TransposeXY
}
impl_xform! {
    /// Mirror image around X axis.
    FlipX: MirrorX
}
impl_xform! {
    /// Mirror image around Y axis.
    FlipY: MirrorY
}

/// Image rotation direction and amount.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Rotation {
    /// No-op (identity) rotation.
    Rotate0,
    /// Rotate 90 degrees to the right.
    Rotate90,
    /// Rotate 180 degrees.
    Rotate180,
    /// Rotate 90 degrees to the left.
    Rotate270,
}

enum RotateInner<D> {
    Rotate0(Rotate0<D>),
    Rotate90(Rotate90<D>),
    Rotate180(Rotate180<D>),
    Rotate270(Rotate270<D>),
}

/// Rotate an image with runtime configuration.
///
/// Unlike the [`Rotate90`]/[`Rotate180`]/[`Rotate270`] types, this allows
/// rotation to be defined as a runtime paramter. It is simply a wrapper over
/// the other implementations, so it should be functionally identical. The only
/// overhead is the cost of dispatching to the appropriate implementation on
/// each call.
pub struct Rotate<D> {
    target: RotateInner<D>,
}

impl<D> Rotate<D> {
    pub fn new(rot: Rotation, target: D) -> Self {
        let target = match rot {
            Rotation::Rotate0 => RotateInner::Rotate0(Rotate0::new(target)),
            Rotation::Rotate90 => RotateInner::Rotate90(Rotate90::new(target)),
            Rotation::Rotate180 => RotateInner::Rotate180(Rotate180::new(target)),
            Rotation::Rotate270 => RotateInner::Rotate270(Rotate270::new(target)),
        };
        Rotate { target }
    }
}

impl<D: Dimensions> Dimensions for Rotate<D> {
    fn bounding_box(&self) -> Rectangle {
        match &self.target {
            RotateInner::Rotate0(inner) => inner.bounding_box(),
            RotateInner::Rotate90(inner) => inner.bounding_box(),
            RotateInner::Rotate180(inner) => inner.bounding_box(),
            RotateInner::Rotate270(inner) => inner.bounding_box(),
        }
    }
}

impl<D: DrawTarget> DrawTarget for Rotate<D> {
    type Color = D::Color;
    type Error = D::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        match &mut self.target {
            RotateInner::Rotate0(inner) => inner.draw_iter(pixels),
            RotateInner::Rotate90(inner) => inner.draw_iter(pixels),
            RotateInner::Rotate180(inner) => inner.draw_iter(pixels),
            RotateInner::Rotate270(inner) => inner.draw_iter(pixels),
        }
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        match &mut self.target {
            RotateInner::Rotate0(inner) => inner.fill_contiguous(area, colors),
            RotateInner::Rotate90(inner) => inner.fill_contiguous(area, colors),
            RotateInner::Rotate180(inner) => inner.fill_contiguous(area, colors),
            RotateInner::Rotate270(inner) => inner.fill_contiguous(area, colors),
        }
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        match &mut self.target {
            RotateInner::Rotate0(inner) => inner.fill_solid(area, color),
            RotateInner::Rotate90(inner) => inner.fill_solid(area, color),
            RotateInner::Rotate180(inner) => inner.fill_solid(area, color),
            RotateInner::Rotate270(inner) => inner.fill_solid(area, color),
        }
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        match &mut self.target {
            RotateInner::Rotate0(inner) => inner.clear(color),
            RotateInner::Rotate90(inner) => inner.clear(color),
            RotateInner::Rotate180(inner) => inner.clear(color),
            RotateInner::Rotate270(inner) => inner.clear(color),
        }
    }
}

mod r#impl {
    use embedded_graphics_core::{prelude::*, primitives::Rectangle};

    trait Transpose {
        fn transpose(self) -> Self;
    }

    impl Transpose for Point {
        #[inline]
        fn transpose(self) -> Point {
            Point {
                x: self.y,
                y: self.x,
            }
        }
    }

    impl Transpose for Size {
        #[inline]
        fn transpose(self) -> Size {
            Size {
                width: self.height,
                height: self.width,
            }
        }
    }

    impl Transpose for Rectangle {
        #[inline]
        fn transpose(self) -> Rectangle {
            Rectangle {
                top_left: self.top_left.transpose(),
                size: self.size.transpose(),
            }
        }
    }

    pub(crate) struct TransposeXY<D> {
        target: D,
    }

    impl<D> TransposeXY<D> {
        pub(crate) fn new(target: D) -> Self {
            TransposeXY { target }
        }
    }

    impl<D: Dimensions> Dimensions for TransposeXY<D> {
        fn bounding_box(&self) -> Rectangle {
            self.target.bounding_box().transpose()
        }
    }

    impl<D: DrawTarget> DrawTarget for TransposeXY<D> {
        type Color = D::Color;
        type Error = D::Error;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            self.target.draw_iter(
                pixels
                    .into_iter()
                    .map(|Pixel(loc, col)| Pixel(loc.transpose(), col)),
            )
        }

        fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
            let area = area.transpose();
            self.target.fill_solid(&area, color)
        }

        #[inline]
        fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
            self.target.clear(color)
        }
    }

    pub(crate) struct MirrorX<D> {
        target: D,
    }

    impl<D> MirrorX<D> {
        pub(crate) fn new(target: D) -> Self {
            MirrorX { target }
        }
    }

    impl<D: Dimensions> Dimensions for MirrorX<D> {
        #[inline]
        fn bounding_box(&self) -> Rectangle {
            self.target.bounding_box()
        }
    }

    impl<D: DrawTarget> DrawTarget for MirrorX<D> {
        type Color = D::Color;
        type Error = D::Error;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            let width = self.bounding_box().size.width as i32;

            self.target.draw_iter(
                pixels
                    .into_iter()
                    .map(|Pixel(Point { x, y }, col)| Pixel(Point { x: width - x, y }, col)),
            )
        }

        fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
            let area = area.transpose();
            self.target.fill_solid(&area, color)
        }

        #[inline]
        fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
            self.target.clear(color)
        }
    }

    pub(crate) struct MirrorY<D> {
        target: D,
    }

    impl<D> MirrorY<D> {
        pub(crate) fn new(target: D) -> Self {
            MirrorY { target }
        }
    }

    impl<D: Dimensions> Dimensions for MirrorY<D> {
        #[inline]
        fn bounding_box(&self) -> Rectangle {
            self.target.bounding_box()
        }
    }

    impl<D: DrawTarget> DrawTarget for MirrorY<D> {
        type Color = D::Color;
        type Error = D::Error;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            let height = self.bounding_box().size.height as i32;

            self.target.draw_iter(
                pixels
                    .into_iter()
                    .map(|Pixel(Point { x, y }, col)| Pixel(Point { x, y: height - y }, col)),
            )
        }

        fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
            let area = area.transpose();
            self.target.fill_solid(&area, color)
        }

        #[inline]
        fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
            self.target.clear(color)
        }
    }
}
