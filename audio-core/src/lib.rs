//! The core [audio] traits.
//!
//! If you want to build an audio component that is completely agnostic to the
//! shape of any one given audio buffer you can add a dependency directly to
//! these traits instead of depending on all of the [audio] crate.
//!
//! [audio]: https://docs.rs/audio

#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![allow(clippy::should_implement_trait)]
#![feature(generic_associated_types)]

mod buf;
pub use self::buf::{
    AsInterleaved, AsInterleavedMut, Buf, ExactSizeBuf, InterleavedBuf, ResizableBuf,
};

mod channels;
pub use self::channels::Channels;

mod channels_mut;
pub use self::channels_mut::ChannelsMut;

mod channel;
pub use self::channel::Channel;

mod channel_mut;
pub use self::channel_mut::ChannelMut;

mod interleaved_channel_mut;
pub use self::interleaved_channel_mut::InterleavedChannelMut;

mod interleaved_channel;
pub use self::interleaved_channel::InterleavedChannel;

mod linear_channel_mut;
pub use self::linear_channel_mut::LinearChannelMut;

mod linear_channel;
pub use self::linear_channel::LinearChannel;

mod translate;
pub use self::translate::Translate;

mod sample;
pub use self::sample::Sample;

mod io;
pub use self::io::{ReadBuf, WriteBuf};
