Trait JITMemoryProvider Copy item path
Search
Settings
Help
Summary
Source

pub trait JITMemoryProvider {
    // Required methods
    fn allocate(
        &mut self,
        size: usize,
        align: u64,
        kind: JITMemoryKind,
    ) -> Result<*mut u8>;
    unsafe fn 
free_memory(&mut self);
    fn 
finalize(
        &mut self,
        branch_protection: BranchProtection,
    ) -> ModuleResult<()>;
}

A provider of memory for the JIT.
Required Methods
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
Dyn Compatibility

This trait is dyn compatible.

In older versions of Rust, dyn compatibility was called "object safety".
Implementors
Source
impl JITMemoryProvider for ArenaMemoryProvider
Source
impl JITMemoryProvider for SystemMemoryProvider
