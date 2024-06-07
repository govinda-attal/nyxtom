use std::collections::VecDeque;

enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    let results = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u32>, Error>>()?;

    results
        .windows(span)
        .map(|nn| nn.iter().map(|&n| n as u64).product())
        .max()
        .ok_or(Error::SpanTooLong)
}

fn largest_series_product(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    let mut dq: VecDeque<u32> = VecDeque::new();
    let mut max = 0 as u64;
    let mut prod = 1 as u64;
    let mut zeros = 0;

    for c in string_digits.chars() {
        let n = c.to_digit(10).ok_or(Error::InvalidDigit(c))?;
        dq.push_front(n);

        if n == 0 {
            zeros += 1;
        } else {
            prod *= n as u64;
        }

        if dq.len() > span {
            let prev = dq.pop_back().unwrap();
            if prev == 0 {
                zeros -= 1;
            } else {
                prod /= prev as u64;
            }
        }

        if dq.len() == span && prod > max && zeros == 0 {
            max = prod;
        }
    }
    Ok(max)
}
