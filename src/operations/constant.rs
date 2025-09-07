use super::Op;

#[derive(Debug, serde::Deserialize)]
pub struct Constant {
    target: String,
    value: serde_json::Value,
}

impl Op for Constant {
    fn apply(&self, json: &mut serde_json::Value) -> anyhow::Result<()> {
        if let Some(object) = json.as_object_mut() {
            object.insert(self.target.clone(), self.value.clone());
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Can't add a field in a non object json value !"
            ))
        }
    }
}
