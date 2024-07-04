use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub metadata: Value,
    pub spec: Spec,
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub sources: Vec<Source>,
    pub destination: Value,
    pub project: String,
    pub sync_policy: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "repoURL")]
    pub repo_url: String,
    pub path: Option<String>,
    pub target_revision: Option<String>,
    pub helm: Option<Helm>,
    #[serde(rename = "ref")]
    pub reference: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Helm {
    pub value_files: Option<Vec<String>>, // Changed to Option<Vec<String>>
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

impl Application {
    pub fn add_image_tag(&mut self, image_tag: String) {
        for source in self.spec.sources.iter_mut() {
            if let Some(ref mut helm) = source.helm {
                let mut found = false;
                if let Some(ref mut parameters) = helm.parameters {
                    for param in parameters.iter_mut() {
                        if param.name == "image.tag" {
                            param.value = image_tag.clone();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        parameters.push(Parameter {
                            name: "image.tag".to_string(),
                            value: image_tag.clone(),
                        });
                    }
                } else {
                    helm.parameters = Some(vec![Parameter {
                        name: "image.tag".to_string(),
                        value: image_tag.clone(),
                    }]);
                }
            }
        }
    }
}
