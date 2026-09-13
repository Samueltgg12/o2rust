Enum JITMemoryKind Copy item path
Search
Settings
Help
Summary
Source

pub enum JITMemoryKind {
    Executable,
    Writable,
    ReadOnly,
}

The kind of memory allocation requested by a JITMemoryProvider.
Variants
Executable

Allocate memory that will be executable once finalized.
Writable

Allocate writable memory.
ReadOnly

Allocate memory that will be read-only once finalized.
Auto Trait Implementations
impl Freeze for JITMemoryKind
impl RefUnwindSafe for JITMemoryKind
impl Send for JITMemoryKind
impl Sync for JITMemoryKind
impl Unpin for JITMemoryKind
impl UnsafeUnpin for JITMemoryKind
impl UnwindSafe for JITMemoryKind
Blanket Implementations
Source
impl<T> Any for T
where
    T: 'static + ?Sized,
Source
impl<T> Borrow<T> for T
where
    T: ?Sized,
Source
impl<T> BorrowMut<T> for T
where
    T: ?Sized,
Source
impl<T> From<T> for T

Source
impl<T, U> Into<U> for T
where
    U: From<T>,

Source
impl<T, U> TryFrom<U> for T
where
    U: Into<T>,
Source
impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
