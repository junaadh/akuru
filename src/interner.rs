use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Symbol(pub u32);

impl Symbol {
    pub fn as_str(self) -> &'static str {
        SYMBOL_INTERNER.with(|cell| {
            let interner = cell.borrow();
            interner.resolve(self)
        })
    }
}

pub struct Interner {
    names: Vec<&'static str>,
    indices: HashMap<&'static str, Symbol>,
}

impl Interner {
    pub fn fresh() -> Self {
        Self {
            names: Vec::new(),
            indices: HashMap::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> Symbol {
        if let Some(sym) = self.indices.get(&name) {
            *sym
        } else {
            let leaked: &'static str = Box::leak(name.to_owned().into_boxed_str());

            let sym = Symbol(self.names.len() as u32);

            self.names.push(leaked);
            self.indices.insert(leaked, sym);

            sym
        }
    }

    pub fn resolve(&self, sym: Symbol) -> &'static str {
        self.names[sym.0 as usize]
    }
}

thread_local! {
    pub static SYMBOL_INTERNER: RefCell<Interner> = RefCell::new(Interner::fresh());
}

pub trait Internable {
    fn intern(&self) -> Symbol;
}

impl Internable for str {
    fn intern(&self) -> Symbol {
        SYMBOL_INTERNER.with(|cell| {
            let mut interner = cell.borrow_mut();
            interner.intern(self)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interning_same_string() {
        let s1 = "hello".intern();
        let s2 = "hello".intern();

        assert_eq!(
            s1, s2,
            "Interned symbols for the same string should be equal"
        );
        assert_eq!(s1.as_str(), "hello");
    }

    #[test]
    fn test_interning_different_strings() {
        let s1 = "hello".intern();
        let s2 = "world".intern();

        assert_ne!(s1, s2, "Different strings should yield different symbols");
        assert_eq!(s1.as_str(), "hello");
        assert_eq!(s2.as_str(), "world");
    }

    #[test]
    fn test_resolve_symbol_to_str() {
        let symbol = "greetings".intern();
        let resolved = symbol.as_str();

        assert_eq!(
            resolved, "greetings",
            "Resolved string should match original"
        );
    }

    #[test]
    fn test_symbols_are_unique() {
        let mut seen = std::collections::HashSet::new();

        for word in ["a", "b", "c", "d", "e", "f", "g", "h"] {
            let sym = word.intern();
            assert!(
                !seen.contains(&sym),
                "Symbol should be unique on first insert"
            );
            seen.insert(sym);
        }

        for word in ["a", "b", "c"] {
            let sym = word.intern();
            assert!(
                seen.contains(&sym),
                "Re-interned symbol should be in the set"
            );
        }
    }
}
