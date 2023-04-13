use crate::Entity;

pub trait Service {
    fn name(&self) -> String;
    fn get_entities(&self) -> Vec<Box<dyn Entity + 'static>>;

    fn path(&self) -> String {
        format!("/{}", self.name())
    }

    fn entity(&self, entity: impl Entity + 'static) -> Box<dyn Entity + 'static> {
        Box::new(entity)
    }
}
