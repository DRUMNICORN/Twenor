use std::env;

use models::PromptResponse;
use rs_openai::{
    chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role},
    OpenAI,
};

pub async fn get_response(instruction: &String, prompt: &String) -> Result<PromptResponse, Box<dyn std::error::Error>> {
    log::info!("Getting response from OpenAI");
    let api_key =
        env::var("OPENAI_API_KEY")?;
    let client = OpenAI::new(&OpenAI {
        api_key,
        org_id: None,
    });

    let req = CreateChatRequestBuilder::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            // Add a system message here
            ChatCompletionMessageRequestBuilder::default()
                .role(Role::System)
                .content(instruction)
                .name("system") // Add name here
                .build()?,
            ChatCompletionMessageRequestBuilder::default()
                .role(Role::User)
                .content(prompt)
                .name("user") // Add name here
                .build()?,
        ])
        .build()?;
    log::info!("Sending request to OpenAI");
    let res = client.chat().create(&req).await?;
    log::info!("Got response from OpenAI");
    let response = res.choices[0].message.content.clone();
    Ok(PromptResponse::from_response(response))
}
