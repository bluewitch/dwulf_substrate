
struct coins {
    coin_name: String,
    coin_index: String,
    coin_value: u64,
}

let coin_data = coins {
        coin_name: String::from("Bitcoin"),
        coin_index: String::from("BTC"),
        coin_value: u64,
    };

fn build_coin(coin_name: String, coin_index: String, coin_value: u64) -> Coins {
    Coins {
        coin_name: coin_name,
        coin_index: coin_index,
        coin_value: coin_value,
    }
}
