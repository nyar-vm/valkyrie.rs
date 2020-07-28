use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use crate::{types::ValkyrieMetaType, ValkyrieDataTable, ValkyrieType, ValkyrieValue};

impl<T> ValkyrieType for Vec<T>
where
    T: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        let mut out = ValkyrieDataTable::list();
        for item in self {
            out.extend_one(item.boxed());
        }
        ValkyrieValue::DataTable(Arc::new(out))
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.collection.Array");
        this.mut_generic_types().push(T::static_type());
        Arc::new(this)
    }
}

impl<T> ValkyrieType for BTreeSet<T>
where
    T: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        let mut out = ValkyrieDataTable::list();
        for item in self {
            out.extend_one(item.boxed());
        }
        ValkyrieValue::DataTable(Arc::new(out))
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.collection.SortedSet");
        this.mut_generic_types().push(T::static_type());
        Arc::new(this)
    }
}

impl<K, V> ValkyrieType for BTreeMap<K, V>
where
    K: ValkyrieType,
    V: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        let mut out = ValkyrieDataTable::list();
        for item in self {
            out.extend_one(item.boxed());
        }
        ValkyrieValue::DataTable(Arc::new(out))
    }

    fn dynamic_type(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.collection.SortedMap");
        this.mut_generic_types().push(K::static_type());
        this.mut_generic_types().push(V::static_type());
        Arc::new(this)
    }
}
