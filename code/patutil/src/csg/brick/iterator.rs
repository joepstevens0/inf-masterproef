use crate::csg::BrickRef;






pub struct BrickIterator {
    next: Option<BrickRef>,
}

impl BrickIterator{
    pub fn new(first_brick: Option<BrickRef>) -> Self {
        Self { next:first_brick }
    }
}

impl Iterator for BrickIterator {
    type Item = BrickRef;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.clone();
        if let Some(r) = &next{
            self.next = r.get().next_brick();
        } else {
            self.next = None;
        }
        next
    }
}