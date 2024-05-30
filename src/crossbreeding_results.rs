use crate::traits::PlantImpl;

pub struct CrossbreedingResults<T> {
    values: Vec<MaybeMany<T>>,
}

pub enum MaybeMany<T> {
    Element(T),
    EndOfMany,
}

impl<T: PlantImpl> CrossbreedingResults<T> {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn push_one(&mut self, elem: T) {
        self.values.push(MaybeMany::Element(elem));
    }
    pub fn push_many(&mut self, elems: impl Iterator<Item = T>) {
        for mut elem in elems {
            elem.set_one_of_many(true);
            self.values.push(MaybeMany::Element(elem));
        }
        self.values.push(MaybeMany::EndOfMany);
    }
}

impl<T> Iterator for CrossbreedingResults<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
