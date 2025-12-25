/// The Seal: A wrapper that proves a Fact has been vetted.
/// You cannot construct this directly. You must go through `verify`.
pub struct Sealed<T>(T);

impl<T> Sealed<T> {
    /// The Lens Correction.
    /// This logic replaces "Argument" with "Verification".
    /// If the `predicate` fails, the fact is rejected.
    /// Creates a new Sealed fact if it passes the predicate.
    ///
    /// # Errors
    /// Returns an error if the predicate fails (returns Err).
    pub fn mint<F>(fact: T, predicate: F) -> Result<Self, String>
    where
        F: FnOnce(&T) -> Result<(), String>,
    {
        match predicate(&fact) {
            Ok(()) => Ok(Sealed(fact)),
            Err(e) => Err(format!("REGULATOR REJECTION: {e}")),
        }
    }

    /// The Witness Accessor.
    /// Allows the system to unwrap the fact after it passes the gate.
    pub fn witness(self) -> T {
        self.0
    }
}
