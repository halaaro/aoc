use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::BuildHasher,
};

pub trait GroupCollect<S = RandomState> {
    type TKey;
    type TValue;

    fn group_collect<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
    ) -> TO;
}

impl<K, V, T: Iterator<Item = (K, V)>> GroupCollect for T
where
    K: Eq + std::hash::Hash + Clone,
{
    type TKey = K;
    type TValue = V;

    fn group_collect<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
    ) -> TO {
        GroupCollectWithHasher::<RandomState>::group_collect_with_hasher(self, Default::default())
    }
}

pub trait GroupCollectWithHasher<S> {
    type TKey;
    type TValue;
    fn group_collect_with_hasher<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
        hasher: S,
    ) -> TO;
}

impl<K, V, T: Iterator<Item = (K, V)>, S> GroupCollectWithHasher<S> for T
where
    K: Eq + std::hash::Hash + Clone,
    S: Default + BuildHasher,
{
    type TKey = K;
    type TValue = V;

    fn group_collect_with_hasher<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
        hasher: S,
    ) -> TO {
        let mut map = HashMap::<K, usize, S>::with_hasher(hasher);
        let mut values: Vec<(K, TI)> = Vec::new();

        for (key, val) in self {
            let idx = *map.entry(key.clone()).or_insert(values.len());
            if idx == values.len() {
                values.push((key, Default::default()))
            };
            values[idx].1.extend([val]);
        }
        TO::from_iter(values)
    }
}

pub trait GroupCollectBy<F, S = RandomState> {
    type TKey;
    type TValue;

    fn group_collect_by<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
        f: F,
    ) -> TO;
}

impl<K, V, T, F> GroupCollectBy<F> for T
where
    K: Eq + std::hash::Hash + Clone,
    T: Iterator<Item = V>,
    F: Fn(&V) -> K,
{
    type TKey = K;
    type TValue = V;

    fn group_collect_by<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
        f: F,
    ) -> TO {
        GroupCollect::group_collect(self.map(|v| (f(&v), v)))
    }
}

pub trait GroupCollectByWithHasher<F, S> {
    type TKey;
    type TValue;

    fn group_collect_by_with_hasher<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
        f: F,
        hasher: S,
    ) -> TO;
}

impl<K, V, T, F, S> GroupCollectByWithHasher<F, S> for T
where
    K: Eq + std::hash::Hash + Clone,
    T: Iterator<Item = V>,
    F: Fn(&V) -> K,
    S: Default + BuildHasher,
{
    type TKey = K;
    type TValue = V;

    fn group_collect_by_with_hasher<
        TO: FromIterator<(Self::TKey, TI)>,
        TI: Default + Extend<Self::TValue>,
    >(
        self,
        f: F,
        hasher: S,
    ) -> TO {
        GroupCollectWithHasher::group_collect_with_hasher(self.map(|v| (f(&v), v)), hasher)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashSet};

    use super::*;

    #[test]
    fn test_group_collect_for_vec_of_vecs() {
        let grps: Vec<_> = [(1, 2), (1, 3), (2, 4)].into_iter().group_collect();
        assert_eq!(grps[0], (1, vec![2, 3]));
        assert_eq!(grps[1], (2, vec![4]));
    }

    #[test]
    fn test_group_collect_for_hashmap_of_hashsets() {
        let grps: HashMap<_, HashSet<_>> = [(1, 2), (1, 3), (2, 4)].into_iter().group_collect();
        assert_eq!(grps[&1], HashSet::from_iter([2, 3]));
        assert_eq!(grps[&2], HashSet::from_iter([4]));
    }

    #[test]
    fn test_group_collect_for_btreemap_of_btreesets() {
        let grps: BTreeMap<_, BTreeSet<_>> =
            [("alpha", "start"), ("beta", "middle"), ("alpha", "end")]
                .into_iter()
                .group_collect();

        assert_eq!(grps["alpha"], BTreeSet::from_iter(["start", "end"]));
        assert_eq!(grps["beta"], BTreeSet::from_iter(["middle"]));
    }

    #[test]
    fn test_group_collect_by_for_vec_of_vecs() {
        let grps: Vec<(_, Vec<_>)> = ["beets", "alpha", "alabama"]
            .into_iter()
            .group_collect_by(|s| s.chars().next());
        assert_eq!(grps[0], (Some('b'), vec!["beets"]));
        assert_eq!(grps[1], (Some('a'), vec!["alpha", "alabama"]));
    }

    #[test]
    fn test_group_collect_by_for_hashmap_of_vecs() {
        let grps: HashMap<_, Vec<_>> = [0, 1, 2, 3].into_iter().group_collect_by(|i| i % 3);

        assert_eq!(grps[&0], vec![0, 3]);
        assert_eq!(grps[&1], vec![1]);
        assert_eq!(grps[&2], vec![2]);
    }

    #[test]
    fn test_group_collect_with_hasher() {
        let grps: HashMap<_, Vec<_>> = [(1, 2), (3, 4), (1, 5)]
            .into_iter()
            .group_collect_with_hasher(RandomState::new());

        assert_eq!(grps[&1], vec![2, 5]);
        assert_eq!(grps[&3], vec![4]);
    }

    #[test]
    fn test_group_collect_by_with_hasher() {
        let grps: HashMap<_, Vec<_>> = [0, 1, 2, 3]
            .into_iter()
            .group_collect_by_with_hasher(|i| i % 3, RandomState::new());

        assert_eq!(grps[&0], vec![0, 3]);
        assert_eq!(grps[&1], vec![1]);
        assert_eq!(grps[&2], vec![2]);
    }
}
