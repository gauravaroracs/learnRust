struct LeafPage {
    keys: Vec<i32>,
    rids: Vec<i32>,
}

impl LeafPage {
    fn find(&self, key: i32) -> Option<i32> {
        for (i, val) in &self.keys.iter().enumerate() {
            if val == key {
                return Some((i));
            } else {
                return None;
            }
        }
    }

    fn insert_sorted(&mut self, key: i32, rid: i32) {}
}
