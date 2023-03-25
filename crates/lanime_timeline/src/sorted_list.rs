use slotmap::{Key, SlotMap};

pub struct SortedList<K: Key, T> {
    items: SlotMap<K, T>,
    order: Vec<K>,
}

impl<K: Key, T> SortedList<K, T> {
    pub fn new() -> Self {
        Self {
            items: SlotMap::with_key(),
            order: Vec::new(),
        }
    }

    pub fn insert_top(&mut self, value: T) -> K {
        let key = self.items.insert(value);
        self.order.push(key);
        key
    }

    pub fn insert_at(&mut self, value: T, index: usize) -> K {
        let key = self.items.insert(value);
        self.order.insert(index, key);
        key
    }

    #[inline]
    pub fn get(&self, key: K) -> Option<&T> {
        self.items.get(key)
    }

    #[inline]
    pub fn get_mut(&mut self, key: K) -> Option<&mut T> {
        self.items.get_mut(key)
    }

    pub fn remove(&mut self, key: K) -> Option<T> {
        let index = self.order.binary_search(&key).ok()?;
        self.order.remove(index);
        self.items.remove(key)
    }

    pub fn move_up(&mut self, key: K) {
        let index = self.order.binary_search(&key).unwrap();
        if index == self.order.len() - 1 {
            return;
        }
        self.order.swap(index + 1, index);
    }

    pub fn move_down(&mut self, key: K) {
        let index = self.order.binary_search(&key).unwrap();
        if index == 0 {
            return;
        }
        self.order.swap(index - 1, index);
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, &T)> {
        self.order.iter().map(|&key| (key, &self.items[key]))
    }
}

#[cfg(test)]
mod test {
    use slotmap::DefaultKey;

    use crate::sorted_list::SortedList;

    #[test]
    fn test_insert_top() {
        let mut list = SortedList::<DefaultKey, &str>::new();

        let key1 = list.insert_top("A");
        let key2 = list.insert_top("B");
        let key3 = list.insert_top("C");

        assert_eq!(list.get(key1), Some(&"A"));
        assert_eq!(list.get(key2), Some(&"B"));
        assert_eq!(list.get(key3), Some(&"C"));
    }

    #[test]
    fn test_insert_at() {
        let mut list = SortedList::<DefaultKey, &str>::new();

        let key1 = list.insert_top("A");
        let key2 = list.insert_top("B");
        let key3 = list.insert_top("C");

        let key4 = list.insert_at("X", 1);

        assert_eq!(list.get(key1), Some(&"A"));
        assert_eq!(list.get(key2), Some(&"B"));
        assert_eq!(list.get(key3), Some(&"C"));
        assert_eq!(list.get(key4), Some(&"X"));

        assert_eq!(
            list.iter().collect::<Vec<_>>(),
            vec![(key1, &"A"), (key4, &"X"), (key2, &"B"), (key3, &"C")]
        );
    }

    #[test]
    fn test_remove() {
        let mut list = SortedList::<DefaultKey, &str>::new();

        let key1 = list.insert_top("A");
        let key2 = list.insert_top("B");
        let key3 = list.insert_top("C");

        let removed = list.remove(key2);
        assert_eq!(removed, Some("B"));

        assert_eq!(list.get(key1), Some(&"A"));
        assert_eq!(list.get(key2), None);
        assert_eq!(list.get(key3), Some(&"C"));
    }

    #[test]
    fn test_iter_order() {
        let mut list = SortedList::<DefaultKey, &str>::new();
        let key1 = list.insert_top("A");
        let key2 = list.insert_top("B");
        let key3 = list.insert_top("C");

        assert_eq!(
            list.iter().collect::<Vec<_>>(),
            vec![(key1, &"A"), (key2, &"B"), (key3, &"C")]
        );
    }

    #[test]
    fn test_move_up() {
        let mut list = SortedList::<DefaultKey, &str>::new();

        let key1 = list.insert_top("A");
        let key2 = list.insert_top("B");
        let key3 = list.insert_top("C");

        list.move_up(key1);

        assert_eq!(
            list.iter().collect::<Vec<_>>(),
            vec![(key2, &"B"), (key1, &"A"), (key3, &"C")]
        );
    }

    #[test]
    fn test_move_down() {
        let mut list = SortedList::<DefaultKey, &str>::new();

        let key1 = list.insert_top("A");
        let key2 = list.insert_top("B");
        let key3 = list.insert_top("C");

        list.move_down(key3);

        assert_eq!(
            list.iter().collect::<Vec<_>>(),
            vec![(key1, &"A"), (key3, &"C"), (key2, &"B")]
        );
    }
}
