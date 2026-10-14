// Copyright 2026 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

//! The tonic codec of rust-protobuf messages, used by the generated tonic
//! clients and servers.

use std::marker::PhantomData;

use bytes::{Buf, BufMut};
use protobuf::{CodedOutputStream, Message, ProtobufError};
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::Status;

/// Encodes `E` and decodes `D` with rust-protobuf.
#[derive(Debug)]
pub struct ProtobufCodec<E, D>(PhantomData<(E, D)>);

impl<E, D> Default for ProtobufCodec<E, D> {
    fn default() -> Self {
        ProtobufCodec(PhantomData)
    }
}

impl<E, D> Codec for ProtobufCodec<E, D>
where
    E: Message + Send + 'static,
    D: Message + Send + 'static,
{
    type Encode = E;
    type Decode = D;
    type Encoder = ProtobufEncoder<E>;
    type Decoder = ProtobufDecoder<D>;

    fn encoder(&mut self) -> Self::Encoder {
        ProtobufEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        ProtobufDecoder(PhantomData)
    }
}

#[derive(Debug)]
pub struct ProtobufEncoder<E>(PhantomData<E>);

impl<E: Message> Encoder for ProtobufEncoder<E> {
    type Item = E;
    type Error = Status;

    fn encode(&mut self, item: E, dst: &mut EncodeBuf<'_>) -> Result<(), Status> {
        // `compute_size` also caches the sizes of nested messages, which
        // `write_to_with_cached_sizes` relies on.
        let size = item.compute_size() as usize;
        dst.reserve(size);
        let spare = dst.chunk_mut();
        assert!(spare.len() >= size);
        // SAFETY: `spare` has at least `size` bytes. `CodedOutputStream` only
        // writes to the slice, and `check_eof` ensures all `size` bytes are
        // written before `advance_mut` exposes them.
        unsafe {
            let buf = std::slice::from_raw_parts_mut(spare.as_mut_ptr(), size);
            let mut os = CodedOutputStream::bytes(buf);
            item.write_to_with_cached_sizes(&mut os)
                .map_err(|e| to_status("encode", e))?;
            os.check_eof();
            dst.advance_mut(size);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ProtobufDecoder<D>(PhantomData<D>);

impl<D: Message> Decoder for ProtobufDecoder<D> {
    type Item = D;
    type Error = Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<D>, Status> {
        // `DecodeBuf` is backed by a `BytesMut`, whose `copy_to_bytes` doesn't
        // copy.
        let buf = src.copy_to_bytes(src.remaining());
        let msg = protobuf::parse_from_bytes(&buf).map_err(|e| to_status("decode", e))?;
        Ok(Some(msg))
    }
}

fn to_status(op: &str, e: ProtobufError) -> Status {
    Status::internal(format!("failed to {} protobuf message: {}", op, e))
}
