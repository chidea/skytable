/*
 * Created on Sat Jan 28 2023
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2023, Sayan Nandan <ohsayan@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

use super::{
    super::{super::sync::atm::Guard, AsKey, DummyMetrics, IndexBaseSpec, MTIndex},
    iter::{IterKV, IterKey, IterVal},
    meta::{AsHasher, Config, Key, Value},
    Tree,
};
use std::{borrow::Borrow, sync::Arc};

#[inline(always)]
fn arc<K, V>(k: K, v: V) -> Arc<(K, V)> {
    Arc::new((k, v))
}

pub type MTArc<K, V, S, C> = Tree<Arc<(K, V)>, S, C>;

impl<K, V, S, C> IndexBaseSpec<K, V> for MTArc<K, V, S, C>
where
    C: Config,
    S: AsHasher,
{
    const PREALLOC: bool = false;

    type Metrics = DummyMetrics;

    fn idx_init() -> Self {
        MTArc::new()
    }

    fn idx_init_with(s: Self) -> Self {
        s
    }

    fn idx_metrics(&self) -> &Self::Metrics {
        &DummyMetrics
    }
}

impl<K, V, S, C> MTIndex<K, V> for MTArc<K, V, S, C>
where
    C: Config,
    S: AsHasher,
    K: Key,
    V: Value,
{
    type IterKV<'t, 'g, 'v> = IterKV<'t, 'g, 'v, (K, V), S, C>
    where
        'g: 't + 'v,
        't: 'v,
        K: 'v,
        V: 'v,
        Self: 't;

    type IterKey<'t, 'g, 'v> = IterKey<'t, 'g, 'v, (K, V), S, C>
    where
        'g: 't + 'v,
        't: 'v,
        K: 'v,
        Self: 't;

    type IterVal<'t, 'g, 'v> = IterVal<'t, 'g, 'v, (K, V), S, C>
    where
        'g: 't + 'v,
        't: 'v,
        V: 'v,
        Self: 't;

    fn mt_clear(&self, g: &Guard) {
        self.nontransactional_clear(g)
    }

    fn mt_insert(&self, key: K, val: V, g: &Guard) -> bool {
        self.insert(arc(key, val), g)
    }

    fn mt_upsert(&self, key: K, val: V, g: &Guard) {
        self.upsert(arc(key, val), g)
    }

    fn mt_contains<Q>(&self, key: &Q, g: &Guard) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + AsKey,
    {
        self.contains_key(key, g)
    }

    fn mt_get<'t, 'g, 'v, Q>(&'t self, key: &Q, g: &'g Guard) -> Option<&'v V>
    where
        K: Borrow<Q>,
        Q: ?Sized + AsKey,
        't: 'v,
        'g: 't + 'v,
    {
        self.get(key, g)
    }

    fn mt_get_cloned<Q>(&self, key: &Q, g: &Guard) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + AsKey,
    {
        self.get(key, g).cloned()
    }

    fn mt_update<Q>(&self, key: K, val: V, g: &Guard) -> bool
    where
        Q: ?Sized + AsKey,
    {
        self.update(arc(key, val), g)
    }

    fn mt_update_return<'t, 'g, 'v, Q>(&'t self, key: K, val: V, g: &'g Guard) -> Option<&'v V>
    where
        Q: ?Sized + AsKey,
        't: 'v,
        'g: 't + 'v,
    {
        self.update_return(arc(key, val), g)
    }

    fn mt_delete<Q>(&self, key: &Q, g: &Guard) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + AsKey,
    {
        self.remove(key, g)
    }

    fn mt_delete_return<'t, 'g, 'v, Q>(&'t self, key: &Q, g: &'g Guard) -> Option<&'v V>
    where
        K: Borrow<Q>,
        Q: ?Sized + AsKey,
        't: 'v,
        'g: 't + 'v,
    {
        self.remove_return(key, g)
    }
}
