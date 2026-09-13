Struct JITModule Copy item path
Search
Settings
Help
Summary
Source

pub struct JITModule { /* private fields */ }

A JITModule implements Module and emits code and data into memory where it can be directly called and accessed.

See the JITBuilder for a convenient way to construct JITModule instances.
Implementations
Source
impl JITModule
Source
pub unsafe fn free_memory(self)

Free memory allocated for code and data segments of compiled functions.
Safety

Because this function invalidates any pointers retrieved from the corresponding module, it should only be used when none of the functions from that module are currently executing and none of the fn pointers are called afterwards.
Source
pub fn get_address(&self, name: &ModuleRelocTarget) -> *const u8

Get the address that the given ModuleRelocTarget would resolve to.

For locally defined functions and data objects this is equivalent to Self::get_finalized_function cq Self::get_finalized_data, while for external functions and data objects this will iterate through the registered symbol lookup functions until one matches.
Source
pub fn get_finalized_function(&self, func_id: FuncId) -> *const u8

Returns the address of a finalized function.

The pointer remains valid until either JITModule::free_memory is called or in the future some way of deallocating this individual function is used.
Source
pub fn get_finalized_data(&self, data_id: DataId) -> (*const u8, usize)

Returns the address and size of a finalized data object.

The pointer remains valid until either JITModule::free_memory is called or in the future some way of deallocating this individual data object is used.
Source
pub fn finalize_definitions(&mut self) -> ModuleResult<()>

Finalize all functions and data objects that are defined but not yet finalized. All symbols referenced in their bodies that are declared as needing a definition must be defined by this point.

Use get_finalized_function and get_finalized_data to obtain the final artifacts.

Returns ModuleError in case of allocation or syscall failure
Source
pub fn new(builder: JITBuilder) -> Self

Create a new JITModule.
Trait Implementations
Source
impl Module for JITModule
Source
fn isa(&self) -> &dyn TargetIsa
Return the TargetIsa to compile for.
Source
fn declarations(&self) -> &ModuleDeclarations
Get all declarations in this module.
Source
fn declare_function(
    &mut self,
    name: &str,
    linkage: Linkage,
    signature: &Signature,
) -> ModuleResult<FuncId>
Declare a function in this module.
Source
fn declare_anonymous_function(
    &mut self,
    signature: &Signature,
) -> ModuleResult<FuncId>
Declare an anonymous function in this module.
Source
fn declare_data(
    &mut self,
    name: &str,
    linkage: Linkage,
    writable: bool,
    tls: bool,
) -> ModuleResult<DataId>
Declare a data object in this module.
Source
fn declare_anonymous_data(
    &mut self,
    writable: bool,
    tls: bool,
) -> ModuleResult<DataId>
Declare an anonymous data object in this module.
Source
fn define_function_with_control_plane(
    &mut self,
    id: FuncId,
    ctx: &mut Context,
    ctrl_plane: &mut ControlPlane,
) -> ModuleResult<()>
Define a function, producing the function body from the given Context. Read more
Source
fn define_function_bytes(
    &mut self,
    id: FuncId,
    alignment: u64,
    bytes: &[u8],
    relocs: &[ModuleReloc],
) -> ModuleResult<()>
Define a function, taking the function body from the given bytes. Read more
Source
fn define_data(
    &mut self,
    id: DataId,
    data: &DataDescription,
) -> ModuleResult<()>
Define a data object, producing the data contents from the given DataDescription.
Source
fn get_name(&self, name: &str) -> Option<FuncOrDataId>
Get the module identifier for a given name, if that name has been declared.
Source
fn target_config(&self) -> TargetFrontendConfig
Return the target information needed by frontends to produce Cranelift IR for the current target.
Source
fn make_context(&self) -> Context
Create a new Context initialized for use with this Module. Read more
Source
fn clear_context(&self, ctx: &mut Context)
Clear the given Context and reset it for use with a new function. Read more
Source
fn make_signature(&self) -> Signature
Create a new empty Signature with the default calling convention for the TargetIsa, to which parameter and return types can be added for declaring a function to be called by this Module.
Source
fn clear_signature(&self, sig: &mut Signature)
Clear the given Signature and reset for use with a new function. Read more
Source
fn declare_func_in_func(
    &mut self,
    func_id: FuncId,
    func: &mut Function,
) -> FuncRef
Use this when you’re building the IR of a function to reference a function. Read more
Source
fn declare_data_in_func(&self, data: DataId, func: &mut Function) -> GlobalValue
Use this when you’re building the IR of a function to reference a data object. Read more
Source
fn declare_func_in_data(
    &self,
    func_id: FuncId,
    data: &mut DataDescription,
) -> FuncRef
TODO: Same as above.
Source
fn declare_data_in_data(
    &self,
    data_id: DataId,
    data: &mut DataDescription,
) -> GlobalValue
TODO: Same as above.
Source
fn define_function(
    &mut self,
    func: FuncId,
    ctx: &mut Context,
) -> Result<(), ModuleError>
Define a function, producing the function body from the given Context. Read more
Auto Trait Implementations
impl !Freeze for JITModule
impl !RefUnwindSafe for JITModule
impl !Sync for JITModule
impl !UnwindSafe for JITModule
impl Send for JITModule
impl Unpin for JITModule
impl UnsafeUnpin for JITModule
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
