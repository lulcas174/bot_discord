extern crate reqwest;
use serde::Deserialize;
use reqwest::Error;
use std::fmt;

#[derive(Deserialize)]
pub struct Endereco {
    cep: String,
    logradouro: String,
    bairro: String,
    localidade: String,
    uf: String,
}

impl fmt::Display for Endereco {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CEP: {}\nLogradouro: {}\nBairro: {}\nLocalidade: {}\nUF: {}", self.cep, self.logradouro, self.bairro, self.localidade, self.uf)
    }
}

pub async fn get_cep_info(cep: String) -> Result<Endereco, Error> {
    let url = format!("https://viacep.com.br/ws/{cep}/json/");
    
    let response = reqwest::get(&url).await?;

    let _body = response.text().await?;
    
    let endereco:Endereco = serde_json::from_str(&_body).unwrap();

    Ok(endereco)
}
