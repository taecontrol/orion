use crate::open_ai::{OpenAI, OpenAIMessage, OpenAIRequest};

#[tauri::command]
pub async fn ask(message: String) {
    let data = OpenAIRequest {
        model: String::from("gpt-3.5-turbo"),
        messages: vec![OpenAIMessage {
            role: String::from("user"),
            content: message,
        }],
        temperature: 0.7,
    };

    let response = OpenAI::new().chat().create(&data).await;

    match response {
        Ok(response) => {
            let answer = response.choices[0].message.content.clone();
            dbg!(answer);
        },
        Err(error) => {
            dbg!(error);
        }
    };
}
