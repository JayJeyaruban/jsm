pub trait PipeableExpression<A, R> {
    fn map<R2>(self, f: impl Fn(R) -> R2) -> impl Fn(A) -> R2;
}

impl<A, R, F> PipeableExpression<A, R> for F
where
    F: Fn(A) -> R,
{
    fn map<R2>(self, f: impl Fn(R) -> R2) -> impl Fn(A) -> R2 {
        move |args: A| f(self(args))
    }
}

#[cfg(test)]
mod test {
    use crate::pipe::PipeableExpression;

    #[test]
    fn test_simple() {
        let f = add(2).map(times(3)).map(|res| res.to_string());

        assert_eq!("15", f(3));
    }

    #[test]
    fn test_map_expr() {
        let f1 = add(2).map(times(3));
        let f2 = add(10).map(f1);

        assert_eq!(45, f2(3));
    }

    fn add(amount: i32) -> impl Fn(i32) -> i32 {
        move |v| v + amount
    }

    fn times(mul: i32) -> impl Fn(i32) -> i32 {
        move |v| v * mul
    }
}
