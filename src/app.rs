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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_add_new_image_tag() {
        let mut app = Application {
            metadata: json!({ "name": "test-app" }),
            spec: Spec {
                sources: vec![Source {
                    repo_url: "https://test.com/repo".to_string(),
                    path: Some("path/to/chart".to_string()),
                    target_revision: Some("main".to_string()),
                    helm: Some(Helm {
                        value_files: Some(vec!["values.yaml".to_string()]),
                        parameters: Some(vec![Parameter {
                            name: "existing.param".to_string(),
                            value: "value".to_string(),
                        }]),
                    }),
                    reference: None,
                }],
                destination: json!({ "namespace": "default" }),
                project: "default".to_string(),
                sync_policy: json!({ "policy": "auto" }),
            },
            status: None,
        };

        app.add_image_tag("new-tag".to_string());

        if let Some(helm) = &app.spec.sources[0].helm {
            let parameters = helm.parameters.as_ref().unwrap();
            assert!(parameters.iter().any(|p| p.name == "image.tag" && p.value == "new-tag"));
        } else {
            panic!("Helm configuration not found");
        }
    }

    #[tokio::test]
    async fn test_update_existing_image_tag() {
        let mut app = Application {
            metadata: json!({ "name": "test-app" }),
            spec: Spec {
                sources: vec![Source {
                    repo_url: "https://example.com/repo".to_string(),
                    path: Some("path/to/chart".to_string()),
                    target_revision: Some("main".to_string()),
                    helm: Some(Helm {
                        value_files: Some(vec!["values.yaml".to_string()]),
                        parameters: Some(vec![
                            Parameter {
                                name: "existing.param".to_string(),
                                value: "value".to_string(),
                            },
                            Parameter {
                                name: "image.tag".to_string(),
                                value: "old-tag".to_string(),
                            }
                        ]),
                    }),
                    reference: None,
                }],
                destination: json!({ "namespace": "default" }),
                project: "default".to_string(),
                sync_policy: json!({ "policy": "auto" }),
            },
            status: None,
        };

        app.add_image_tag("new-tag".to_string());

        if let Some(helm) = &app.spec.sources[0].helm {
            let parameters = helm.parameters.as_ref().unwrap();
            assert!(parameters.iter().any(|p| p.name == "image.tag" && p.value == "new-tag"));
            assert!(parameters.iter().any(|p| p.name == "existing.param" && p.value == "value"));
        } else {
            panic!("Helm configuration not found");
        }
    }

    #[tokio::test]
    async fn test_add_image_tag_when_parameters_none() {
        let mut app = Application {
            metadata: json!({ "name": "test-app" }),
            spec: Spec {
                sources: vec![Source {
                    repo_url: "https://example.com/repo".to_string(),
                    path: Some("path/to/chart".to_string()),
                    target_revision: Some("main".to_string()),
                    helm: Some(Helm {
                        value_files: Some(vec!["values.yaml".to_string()]),
                        parameters: None,
                    }),
                    reference: None,
                }],
                destination: json!({ "namespace": "default" }),
                project: "default".to_string(),
                sync_policy: json!({ "policy": "auto" }),
            },
            status: None,
        };

        app.add_image_tag("new-tag".to_string());

        if let Some(helm) = &app.spec.sources[0].helm {
            let parameters = helm.parameters.as_ref().unwrap();
            assert!(parameters.iter().any(|p| p.name == "image.tag" && p.value == "new-tag"));
        } else {
            panic!("Helm configuration not found");
        }
    }

    #[tokio::test]
    async fn test_other_fields_not_modified() {
        let mut app = Application {
            metadata: json!({ "name": "test-app" }),
            spec: Spec {
                sources: vec![Source {
                    repo_url: "https://example.com/repo".to_string(),
                    path: Some("path/to/chart".to_string()),
                    target_revision: Some("main".to_string()),
                    helm: Some(Helm {
                        value_files: Some(vec!["values.yaml".to_string()]),
                        parameters: None,
                    }),
                    reference: None,
                }],
                destination: json!({ "namespace": "default" }),
                project: "default".to_string(),
                sync_policy: json!({ "policy": "auto" }),
            },
            status: Some(json!({ "status": "healthy" })),
        };

        app.add_image_tag("new-tag".to_string());

        assert_eq!(app.metadata["name"], "test-app");
        assert_eq!(app.status.as_ref().unwrap()["status"], "healthy");
    }
}
