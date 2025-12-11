use java_oxide::VM as JVM;

#[doc(hidden)]
/// Container for the Global JVM
pub(crate) struct VM {
    inner: std::sync::OnceLock<JVM>,
}
impl VM {
    /// Creates a new uninitialized VM container
    pub(crate) const fn new() -> Self {
        VM {
            inner: std::sync::OnceLock::new(),
        }
    }

    /// Initializes the VM to `vm`
    ///
    /// # Panics
    ///
    /// Panics if the VM was previously initialized
    pub(crate) fn init(&self, vm: JVM) {
        if let Err(_) = self.inner.set(vm) {
            println!("The VM has already been initialized");
        }
    }

    /// Gets a reference to the initialized VM
    ///
    /// # Panics
    ///
    /// Panics if the VM was __NOT__ previously initialized
    pub(crate) fn get(&self) -> &JVM {
        self.inner.get().expect("Attempted to get a reference to an uninitialized VM. Panicking because of undefined behavior")
    }
}
