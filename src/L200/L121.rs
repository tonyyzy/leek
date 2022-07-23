use crate::Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for p in prices {
            if p < min_price {
                min_price = p
            }
            if p - min_price > max_profit {
                max_profit = p - min_price
            }
        }
        max_profit
    }
}
