Struct JITBuilder

pub struct JITBuilder { /* private fields */ }

A builder for JITModule.
Implementations
Source
impl JITBuilder
Source
pub fn new(
    libcall_names: Box<dyn Fn(LibCall) -> String + Send + Sync>,
) -> ModuleResult<Self>

Create a new JITBuilder.

The libcall_names function provides a way to translate cranelift_codegen’s ir::LibCall enum to symbols. LibCalls are inserted in the IR as part of the legalization for certain floating point instructions, and for stack probes. If you don’t know what to use for this argument, use cranelift_module::default_libcall_names().
Source
pub fn with_flags(
    flags: &[(&str, &str)],
    libcall_names: Box<dyn Fn(LibCall) -> String + Send + Sync>,
) -> ModuleResult<Self>

Create a new JITBuilder with the given flags.

The libcall_names function provides a way to translate cranelift_codegen’s ir::LibCall enum to symbols. LibCalls are inserted in the IR as part of the legalization for certain floating point instructions, and for stack probes. If you don’t know what to use for this argument, use cranelift_module::default_libcall_names().
Source
pub fn with_isa(
    isa: OwnedTargetIsa,
    libcall_names: Box<dyn Fn(LibCall) -> String + Send + Sync>,
) -> Self

Create a new JITBuilder with an arbitrary target. This is mainly useful for testing.

To create a JITBuilder for native use, use the new or with_flags constructors instead.

The libcall_names function provides a way to translate cranelift_codegen’s ir::LibCall enum to symbols. LibCalls are inserted in the IR as part of the legalization for certain floating point instructions, and for stack probes. If you don’t know what to use for this argument, use cranelift_module::default_libcall_names().
Source
pub fn symbol<K>(&mut self, name: K, ptr: *const u8) -> &mut Self
where
    K: Into<String>,

Define a symbol in the internal symbol table.

The JIT will use the symbol table to resolve names that are declared, but not defined, in the module being compiled. A common example is external functions. With this method, functions and data can be exposed to the code being compiled which are defined by the host.

If a symbol is defined more than once, the most recent definition will be retained.

If the JIT fails to find a symbol in its internal table, it will fall back to a platform-specific search (this typically involves searching the current process for public symbols, followed by searching the platform’s C runtime).
Source
pub fn symbols<It, K>(&mut self, symbols: It) -> &mut Self
where
    It: IntoIterator<Item = (K, *const u8)>,
    K: Into<String>,

Define multiple symbols in the internal symbol table.

Using this is equivalent to calling symbol on each element.
Source
pub fn symbol_lookup_fn(
    &mut self,
    symbol_lookup_fn: Box<dyn Fn(&str) -> Option<*const u8> + Send>,
) -> &mut Self

Add a symbol lookup fn.

Symbol lookup fn’s are used to lookup symbols when they couldn’t be found in the internal symbol table. Symbol lookup fn’s are called in reverse of the order in which they were added.
Source
pub fn memory_provider(
    &mut self,
    provider: Box<dyn JITMemoryProvider + Send>,
) -> &mut Self

Set the memory provider for the module.

If unset, defaults to SystemMemoryProvider.
Auto Trait Implementations
impl !RefUnwindSafe for JITBuilder
impl !Sync for JITBuilder
impl !UnwindSafe for JITBuilder
impl Freeze for JITBuilder
impl Send for JITBuilder
impl Unpin for JITBuilder
impl UnsafeUnpin for JITBuilder
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
§
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
