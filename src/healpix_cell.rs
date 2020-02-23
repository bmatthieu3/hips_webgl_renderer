use std::cmp::Ordering;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct HEALPixCell(pub u8, pub u64);

impl PartialOrd for HEALPixCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        /*let uniq = (1 << (2*((self.0 as u64) + 1))) + self.1;
        let uniq_other = (1 << (2*((other.0 as u64) + 1))) + other.1;

        uniq.partial_cmp(&uniq_other)*/

        // Compare the two nested cells at MAX_DEPTH in the nested scheme
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