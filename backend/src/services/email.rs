use crate::config::EmailSettings;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

#[allow(dead_code)]
pub struct EmailService {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum EmailError {
    Transport(lettre::transport::smtp::Error),
    Message(lettre::error::Error),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::Transport(e) => write!(f, "Transport error: {}", e),
            EmailError::Message(e) => write!(f, "Message error: {}", e),
        }
    }
}

impl std::error::Error for EmailError {}

impl From<lettre::transport::smtp::Error> for EmailError {
    fn from(e: lettre::transport::smtp::Error) -> Self {
        EmailError::Transport(e)
    }
}

impl From<lettre::error::Error> for EmailError {
    fn from(e: lettre::error::Error) -> Self {
        EmailError::Message(e)
    }
}

#[allow(dead_code)]
impl EmailService {
    pub fn new(settings: &EmailSettings) -> Result<Self, lettre::transport::smtp::Error> {
        let creds = Credentials::new(
            settings.smtp_user.clone(),
            settings.smtp_password.clone(),
        );

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.smtp_host)?
            .port(settings.smtp_port)
            .credentials(creds)
            .build();

        Ok(Self {
            transport,
            from_email: settings.from_email.clone(),
            from_name: settings.from_name.clone(),
        })
    }

    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), EmailError> {
        let email = Message::builder()
            .from(
                format!("{} <{}>", self.from_name, self.from_email)
                    .parse()
                    .map_err(|_| lettre::error::Error::MissingFrom)?,
            )
            .to(to.parse().map_err(|_| lettre::error::Error::MissingTo)?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())?;

        match self.transport.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => Err(EmailError::Transport(e)),
        }
    }

    pub async fn send_password_reset(
        &self,
        to: &str,
        reset_link: &str,
    ) -> Result<(), EmailError> {
        let subject = "Reset your Jojuhu password";
        let body = format!(
            r#"<html>
            <body>
                <h2>Password Reset Request</h2>
                <p>You requested to reset your password. Click the link below to proceed:</p>
                <p><a href="{}">Reset Password</a></p>
                <p>This link will expire in 1 hour.</p>
                <p>If you didn't request this, you can safely ignore this email.</p>
            </body>
            </html>"#,
            reset_link
        );

        self.send_email(to, subject, &body).await
    }

    pub async fn send_verification_email(
        &self,
        to: &str,
        verification_link: &str,
    ) -> Result<(), EmailError> {
        let subject = "Verify your Jojuhu email address";
        let body = format!(
            r#"<html>
            <body>
                <h2>Email Verification</h2>
                <p>Welcome to Jojuhu! Please verify your email address by clicking the link below:</p>
                <p><a href="{}">Verify Email</a></p>
                <p>If you didn't create an account, you can safely ignore this email.</p>
            </body>
            </html>"#,
            verification_link
        );

        self.send_email(to, subject, &body).await
    }
}