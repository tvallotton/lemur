
#[cfg(test)]
mod test {
    use crate::parser::stream::*;
    #[test]
    fn test_iterator() {
        let string = String::from("sadknf asdjfek \nasdfm");
        let stream = Stream::from(&string);

        for (c0, c1) in stream.zip(string.chars()) {
            assert_eq!(c0, c1);
        }
    }
}
