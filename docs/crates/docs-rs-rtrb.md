A realtime-safe single-producer single-consumer (SPSC) ring buffer.

A RingBuffer consists of two parts: a Producer for writing into the ring buffer and a Consumer for reading from the ring buffer.

A fixed-capacity buffer is allocated on construction. After that, no more memory is allocated (unless the type T does that internally). Reading from and writing into the ring buffer is lock-free and wait-free. All reading and writing functions return immediately. Attempts to write to a full buffer return an error; values inside the buffer are not overwritten. Attempts to read from an empty buffer return an error as well. Only a single thread can write into the ring buffer and a single thread (typically a different one) can read from the ring buffer. If the queue is empty, there is no way for the reading thread to wait for new data, other than trying repeatedly until reading succeeds. Similarly, if the queue is full, there is no way for the writing thread to wait for newly available space to write to, other than trying repeatedly.
Examples

Moving single elements into and out of a queue with Producer::push() and Consumer::pop(), respectively:

use rtrb::{RingBuffer, PushError, PopError};

let (mut producer, mut consumer) = RingBuffer::new(2);

assert_eq!(producer.push(10), Ok(()));
assert_eq!(producer.push(20), Ok(()));
assert_eq!(producer.push(30), Err(PushError::Full(30)));

std::thread::spawn(move || {
    assert_eq!(consumer.pop(), Ok(10));
    assert_eq!(consumer.pop(), Ok(20));
    assert_eq!(consumer.pop(), Err(PopError::Empty));
}).join().unwrap();

See the documentation of the chunks module for examples that write/read multiple items at once. See also:

    Producer::write_chunk()

    Producer::write_chunk_uninit()

    Producer::push_partial_slice() (if T: Copy)

    Producer::push_entire_slice() (if T: Copy)

    Consumer::read_chunk()

    Consumer::pop_partial_slice() (if T: Copy)

    Consumer::pop_partial_slice_uninit() (if T: Copy)

    Consumer::pop_entire_slice() (if T: Copy)

    Consumer::pop_entire_slice_uninit() (if T: Copy)

Modules

chunks
    Writing and reading multiple items at once into and from a RingBuffer.

Structs

Consumer
    The consumer side of a RingBuffer.
Producer
    The producer side of a RingBuffer.
RingBuffer
    A bounded single-producer single-consumer (SPSC) queue.

Enums

PeekError
    Error type for Consumer::peek().
PopError
    Error type for Consumer::pop().
PushError
    Error type for Producer::push().

Traits

CopyToUninit
    Extension trait used to provide a copy_to_uninit() method on built-in slices.