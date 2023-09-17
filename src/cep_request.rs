extern crate reqwest;

use reqwest::Error;

pub async fn get_cep_info(cep: String) -> Result<String, Error> {
    let url = format!("https://viacep.com.br/ws/{cep}/json/");
    
    let response = reqwest::get(&url).await?;

    let _body = response.text().await?;
    
    Ok(_body)
}
