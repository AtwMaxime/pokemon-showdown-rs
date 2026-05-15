//! DexTypes - Types lookup helper
//!
//! Equivalent to DexTypes class in dex-data.ts

use crate::dex::{Dex, TypeData};

/// Helper struct for type lookups
/// JavaScript equivalent: DexTypes (sim/dex-data.ts)
/// 1 field in JavaScript (dex)
pub struct DexTypes<'a> {
    /// Dex reference
    /// JavaScript: readonly dex: ModdedDex
    pub(crate) dex: &'a Dex,
}

impl<'a> DexTypes<'a> {
    /// Get type data by name
    /// Equivalent to DexTypes.get() in dex-data.ts
    pub fn get(&self, name: &str) -> Option<&'a TypeData> {
        // Types use capitalized names as keys
        self.dex.types.get(name)
    }

    /// Get all type names
    /// Equivalent to DexTypes.names() in dex-data.ts
    /// IMPORTANT: Returns types in alphabetical order to match JavaScript's TypeChart
    /// iteration order.
    pub fn names(&self) -> Vec<&'a String> {
        // JavaScript's names() method does:
        // this.all().filter(type => !type.isNonstandard).map(type => type.name)
        // where all() iterates over TypeChart in property insertion order (alphabetical)
        // In Gen 9, Stellar is a standard type and should be included.
        let mut names: Vec<&'a String> = self.dex.types.keys().collect();
        // Sort alphabetically to match JavaScript's TypeChart property order
        names.sort();
        names
    }
}
