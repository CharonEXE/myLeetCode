fn main() {
    println!("Max Profit = {}", max_profit(vec![7, 1, 5, 3, 6, 4]));
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cheapest: i32 = i32::MAX;
    let mut most_profit: i32 = 0;

    let iterator = prices.iter();

    for price in iterator
    {
        if price < &cheapest
        {
            println!("Trigger Cheapest");
            cheapest = *price;
        }
        
        if most_profit < price - cheapest
        {
            most_profit = price - cheapest
        }
        println!("{}", most_profit);
    }

    return most_profit;
}