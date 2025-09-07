use std::{fs::File, path::PathBuf};

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operations {
    operations: Vec<Operation>,
}

impl Operations {
    pub fn try_new(operations_filepath: &PathBuf) -> anyhow::Result<Self> {
        let operations_file = File::open(operations_filepath)?;
        let ops = serde_yaml::from_reader(operations_file)?;

        Ok(ops)
    }

    pub fn apply(&self, json: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
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
    Constant {
        target: String,
        value: serde_json::Value,
    },
}

impl Operation {
    pub fn apply(&self, json: &mut serde_json::Value) -> anyhow::Result<()> {
        match self {
            Operation::Constant { target, value } => {
                if let Some(object) = json.as_object_mut() {
                    object.insert(target.clone(), value.clone());
                    Ok(())
                } else {
                    Err(anyhow::anyhow!(
                        "Can't add a field in a non object json value !"
                    ))
                }
            }
        }
    }
}
