use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Index;
use std::ops::IndexMut;

use starlib_proto::ImageData;

use crate::geom::PointI;
use crate::geom::SizeI;
use crate::grid::Pixel;
use crate::grid::Scalar;
use crate::traits::FromProto;
use crate::traits::IntoSc2;

// -----------------------------------------------------------------------------
// Pixel Grid
// -----------------------------------------------------------------------------

pub struct PixelGrid<P = u8>
where
  P: Pixel,
{
  size: SizeI,
  data: Vec<P::Inner>,
}

impl<P> PixelGrid<P>
where
  P: Pixel,
{
  #[inline]
  pub fn new(size: SizeI) -> Self {
    let mut this: Self = Self {
      size,
      data: Vec::new(),
    };

    this.resize_buffer();
    this
  }

  #[inline]
  pub fn size(&self) -> SizeI {
    self.size
  }

  #[inline]
  pub fn width(&self) -> i32 {
    self.size.w
  }

  #[inline]
  pub fn height(&self) -> i32 {
    self.size.h
  }

  #[inline]
  pub const fn display(&self) -> PrintGrid<'_, P> {
    PrintGrid(self)
  }

  pub fn update(&mut self, source: &ImageData) {
    if let Some(size) = source.size {
      self.size = size.into_sc2();
    }

    self.resize_buffer();

    match source.bits_per_pixel {
      1 => {
        #[inline]
        fn expand<P>(byte: u8) -> impl Iterator<Item = P::Inner>
        where
          P: Pixel,
        {
          (0..u8::BITS)
            .rev()
            .map(move |index| P::Inner::cast((byte >> index) & 1))
        }

        assert_eq!(self.data.len(), source.data.len() * u8::BITS as usize);

        self.data.clear();

        self
          .data
          .extend(source.data.iter().copied().flat_map(expand::<P>));
      }
      8 => {
        assert_eq!(self.data.len(), source.data.len());

        self
          .data
          .copy_from_slice(P::Inner::cast_slice(&source.data));
      }
      bpp => {
        panic!("Invalid BPP (expected 1 or 8, received {bpp})");
      }
    }
  }

  #[inline]
  fn resize_buffer(&mut self) {
    assert!(self.size.w >= 0);
    assert!(self.size.h >= 0);

    let size: u32 = self.size.w as u32 * self.size.h as u32;
    let with: P::Inner = P::Inner::default();

    self.data.resize(size as usize, with);
  }

  #[inline]
  const fn to_index(&self, x: i32, y: i32) -> usize {
    (x + y * self.size.w) as usize
  }
}

impl<P> Debug for PixelGrid<P>
where
  P: Pixel,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Grid{}", self.size)
  }
}

impl<P> Display for PixelGrid<P>
where
  P: Pixel,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.size)
  }
}

impl<P> Index<(i32, i32)> for PixelGrid<P>
where
  P: Pixel,
{
  type Output = P;

  #[inline]
  fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
    P::cast_ref(self.data.index(self.to_index(x, y)))
  }
}

impl<P> IndexMut<(i32, i32)> for PixelGrid<P>
where
  P: Pixel,
{
  #[inline]
  fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
    P::cast_mut(self.data.index_mut(self.to_index(x, y)))
  }
}

impl<P> Index<PointI> for PixelGrid<P>
where
  P: Pixel,
{
  type Output = P;

  #[inline]
  fn index(&self, index: PointI) -> &Self::Output {
    self.index((index.x, index.y))
  }
}

impl<P> IndexMut<PointI> for PixelGrid<P>
where
  P: Pixel,
{
  #[inline]
  fn index_mut(&mut self, index: PointI) -> &mut Self::Output {
    self.index_mut((index.x, index.y))
  }
}

impl<P> FromProto<ImageData> for PixelGrid<P>
where
  P: Pixel,
{
  #[inline]
  fn from_proto(other: ImageData) -> Self {
    let mut this: Self = Self::new(Default::default());
    this.update(&other);
    this
  }
}

// -----------------------------------------------------------------------------
// Print Grid
// -----------------------------------------------------------------------------

#[repr(transparent)]
pub struct PrintGrid<'a, P>(&'a PixelGrid<P>)
where
  P: Pixel;

impl<'a, P> Display for PrintGrid<'a, P>
where
  P: Pixel,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    const BTL: char = '╭';
    const BTR: char = '╮';
    const BBL: char = '╰';
    const BBR: char = '╯';

    const HL: char = '─';
    const VL: char = '│';

    let size: SizeI = self.0.size();
    let line: String = HL.to_string();

    writeln!(f, "PixelGrid({size})")?;

    writeln!(f, "{BTL}{}{BTR}", line.repeat(size.w as usize))?;

    for row in self.0.data.chunks_exact(size.w as usize).rev() {
      write!(f, "{VL}")?;

      for pixel in row {
        P::fmt(pixel, f)?;
      }

      write!(f, "{VL}")?;
      write!(f, "\n")?;
    }

    writeln!(f, "{BBL}{}{BBR}", line.repeat(size.w as usize))?;

    Ok(())
  }
}
