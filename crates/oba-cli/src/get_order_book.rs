use comfy_table::{presets::UTF8_FULL, Table};
use oba_client::{client::OBAClient, models::GetOrderBookResponse};



pub async fn get_order_books(address: &str, instrument: &str) {

    let client = OBAClient::from_endpoint_str(address);
    let response = client.get_order_book(instrument).await;

    match response {
        Ok(response) => {
            print_bids_and_asks(response);
        },
        Err(e) => {
            println!("{}", e.to_string());
        }
        
    }
}


fn print_bids_and_asks(response: GetOrderBookResponse) {
    
    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
        .set_header(vec![
            "Bids",
            "Asks"
        ]);


    let mut rows: Vec<Vec<String>> = Vec::new();
    for (price, amount) in response.bids.into_iter().rev() {
        let mut row = Vec::new();
        row.push(format!("{} @ {}", amount.to_string(), price.to_string()));
        rows.push(row);
    }

    let mut i = 0 as usize;
    for (price, amount) in response.asks {
        let row = rows.get_mut(i).unwrap();
        row.push(format!("{} @ {}", amount.to_string(), price.to_string()));
        i = i + 1;
    }

    table.add_rows(rows);

    println!("{table}");
}
