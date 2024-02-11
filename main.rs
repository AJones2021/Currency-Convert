use slint::SharedString;
use std::collections::HashMap;
use reqwest;
use serde::{Deserialize, Serialize};
slint::include_modules!();


#[derive(Serialize, Deserialize)]
struct CurrencyResponse {
    new_amount: f64,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_calc_convert(move |have: SharedString, amount: SharedString, want: SharedString| {

        let ui = ui_handle.unwrap();
        let have_val: String = have.trim().parse().unwrap();
        let amount_val: String = amount.trim().parse().unwrap();
        let want_val: String = want.trim().parse().unwrap();

        match convert_api(&have_val, &want_val, &amount_val) {
            Ok(result) => ui.set_results(result.into()),
            Err(err) => eprintln!("Error: {}", err),
        }
    });

    ui.run()
}

fn convert_api(have: &str, want: &str, amount: &str) -> Result<String, String> {

    let url = "https://currency-converter-by-api-ninjas.p.rapidapi.com/v1/convertcurrency";


    let money_sign = match want {
        "USD" => '$',
        "EUR" => '€',
        "CAD" => 'C',

        _ => '$',
    };

    let mut querystring = HashMap::new();
    querystring.insert("have", have);
    querystring.insert("want", want);
    querystring.insert("amount", amount);

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "X-RapidAPI-Key",
                reqwest::header::HeaderValue::from_static("f7b873b383mshf919bb732781e22p19d747jsneb32b8d4cb32"),
            );
            headers.insert(
                "X-RapidAPI-Host",
                reqwest::header::HeaderValue::from_static("currency-converter-by-api-ninjas.p.rapidapi.com"),
            );
            headers
        })
        .query(&querystring)
        .send()
        .unwrap();

    let data: CurrencyResponse = serde_json::from_str(&response.text().unwrap()).unwrap();

    let amount_converted = data.new_amount;
    let result_string = format!("{}{:0.2}", money_sign, amount_converted);

    Ok(result_string)
}