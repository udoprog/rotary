use crate::buf::Buf;
use crate::frame::Frame;

/// A buffer which has a unifom channel size.
pub trait UniformBuf: Buf {
    /// The type the channel assumes when coerced into a reference.
    type Frame<'this>: Frame<Sample = Self::Sample>
    where
        Self: 'this;

    /// A borrowing iterator over the channel.
    type FramesIter<'this>: Iterator<Item = Self::Frame<'this, Sample = Self::Sample>>
    where
        Self: 'this;

    /// Get a single frame at the given offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use audio::{Frame, UniformBuf};
    ///
    /// fn test<B>(buf: B)
    /// where
    ///     B: UniformBuf,
    ///     B::Sample: std::fmt::Debug + PartialEq<u32>
    /// {
    ///     let frame = buf.get_frame(0).unwrap();
    ///     assert_eq!(frame.iter().collect::<Vec<_>>(), [1, 5]);
    ///
    ///     let frame = buf.get_frame(2).unwrap();
    ///     assert_eq!(frame.iter().collect::<Vec<_>>(), [3, 7]);
    ///
    ///     assert!(buf.get_frame(4).is_none());
    /// }
    ///
    /// test(audio::sequential![[1, 2, 3, 4], [5, 6, 7, 8]]);
    /// test(audio::interleaved![[1, 2, 3, 4], [5, 6, 7, 8]]);
    /// ```
    fn get_frame(&self, frame: usize) -> Option<Self::Frame<'_>>;

    /// Construct an iterator over all the frames in the audio buffer.
    fn iter_frames(&self) -> Self::FramesIter<'_>;
}
