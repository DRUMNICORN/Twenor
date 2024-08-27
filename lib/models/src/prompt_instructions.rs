pub struct PromptInstructions {
    instruction: String,
    base_prompt: String,
    director_prompt: String,
    goal_prompt: String,
    output_format_prompt: String,
    example_scene: String,
    topic_guidance_prompt: String,
}

 impl PromptInstructions {
    pub fn default() -> PromptInstructions {
        PromptInstructions {
            instruction: "[INSTRUCTION]".to_string(),
            base_prompt: "Write a music video script based on the following audio information.".to_string(),
            director_prompt: "Act as a director and create a script for a music video based on the given audio information.".to_string(),
            goal_prompt: "Write an interesting and engaging script.".to_string(),
            output_format_prompt: "Output a JSON array of scenes in the following format:".to_string(),
            example_scene: "{
                \"script\": {
                    \"title\": \"...\",  # title of the script
                    \"description\": \"...\",  # description of the script, one sentence (up to twenty words)
                },
                \"scenes\": [-
                    {
                        \"scene_id\": 0,  # unique scene id
                        \"scene_title\": \"...\",  # title of the scene
                        \"scene_description\": \"...\",  # description of the scene, one sentence (up to twenty words)
                        \"scene_tags\": [\"...\", \"...\", \"...\"],  # tags of the scene (up to twenty tags)
                        \"scene_color\": \"#...\",  # creative fitting Color of the scene in hex format
                    },
                    { 
                        # Additional scenes
                    },
                    ...
                ]
            }".to_string(),
            topic_guidance_prompt: "Use the following audio information as guidance and follow the instructions below:".to_string(),
        }
    }

    pub fn get(&self) -> String {
       format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n\n",
            self.instruction,
            self.base_prompt,
            self.director_prompt,
            self.goal_prompt,
            self.output_format_prompt,
            self.example_scene,
            self.topic_guidance_prompt,
        )
    }
 }