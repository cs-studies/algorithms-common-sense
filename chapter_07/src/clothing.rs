pub fn mark_inventory(clothes: &[&str], max_size: i8) -> Vec<String> {
    let mut inventory: Vec<String> = Vec::new();
    for cloth in clothes {
        for size in 1..=max_size {
            inventory.push(format!("{cloth} Size: {}", size));
        }
    }
    inventory
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_inventory() {
        assert_eq!(Vec::<String>::new(), mark_inventory(&[], 5));
        let inventory = &["Purple Shirt", "Green Shirt"];
        let marked_inventory_1 = vec!["Purple Shirt Size: 1", "Green Shirt Size: 1"];
        assert_eq!(marked_inventory_1, mark_inventory(inventory, 1));
        let marked_inventory_3 = vec![
            "Purple Shirt Size: 1",
            "Purple Shirt Size: 2",
            "Purple Shirt Size: 3",
            "Green Shirt Size: 1",
            "Green Shirt Size: 2",
            "Green Shirt Size: 3",
        ];
        assert_eq!(marked_inventory_3, mark_inventory(inventory, 3));
    }
}
