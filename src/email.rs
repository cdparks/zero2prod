use crate::domain::Email;
use reqwest::Client;
use std::time::Duration;

#[derive(Debug)]
pub struct EmailClient {
    client: Client,
    sender: Email,
    base_url: String,
    auth_token: String,
}

impl EmailClient {
    pub fn new(base_url: String, sender: Email, auth_token: String, timeout: Duration) -> Self {
        let client = reqwest::Client::builder().timeout(timeout).build().unwrap();

        Self {
            client,
            base_url,
            sender,
            auth_token,
        }
    }

    pub async fn send(
        &self,
        recipient: Email,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);
        let body = Body {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject,
            html_body,
            text_body,
        };

        self.client
            .post(&url)
            .header("X-Postmark-Server-Token", &self.auth_token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct Body<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

#[cfg(test)]
mod tests {
    use super::EmailClient;
    use crate::email::Email;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    struct MatchBody;

    impl wiremock::Match for MatchBody {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            match result {
                Ok(body) => ["From", "To", "Subject", "HtmlBody", "TextBody"]
                    .iter()
                    .all(|field| body.get(field).is_some()),
                Err(_) => false,
            }
        }
    }

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> Email {
        SafeEmail().fake::<String>().parse().unwrap()
    }

    fn client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            Faker.fake(),
            super::Duration::from_millis(200),
        )
    }

    #[tokio::test]
    async fn sends_expected_request() {
        let server = MockServer::start().await;
        let client = client(server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(MatchBody)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let result = client
            .send(email(), &subject(), &content(), &content())
            .await;
        assert_ok!(result);
    }

    #[tokio::test]
    async fn sends_fails_if_server_500s() {
        let server = MockServer::start().await;
        let client = client(server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(MatchBody)
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;

        let result = client
            .send(email(), &subject(), &content(), &content())
            .await;
        assert_err!(result);
    }
}
