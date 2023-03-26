// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }
        println!("{:?}", optional_integers);
        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let

        // Nest Somes, interesting...
        while let Some(Some(i)) = optional_integers.pop() {
            //println!("{:?}", i);
            //println!("{:?}", range);
            assert_eq!(i, range);
            range -= 1;
        }
    }
}
