use super::sendmail::send_mail;

pub async fn send_verification_email(
    to_email: &str,
    username: &str,
    token: &str
) -> Result<(), Box<dyn std::error::Error>>{
    let subject = "Email Verification";
    let template_path = "src/mail/templates/Verification-email.html";
    let base_url = "http://localhost:8000/api/auth/verify";
    let verification_link = create_verification_link(base_url, token);
    let placeholders = vec![(
        "{{username}}".to_string(), username.to_string()), 
        ("{{verification_link}}".to_string(), verification_link)];

    send_mail(to_email, subject, template_path, &placeholders).await
}

fn create_verification_link(base_url: &str, token:&str) -> String{
    format!("{}?token={}", base_url, token)
}

pub async fn welcome_email(username: &str, to_email: &str) -> Result<(), Box<dyn std::error::Error>>{
    let subject = "Welcome Niyi";
    let template_path = "src/mail/templates/Welcome-email.html";
    let placeholders = vec![("{{username}}".to_string(), username.to_string())];

    send_mail(to_email, subject, template_path, &placeholders).await
}

pub async fn send_forgot_password_email(
    to_email: &str,
    username: &str,
    reset_link: &str
) -> Result<(), Box<dyn std::error::Error>>{
    let subject = "Reset your password";
    let template_path = "src/mail/templates/RestPassword-email.html";
    let placeholders = vec![(
        "{{username}}".to_string(), username.to_string()), 
        ("{{reset_link}}".to_string(), reset_link.to_string())];

    send_mail(to_email, subject, template_path, &placeholders).await
}
