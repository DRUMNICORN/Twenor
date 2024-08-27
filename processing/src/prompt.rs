use std::env;

pub fn handle_prompt_state(track_id: i32, database_config: &crate::db::DatabaseConfig) {
    let script = crate::writing::ScriptDB::load_script(track_id, database_config);

    let prompt = script.get_prompt();

    // Load OpenAI API key from environment variable
    let openai_api_key = env::var("OPENAI_API_KEY")
        .expect("Please set the OPENAI_API_KEY environment variable");

    // Create OpenAI client
    let client = openai_api::Client::new(&openai_api_key);

    let args = openai_api::api::CompletionArgs::builder()
    .prompt("Once upon a time,")
    .engine("text-davinci-003")
    .max_tokens(20)
    .temperature(0.7)
    .top_p(0.9)
    .stop(vec!["\n".into()]);
    let completion = client.complete_prompt(args).await?;
    println!("Response: {}", response.choices[0].text);
    println!("Model used: {}", response.model);

    // Process the response
    println!("Generated Response: {}", response.choices[0].text);
}
