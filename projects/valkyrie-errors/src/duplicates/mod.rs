use std::fmt::{Debug, Display, Formatter};

use crate::ValkyrieError;

mod kind;

#[derive(Copy, Clone)]
pub enum DuplicateKind {
    Type = 1002,
    Function = 1003,
    Variable = 1004,
}

#[derive(Clone, Debug)]
pub struct DuplicateError {
    kind: DuplicateKind,
    name: String,
    // this_item: FileSpan,
    // last_item: FileSpan,
}

impl Display for DuplicateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Duplicate {} `{}`", self.kind, self.name)
    }
}
//
// impl DuplicateError {
//     pub fn as_report(&self, level: ReportKind) -> ValkyrieReport {
//         let mut report = Report::build(level, self.this_item.file, 0).with_code(self.kind as u32);
//         report.set_message(self.to_string());
//         report.add_label(
//             self.this_item.as_label(format!("{:?} `{}` is defined here.", self.kind, self.name)).with_color(Color::Blue),
//         );
//         report.add_label(
//             self.last_item
//                 .as_label(format!("But {} `{}` has been defined here.", self.kind, self.name))
//                 .with_color(Color::Cyan),
//         );
//         report.set_help(format!("Items must have unique names, rename one of the items to have a unique name"));
//         report.finish()
//     }
// }
