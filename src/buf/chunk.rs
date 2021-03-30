use crate::buf::{Buf, BufInfo, BufMut, Channel, ChannelMut, ResizableBuf};
use crate::sample::Sample;

/// A chunk of another buffer.
///
/// Created with [Buf::chunk].
pub struct Chunk<B> {
    buf: B,
    n: usize,
    len: usize,
}

impl<B> Chunk<B> {
    /// Construct a new limited buffer.
    pub(crate) fn new(buf: B, n: usize, len: usize) -> Self {
        Self { buf, n, len }
    }
}

impl<B> BufInfo for Chunk<B>
where
    B: BufInfo,
{
    fn buf_info_frames(&self) -> usize {
        self.buf.buf_info_frames().saturating_sub(self.n * self.len)
    }

    fn buf_info_channels(&self) -> usize {
        self.buf.buf_info_channels()
    }
}

impl<B, T> Buf<T> for Chunk<B>
where
    B: Buf<T>,
    T: Sample,
{
    fn channel(&self, channel: usize) -> Channel<'_, T> {
        self.buf.channel(channel).chunk(self.n, self.len)
    }
}

impl<B> ResizableBuf for Chunk<B>
where
    B: ResizableBuf,
{
    fn resize(&mut self, frames: usize) {
        let frames = frames.saturating_add(self.n).saturating_mul(self.len);
        self.buf.resize(frames);
    }

    fn resize_topology(&mut self, channels: usize, frames: usize) {
        let frames = frames.saturating_add(self.n).saturating_mul(self.len);
        self.buf.resize_topology(channels, frames);
    }
}

impl<B, T> BufMut<T> for Chunk<B>
where
    B: BufMut<T>,
    T: Sample,
{
    fn channel_mut(&mut self, channel: usize) -> ChannelMut<'_, T> {
        self.buf.channel_mut(channel).chunk(self.n, self.len)
    }
}
