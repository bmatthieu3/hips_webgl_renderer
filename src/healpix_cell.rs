use std::cmp::Ordering;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct HEALPixCell(pub u8, pub u64);

impl HEALPixCell {
    // Build the parent cell
    pub fn parent(self) -> HEALPixCell {
        if self.depth() == 0 {
            // If cell belongs to a root cell
            // we return it as a root cell do not have any parent
            self
        } else {
            HEALPixCell(self.0 - 1, self.1 >> 2)
        }
    }

    pub fn idx(&self) -> u64 {
        self.1
    }

    pub fn depth(&self) -> u8 {
        self.0
    }
}

impl PartialOrd for HEALPixCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let n1 = self.1 << ((29 - self.0) << 1);
        let n2 = other.1 << ((29 - other.0) << 1);

        n1.partial_cmp(&n2)
    }
}
impl Ord for HEALPixCell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}