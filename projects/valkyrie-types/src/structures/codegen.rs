use super::*;

impl ConvertTo<StructureType> for ValkyrieStructure {
    fn convert(&self) -> StructureType {
        let mut item = StructureType::new(self.name());
        for field in self.fields.values() {
            item.add_field(field.convert());
        }
        item
    }
}