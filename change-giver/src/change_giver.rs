pub mod coins;

use coins::Coins;


pub fn get_change(amt: &mut f32) -> Vec<(Coins, u32)> {
    let mut coins: Vec<(Coins, u32)> = Vec::new();

    for coin in &Coins::as_vec() {
        let mut num_of_coins = 0;

        loop {
            let coin_value = coin.value();

            if *amt < coin_value {
                break;
            } else {
                *amt -= coin_value;

                // Ensure no floating point precision errors
                *amt = (*amt * 100.0).round() / 100.0;
                num_of_coins += 1;
            }
        }
        coins.push((coin.as_enum(), num_of_coins))
    }

    coins
}
