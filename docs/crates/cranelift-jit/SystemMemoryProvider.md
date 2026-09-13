Struct SystemMemoryProvider Copy item path
Search
Settings
Help
Summary
Source

pub struct SystemMemoryProvider { /* private fields */ }

A memory provider that allocates memory on-demand using the system allocator.

Note: Memory will be leaked by default unless JITMemoryProvider::free_memory is called to ensure function pointers remain valid for the remainder of the program’s life.
Implementations
Source
impl SystemMemoryProvider
Source
pub fn new() -> Self

Create a new memory handle with the given branch protection.
Trait Implementations
Source
impl JITMemoryProvider for SystemMemoryProvider
Source
unsafe fn free_memory(&mut self)
Free the memory region.
Source
fn finalize(&mut self, branch_protection: BranchProtection) -> ModuleResult<()>
Finalize the memory region and apply memory protections.
Source
fn allocate(
    &mut self,
    size: usize,
    align: u64,
    kind: JITMemoryKind,
) -> Result<*mut u8>
Allocate memory
Auto Trait Implementations
impl !Sync for SystemMemoryProvider
impl Freeze for SystemMemoryProvider
impl RefUnwindSafe for SystemMemoryProvider
impl Send for SystemMemoryProvider
impl Unpin for SystemMemoryProvider
impl UnsafeUnpin for SystemMemoryProvider
impl UnwindSafe for SystemMemoryProvider
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
