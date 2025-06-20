struct MathParser;

impl MathParser {
    pub fn parse(input_str: &str) -> f32{
        let mut sum = 0.0;
        for term in input_str.split('+') {
            let mut product = 1.0;
            for mul_term in term.split('*') {
                product *= mul_term.to_string().parse::<f32>().expect("It wasn't a number");
            }
            sum += product;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::MathParser;

    #[test]
    fn test_mathparser() {
        let string = "100*20*11*58+11+22*59+1.5";

        let result = MathParser::parse(string);

        assert_eq!(result, 100.0*20.0*11.0*58.0+11.0+22.0*59.0+1.5);
    }
}
