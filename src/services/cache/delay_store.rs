///
/// Counts number of next call, 
/// - next returns None if presset count exceeded, then repeat
pub struct DelydStore {
    count: isize,
    val: isize,
    stored: bool,
}
///
/// 
impl DelydStore {
    pub fn new(count: isize) -> Self {
        Self { count, val: count, stored: true }
    }
    ///
    /// 
    pub fn next(&mut self) -> Option<()> {
        self.val -= 1;
        self.stored = false;
        if self.val <= 0 {
            self.val = self.count;
            None
        } else {
            Some(())
        }
    }
    ///
    /// 
    pub fn stored(&self) -> bool {
        self.stored
    }
    ///
    /// 
    pub fn set_stored(&mut self) {
        self.stored = true;
    }
}