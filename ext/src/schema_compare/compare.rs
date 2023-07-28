use bluejay_core::definition::SchemaDefinition;
use super::changes::Change;
use super::changes::Criticality;

pub struct Result<'a, S: SchemaDefinition> {
    breaking_changes: Vec<Change<'a, S>>,
    non_breaking_changes: Vec<Change<'a, S>>,
    dangerous_changes: Vec<Change<'a, S>>,
}

impl<'a, S: SchemaDefinition> Result<'a, S> {
    pub fn new(mut changes: Vec<Change<'a, S>>) -> Self {
        changes.sort_by(|a: &Change<'a, S>, b: &Change<'a, S>| b.criticality().cmp(&a.criticality()));

        let (breaking_changes, non_breaking_changes, dangerous_changes) = changes
            .into_iter()
            .fold((Vec::new(), Vec::new(), Vec::new()), |(mut breaking, mut non_breaking, mut dangerous), change| {
                match change.criticality() {
                    Criticality::Breaking { .. } => breaking.push(change),
                    Criticality::Dangerous { .. } => dangerous.push(change),
                    Criticality::NonBreaking { .. } => non_breaking.push(change),
                }

                (breaking, non_breaking, dangerous)
            });

        Self {
            breaking_changes,
            non_breaking_changes,
            dangerous_changes,
        }
    }

    pub fn changes(&self) -> Vec<&Change<'a, S>> {
        let mut changes: Vec<&Change<'a, S>> = Vec::new();

        changes.extend(self.breaking_changes.iter());
        changes.extend(self.dangerous_changes.iter());
        changes.extend(self.non_breaking_changes.iter());

        changes
    }

    pub fn is_breaking(&self) -> bool {
        !self.breaking_changes.is_empty()
    }
}
