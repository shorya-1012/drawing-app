use dotenv::dotenv;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use sqlx::{Pool, Postgres};

use super::helpers::generate_verification_code;

pub fn verification_code_email_template() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta http-equiv="X-UA-Compatible" content="ie=edge" />
    <title>Static Template</title>

    <link
      href="https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;600&display=swap"
      rel="stylesheet"
    />
  </head>
  <body
    style="
      margin: 0;
      font-family: 'Poppins', sans-serif;
      background: #ffffff;
      font-size: 14px;
    "
  >
      <main>
        <div
          style="
            margin: 0;
            margin-top: 70px;
            padding: 92px 30px 115px;
            background: #ffffff;
            border-radius: 30px;
            text-align: center;
          "
        >
          <div style="width: 100%; max-width: 489px; margin: 0 auto;">
            <h1
              style="
                margin: 0;
                font-size: 24px;
                font-weight: 500;
                color: #1f1f1f;
              "
            >
              Your Verification Code
            </h1>
            <p
              style="
                margin: 0;
                margin-top: 17px;
                font-size: 16px;
                font-weight: 500;
              "
            >
            Hey {username},
            </p>
            <p
              style="
                margin: 0;
                margin-top: 17px;
                font-weight: 500;
                letter-spacing: 0.56px;
              "
            >
            Thanks for creating an account on Drawing App. Use the following code to 
            verify you account. This code is valid for
              <span style="font-weight: 600; color: #1f1f1f;">15 minutes</span>.
              Do not share this code with others, including employees.
            </p>
            <p
              style="
                margin: 0;
                margin-top: 60px;
                font-size: 40px;
                font-weight: 600;
                letter-spacing: 25px;
                color: #ba3d4f;
              "
            >
            {verify-code}
            </p>
          </div>
        </div>
      </main>
  </body>
</html>
        "#
    .to_string()
}

pub async fn send_email(
    db: &Pool<Postgres>,
    user_id: String,
    username: String,
    user_email: String,
) -> Result<String, String> {
    dotenv().ok();
    let email_username = std::env::var("APP_USERNAME").expect("username for email not found");
    let email_password = std::env::var("APP_PASSWORD").expect("password for email not found");

    let verifcation_code = generate_verification_code();
    let query = "INSERT INTO verification_codes (code , userid) values ($1 , $2)";

    match sqlx::query(query)
        .bind(&verifcation_code)
        .bind(user_id)
        .execute(db)
        .await
    {
        Ok(_) => println!("Added verifcation code"),
        Err(err) => {
            println!("{:#?}", err);
            return Err(err.to_string());
        }
    };

    let email_template = verification_code_email_template()
        .replace("{username}", &username)
        .replace("{verify-code}", &verifcation_code);

    let email = Message::builder()
        .from(email_username.parse().unwrap())
        .to(user_email.parse().unwrap())
        .header(ContentType::parse("text/html; charset=utf-8").unwrap())
        .subject("Test Email for verifying")
        .body(email_template)
        .unwrap();

    let creds = Credentials::new(email_username, email_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Added verifcation code"),
        Err(err) => {
            println!("{:#?}", err);
            return Err(err.to_string());
        }
    }

    return Ok("Email sent".to_string());
}
