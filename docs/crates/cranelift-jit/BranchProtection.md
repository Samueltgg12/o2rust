Enum BranchProtection Copy item path
Search
Settings
Help
Summary
Source

pub enum BranchProtection {
    None,
    BTI,
}

Type of branch protection to apply to executable memory.
Variants
None

No protection.
BTI

Use the Branch Target Identification extension of the Arm architecture.
Trait Implementations
Source
impl Clone for BranchProtection
Source
fn clone(&self) -> BranchProtection
Returns a duplicate of the value. Read more
1.0.0 (const: unstable) · Source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
Source
impl Copy for BranchProtection
Source
impl Debug for BranchProtection
Source
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read more
Source
impl PartialEq for BranchProtection
Source
fn eq(&self, other: &BranchProtection) -> bool
Equality operator ==. Read more
1.0.0 (const: unstable) · Source
fn ne(&self, other: &Rhs) -> bool
Inequality operator !=. Read more
Source
impl StructuralPartialEq for BranchProtection
Auto Trait Implementations
impl Freeze for BranchProtection
impl RefUnwindSafe for BranchProtection
impl Send for BranchProtection
impl Sync for BranchProtection
impl Unpin for BranchProtection
impl UnsafeUnpin for BranchProtection
impl UnwindSafe for BranchProtection
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
impl<T> CloneToUninit for T
where
    T: Clone,
Source
impl<T> From<T> for T

Source
impl<T, U> Into<U> for T
where
    U: From<T>,

Source
impl<T> ToOwned for T
where
    T: Clone,
Source
impl<T, U> TryFrom<U> for T
where
    U: Into<T>,
Source
impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
