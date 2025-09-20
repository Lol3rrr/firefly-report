pub mod api;

pub struct Session {
    base_url: reqwest::Url,
    token: String,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum RequestError {
    ConstructUrl,
    BuildRequest,
    ExecuteRequest,
    DecodeResponse(reqwest::Error),
}

#[derive(Debug)]
pub enum FireflyError {
    ConstructUrl(url::ParseError),
    BuildRequest(reqwest::Error),
    ExecuteRequest(reqwest::Error),
    ErrorResponse(reqwest::Error),
    ReceiveResponse(reqwest::Error),
    DecodeResponse(serde_json::Error, String),
}

impl Session {
    pub fn new(url: impl reqwest::IntoUrl, token: impl Into<String>) -> Self {
        Self {
            base_url: url.into_url().unwrap(),
            token: token.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn load_bills(&self) -> Result<Vec<api::Bill>, FireflyError> {
        let url = self.base_url.join("api/v1/bills").map_err(|e| FireflyError::ConstructUrl(e))?;

        let request = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .build()
            .map_err(|e| FireflyError::BuildRequest(e))?;
        let response = self.client.execute(request).await.map_err(|e| FireflyError::ExecuteRequest(e))?;

        let response = response.error_for_status().map_err(|e| FireflyError::ErrorResponse(e))?;

        let raw_data = response.text().await.map_err(|e| FireflyError::ReceiveResponse(e))?;
        let data: api::FireflyResponse<Vec<api::Bill>> = serde_json::from_str(&raw_data).map_err(|e| FireflyError::DecodeResponse(e, raw_data))?;

        Ok(data.data)
    }

    pub async fn load_budgets(
        &self,
        timerange: Option<(chrono::NaiveDate, chrono::NaiveDate)>,
    ) -> Result<Vec<api::Budget>, ()> {
        let base_url = self.base_url.join("api/v1/budgets").map_err(|e| ())?;
        let url: reqwest::Url = match timerange {
            None => base_url,
            Some((start, end)) => {
                let mut tmp = base_url;
                tmp.set_query(Some(&format!("start={}&end={}", start, end)));
                tmp
            }
        };

        let request = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .build()
            .map_err(|e| ())?;
        let response = self.client.execute(request).await.map_err(|e| ())?;

        let response = response.error_for_status().map_err(|e| ())?;

        let data: api::FireflyResponse<Vec<api::Budget>> = response.json().await.map_err(|e| ())?;

        Ok(data.data)
    }

    pub async fn load_categories(&self) -> Result<Vec<api::ListCategory>, ()> {
        let url = self.base_url.join("api/v1/categories").map_err(|e| ())?;

        let request = self.client.get(url).bearer_auth(&self.token).build().map_err(|e| ())?;
        let response = self.client.execute(request).await.map_err(|e| ())?;

        let data: api::FireflyResponse<Vec<api::ListCategory>> = response.json().await.map_err(|e| ())?;

        Ok(data.data)
    }

    pub async fn load_category(&self, category: &str, timerange: Option<(chrono::NaiveDate, chrono::NaiveDate)>) -> Result<api::DetailsCategory, RequestError> {
        let url = self.base_url.join("api/v1/categories/").map_err(|e| RequestError::ConstructUrl)?.join(category).map_err(|e| RequestError::ConstructUrl)?;
        let url: reqwest::Url = match timerange {
            None => url,
            Some((start, end)) => {
                let mut tmp = url;
                tmp.set_query(Some(&format!("start={}&end={}", start, end)));
                tmp
            }
        };

        let request = self.client.get(url).bearer_auth(&self.token).build().map_err(|e| RequestError::BuildRequest)?;
        let response = self.client.execute(request).await.map_err(|e| RequestError::BuildRequest)?;

        let data: api::FireflyResponse<api::DetailsCategory> = response.json().await.map_err(|e| RequestError::DecodeResponse(e))?;

        Ok(data.data)
    }
}
