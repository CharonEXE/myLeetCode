impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 1 
        {
            return 0;
        }

        let mut cheapest: i32 = i32::MAX;
        let mut most_profit: i32 = 0;

        for price in prices.iter()
        {
            if price < &cheapest
            {
                cheapest = *price;
            } else {
                if most_profit < price - cheapest
                {
                    most_profit = price - cheapest
                }
            }
            
        }

        return most_profit;
    }
}