mod calculator1;
mod calculator1_actions;
mod calculator2;
mod calculator2_actions;

#[cfg(test)]
mod test_calculator1 {
    use crate::calculator1::Calculator1Parser;

    #[test]
    fn test_calculator1_1() {
        let result = Calculator1Parser::parse_str("2 + 3 * 7 + 6 * 3");
        assert_eq!(result.unwrap(), 41);
    }

    #[test]
    fn test_calculator1_2() {
        let result = Calculator1Parser::parse_str("2 + ( 3  * 7 ) + 2 * 4");
        assert_eq!(result.unwrap(), 31);
    }

}


#[cfg(test)]
mod test_calculator2 {
    use crate::calculator2;

    #[test]
    fn test_calculator2_1() {
        let result = calculator2::Calculator2Parser::parse_str("7 + 56.4 / 3 + 5 / 2 * (7 - 1)");
        assert_eq!(result.unwrap(), 40.800003f32);
    }
}
