pub fn greatest_profit(prices: &[u32]) -> Option<u32> {
    let (mut min, rest) = prices.split_first()?;
    let mut profit = 0;

    for p in rest {
        if p < min {
            min = p;
        } else {
            profit = profit.max(p - min);
        }
    }

    (profit > 0).then_some(profit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greatest_profit() {
        assert_eq!(greatest_profit(&[]), None);
        assert_eq!(greatest_profit(&[1]), None);
        assert_eq!(greatest_profit(&[2, 1]), None);
        assert_eq!(greatest_profit(&[1, 2]), Some(1));
        assert_eq!(greatest_profit(&[5, 1, 3]), Some(2));
        assert_eq!(greatest_profit(&[1, 2, 5, 3]), Some(4));
        assert_eq!(greatest_profit(&[5, 3, 2, 4, 5, 7, 6]), Some(5));
    }
}
