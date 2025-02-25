use crate::memdata::CoreData;

pub struct Server {
    data: CoreData,
}

impl Server {
    pub fn new() -> Self {
        Self {
            data: CoreData::default(),
        }
    }
}
