use nom::IResult;

/// Combine a parser 2 times
pub fn count_2<I, O, E, F>(fun: F) -> impl Fn(I) -> IResult<I, [O;2], E>
where
  F: Fn(I) -> IResult<I, O, E>,
{
  move |input: I| {
    let (input, o1) = fun(input)?;
    fun(input).map(|(i, o2)| (i, [o1, o2]))
  }
}

/// Combine a parser 5 times
pub fn count_5<I, O, E, F>(fun: F) -> impl Fn(I) -> IResult<I, [O;5], E>
where
  F: Fn(I) -> IResult<I, O, E>,
{
  move |input: I| {
    let (input, o1) = fun(input)?;
    let (input, o2) = fun(input)?;
    let (input, o3) = fun(input)?;
    let (input, o4) = fun(input)?;
    fun(input).map(|(i, o5)| (i, [o1, o2, o3, o4, o5]))
  }
}
