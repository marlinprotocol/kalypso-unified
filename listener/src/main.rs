use std::error::Error;
use std::fs;

use listener::job_creator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let generator_config_path = "./generator_config/generator_config.json".to_string();
    let alt_generator_config_path = "../generator_config/generator_config.json".to_string();
    let file_content = fs::read_to_string(&generator_config_path)
        .or_else(|_| fs::read_to_string(&alt_generator_config_path))?;

    println!("{}", &file_content);
    let config: job_creator::Config = serde_json::from_str(&file_content)?;

    let runtime_config_path = "./generator_config/runtime_config.json".to_string();
    let alt_runtime_config_path = "../generator_config/runtime_config.json".to_string();
    let file_content = fs::read_to_string(&runtime_config_path)
        .or_else(|_| fs::read_to_string(&alt_runtime_config_path))?;
    println!("{}", &file_content);
    let runtime_config: job_creator::RuntimeConfig = serde_json::from_str(&file_content)?;
    let _ = job_creator::JobCreator::new(config, runtime_config, false)
        .run()
        .await;
    Ok(())
}
