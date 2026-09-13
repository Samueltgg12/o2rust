Struct ArenaMemoryProvider

pub struct ArenaMemoryProvider { /* private fields */ }

ArenaMemoryProvider allocates segments from a contiguous memory region that is reserved up-front.

The arena’s memory is initially allocated with PROT_NONE and gradually updated as the JIT requires more space. This approach allows for stable addresses throughout the lifetime of the JIT.

Depending on the underlying platform, requesting large parts of the address space to be allocated might fail. This implementation currently doesn’t do overcommit on Windows.

Note: Memory will be leaked by default unless JITMemoryProvider::free_memory is called to ensure function pointers remain valid for the remainder of the program’s life.
Implementations
Source
impl ArenaMemoryProvider
Source
pub fn new_with_size(reserve_size: usize) -> Result<Self, Error>

Create a new memory region with the given size.
Trait Implementations
Source
impl Drop for ArenaMemoryProvider
Source
fn drop(&mut self)
Executes the destructor for this type. Read more
Source
fn pin_drop(self: Pin<&mut Self>)
🔬This is a nightly-only experimental API. (pin_ergonomics)
Execute the destructor for this type, but different to Drop::drop, it requires self to be pinned. Read more
Source
impl JITMemoryProvider for ArenaMemoryProvider
Source
fn allocate(
    &mut self,
    size: usize,
    align: u64,
    kind: JITMemoryKind,
) -> Result<*mut u8>
Allocate memory
Source
unsafe fn free_memory(&mut self)
Free the memory region.
Source
fn finalize(&mut self, branch_protection: BranchProtection) -> ModuleResult<()>
Finalize the memory region and apply memory protections.
Source
impl Send for ArenaMemoryProvider
Auto Trait Implementations
impl !Sync for ArenaMemoryProvider
impl Freeze for ArenaMemoryProvider
impl RefUnwindSafe for ArenaMemoryProvider
impl Unpin for ArenaMemoryProvider
impl UnsafeUnpin for ArenaMemoryProvider
impl UnwindSafe for ArenaMemoryProvider
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
