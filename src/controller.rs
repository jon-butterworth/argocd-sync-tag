use crate::api::BuildApi;
use anyhow::Error;

pub struct Controller {
    api: BuildApi,
}

impl Controller {
    pub fn new(api: BuildApi) -> Self {
        Self { api }
    }
    pub async fn sync(&self, app_name: &str, image_tag: &str) -> Result<(), Error> {
        let mut application = self.api.get_application(app_name).await?;
        application.add_image_tag(image_tag.to_string());
        self.api.update_application(app_name, &application).await?;
        self.api.sync(app_name).await.map_err(Error::new)
    }
}
