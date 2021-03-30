use crate::buf::{Buf, BufInfo, BufMut, ReadBuf};
use crate::sample::Sample;
use crate::translate::Translate;

/// A writer abstraction allowing a caller to keep track of how many frames are
/// remaining to write.
///
/// You can access the writable slice of the underlying buffer through
/// [Write::write].
pub struct Write<B> {
    buf: B,
    available: usize,
}

impl<B> Write<B>
where
    B: BufInfo,
{
    pub(super) fn new(buf: B) -> Self {
        let available = buf.buf_info_frames();
        Self { buf, available }
    }

    /// Access the underlying buffer immutably.
    pub fn as_ref(&self) -> &B {
        &self.buf
    }

    /// Access the underlying buffer mutably.
    pub fn as_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Test if buffer has remaining data.
    pub fn has_remaining_mut(&self) -> bool {
        self.available > 0
    }

    /// Remaining number of frames available.
    pub fn remaining_mut(&self) -> usize {
        self.available
    }

    /// Write to the underlying buffer.
    pub fn copy<T, I>(&mut self, mut buf: I)
    where
        B: BufMut<T>,
        T: Sample,
        I: ReadBuf + Buf<T>,
    {
        let len = usize::min(self.available, buf.buf_info_frames());
        crate::utils::copy(&buf, (&mut self.buf).tail(self.available));
        self.available = self.available.saturating_sub(len);
        buf.advance(len);
    }

    /// Write translated samples to the underlying buffer.
    pub fn translate<T, I, U>(&mut self, mut buf: I)
    where
        B: BufMut<T>,
        T: Sample + Translate<U>,
        I: ReadBuf + Buf<U>,
        U: Sample,
    {
        let len = usize::min(self.available, buf.remaining());
        crate::utils::translate(&buf, (&mut self.buf).tail(self.available));
        self.available = self.available.saturating_sub(len);
        buf.advance(len);
    }
}
