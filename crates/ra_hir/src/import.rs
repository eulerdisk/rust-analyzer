//! todo

use std::sync::Arc;
use ra_syntax::SmolStr;

use crate::{
    Crate, Path, Name, PersistentHirDatabase,
    PathKind, path::PathSegment
};

#[derive(Debug, Clone)]
pub struct ImportResolver {
    krate: Crate,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ImportItemMap {
    map: Vec<(SmolStr, Path)>,
}

impl ImportResolver {
    pub(crate) fn new(krate: Crate) -> Self {
        ImportResolver { krate }
    }

    pub fn resolve_name(
        &self,
        db: &impl PersistentHirDatabase,
        _name: &Name,
    ) -> Vec<(SmolStr, Path)> {
        let import_map = db.import_item_map(self.krate);
        import_map.map.clone()
    }
}

impl ImportItemMap {
    pub(crate) fn import_item_map_query(
        _db: &impl PersistentHirDatabase,
        _krate: Crate,
    ) -> Arc<ImportItemMap> {
        let mut import_map = ImportItemMap::default();

        let dummy_segments = vec![
            PathSegment { name: Name::new(SmolStr::new("std")), args_and_bindings: None },
            PathSegment { name: Name::new(SmolStr::new("fmt")), args_and_bindings: None },
            PathSegment { name: Name::new(SmolStr::new("Debug")), args_and_bindings: None },
        ];

        let dummy_path = Path { kind: PathKind::Plain, segments: dummy_segments };

        import_map.map.push((SmolStr::new("DummyDebug"), dummy_path));

        let dummy_segments = vec![
            PathSegment { name: Name::new(SmolStr::new("std")), args_and_bindings: None },
            PathSegment { name: Name::new(SmolStr::new("fmt")), args_and_bindings: None },
            PathSegment { name: Name::new(SmolStr::new("Display")), args_and_bindings: None },
        ];

        let dummy_path = Path { kind: PathKind::Plain, segments: dummy_segments };

        import_map.map.push((SmolStr::new("DummyDisplay"), dummy_path));

        Arc::new(import_map)
    }
}
