pub mod api;

pub struct Session {
    base_url: reqwest::Url,
    token: String,
    client: reqwest::Client,
}

impl Session {
    pub fn new(url: impl reqwest::IntoUrl, token: impl Into<String>) -> Self {
        Self {
            base_url: url.into_url().unwrap(),
            token: token.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn load_bills(&self) -> Result<Vec<api::Bill>, ()> {
        let url = self.base_url.join("api/v1/bills").map_err(|e| ())?;

        let request = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .build()
            .map_err(|e| ())?;
        let response = self.client.execute(request).await.map_err(|e| ())?;

        let response = response.error_for_status().map_err(|e| ())?;

        let data: api::FireflyResponse<Vec<api::Bill>> = response.json().await.map_err(|e| ())?;

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
}
