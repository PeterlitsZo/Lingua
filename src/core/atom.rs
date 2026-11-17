use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct AtomId(u64);

#[derive(Default)]
struct Interner {
    atoms: HashMap<Arc<str>, AtomId>,
    next_id: u64,
}

static INTERNER: OnceLock<Mutex<Interner>> = OnceLock::new();

#[derive(Clone, Debug)]
pub(crate) struct Atom {
    id: AtomId,
    name: Arc<str>,
}

impl Atom {
    pub(super) fn intern(name: &str) -> Result<Self, &'static str> {
        let mut interner = INTERNER
            .get_or_init(Mutex::default)
            .lock()
            .map_err(|_| "Atom interner lock poisoned")?;
        if let Some((name, &id)) = interner.atoms.get_key_value(name) {
            return Ok(Self {
                id,
                name: Arc::clone(name),
            });
        }
        let id = AtomId(interner.next_id);
        let next_id = interner.next_id.checked_add(1).ok_or("Too many atoms")?;
        let name: Arc<str> = Arc::from(name);
        interner.atoms.insert(Arc::clone(&name), id);
        interner.next_id = next_id;
        Ok(Self { id, name })
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Atom {}

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
