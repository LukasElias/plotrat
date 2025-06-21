struct MathParser {
}

enum ExprType {
    FunctionDefinition {
        name: String,
        param: char,
        body: CalculatableExpr, // your internal expression representation
    },
    Equation {
        left: CalculatableExpr,
        op: ComparisonOp, // <, >, <=, >=, =, !=
        right: CalculatableExpr,
    },
    CalculatableExpression(CalculatableExpr),
    VariableAssignment {
        name: String,
        value: CalculatableExpr,
    },
    Unknown,
}

struct CalculatableExpr(Vec<SimpleToken>);

enum ComplexToken {
    Number(f32),
    Operator(Operator),
    ComparisonOp(ComparisonOp),
    Variable(char),
    Constant(char),
    Function(String),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Unknown(char),
}

enum SimpleToken {
    Number(f32),
    Operator(Operator),
    Variable(char),
    Constant(char),
    Function(String),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Unknown(char),
}

enum ComparisonOp {
    LessThan,             // <
    GreaterThen,          // >
    LessThanOrEqualTo,    // <=
    GreaterThenOrEqualTo, // >=
    EqualTo,              // =
    NotEqualTo,           // !=
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

enum ShuntingYardOperator {
    Operator(Operator),
    Function(String),
    LeftParanthesis,
}

/*

Rules for when an expression is a function

1. It has an single character next to a parathesis with a variable inside e.g. f(x)
2. There is either a y and x in the expr, or there's a function definition as defined in rule 1
3. There is a LHS and RHS, e.g. there's an equal sign =


Steps for parsing a math expression

1. Get the string, for example "g(y)*4=2y-4/10"
2. Figure out what type of expression this is, e.g. function, equation with unknowns, or maybe a simple equation with a single constant output etc... In this example it would be a function called g, with a variable y
3. Then simplifying till it's something that can be either plotted or calculated, which would be g(y)=(2y-4/10)/4
4. Optional, but simplifying that to the simplest form which would be g(y)=y/2-0.1
5. Then using picking the right implementation for this type of expression which is known since the 2nd step. Fx plotting a function

*/


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
