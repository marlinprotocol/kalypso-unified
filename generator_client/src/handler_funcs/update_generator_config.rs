use crate::{
    kalypso::{read_generator_config_file, update_generator_config_file},
    model::UpdateGeneratorConfig,
};
use std::io::ErrorKind;

pub async fn _update_generator_config(json_input: &UpdateGeneratorConfig) -> anyhow::Result<()> {
    if json_input.supported_markets.as_ref().unwrap().len() > 1 {
        return Err(anyhow::Error::msg(
            "Only one market is supported for every generator".to_string(),
        ));
    }

    let config_file_call = read_generator_config_file().await;
    let mut config_file = match config_file_call {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            return Err(anyhow::Error::msg(
                "There was an issue while updating the generator config file".to_string(),
            ));
        }
    };

    //Finding the generator to update
    let generator_to_be_updated = json_input.address.as_ref().unwrap();
    let finding_generator_index = config_file
        .generator_config
        .iter()
        .position(|x| &x.address == generator_to_be_updated);

    let generator_index = match finding_generator_index {
        Some(data) => data,
        None => {
            return Err(anyhow::Error::msg(
                "No generator found for the provided address".to_string(),
            ));
        }
    };

    //Checking the input for changes
    if let Some(new_supported_markets) = &json_input.supported_markets {
        config_file.generator_config[generator_index].supported_markets =
            new_supported_markets.to_vec()
    }
    if let Some(new_data) = &json_input.data {
        config_file.generator_config[generator_index].data = new_data.to_string()
    }

    //Updating the config file
    let update_config_file = update_generator_config_file(config_file).await;
    match update_config_file {
        Ok(data) => data,
        Err(e) => {
            log::error!("{}", e);
            if e.kind() == ErrorKind::NotFound {
                return Err(anyhow::Error::msg("There was an issue in updating the config file, since the config file was not found".to_string()));
            }
            return Err(anyhow::Error::msg(
                "There was an issue in updating the config file".to_string(),
            ));
        }
    }

    Ok(())
}
