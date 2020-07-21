use super::Component;

pub struct Entity {
    id: u64,
    components: Vec<Box<dyn Component>>
}

impl Entity {
    pub fn get_components(&self) -> &Vec<Box<dyn Component>> {
        &self.components
    }

    pub fn get_mut_components(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.components
    }
}
