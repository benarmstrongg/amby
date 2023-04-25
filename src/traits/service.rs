use crate::{Entity, Name, Path};

pub trait Service {
    fn name(&self) -> Name;
    fn get_entities(&self) -> Vec<Box<dyn Entity + 'static>>;

    fn path(&self) -> Path {
        Path::from_str_unchecked(&format!("/{}", self.name()))
    }

    fn entity(&self, entity: impl Entity + 'static) -> Box<dyn Entity + 'static> {
        Box::new(entity)
    }
}
