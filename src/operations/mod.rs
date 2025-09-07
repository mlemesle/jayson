use std::{fs::File, ops::Deref, path::PathBuf};

mod constant;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operations {
    operations: Vec<Operation>,
}

pub trait Op {
    fn apply(&self, json: &mut serde_json::Value) -> anyhow::Result<()>;
}

impl Operations {
    pub fn try_new(operations_filepath: &PathBuf) -> anyhow::Result<Self> {
        let operations_file = File::open(operations_filepath)?;
        let ops = serde_yaml::from_reader(operations_file)?;

        Ok(ops)
    }

    pub fn process(&self, json: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
        let mut work_copy = json.clone();

        for op in &self.operations {
            op.apply(&mut work_copy)?;
        }

        Ok(work_copy)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "op", deny_unknown_fields, rename_all = "camelCase")]
pub enum Operation {
    Constant(constant::Constant),
}

impl Deref for Operation {
    type Target = dyn Op;

    fn deref(&self) -> &Self::Target {
        match self {
            Operation::Constant(constant) => constant,
        }
    }
}
