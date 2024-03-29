pub trait CollectResult<T, E> {
    fn collect_res(self) -> Result<Vec<T>, E>;
}

impl<I, T, E> CollectResult<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    fn collect_res(self) -> Result<Vec<T>, E> {
        self.collect()
    }
}

#[test]
fn ext_test() {
    let err_string = "Invalid val";
    let vals = (0..10)
        .map(|item| match item % 2 == 0 {
            true => Ok(item),
            false => Err(err_string),
        })
        .collect_res();

    assert_eq!(vals, Err(err_string));
}
