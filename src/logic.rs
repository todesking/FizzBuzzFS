use std::borrow::Cow;

pub fn char_at(index: i64) -> u8 {
    assert!(index >= 0);
    return b'X';
}

pub fn line_at(n: i64) -> Cow<'static, [u8]> {
    if n % 15 == 0 {
        Cow::from(b"FizzBuzz" as &'static [u8])
    } else if n % 3 == 0 {
        Cow::from(b"Fizz" as &'static [u8])
    } else if n % 5 == 0 {
        Cow::from(b"Buzz" as &'static [u8])
    } else {
        let s = format!("{}", n);
        let v = Vec::from(s.as_bytes());
        Cow::from(v)
    }
}

pub fn fizz_buzz_length(n: i64) -> i64 {
    assert!(n >= 0);
    let mut num_len_sum = 0i64;
    let mut num_count = 0i64;
    let mut fizz_count = 0i64;
    let mut buzz_count = 0i64;
    let mut fizz_buzz_count = 0i64;
    let max = i64::MAX / 10 + 1;
    let mut i = 10i64;
    let mut digits = 1;
    println!("Start: {}", n);
    while i <= n {
        let fizz_buzz_count0 = (i - 1) / 15 - fizz_buzz_count;
        let fizz_count0 = (i - 1) / 3 - fizz_buzz_count0 - fizz_count - fizz_buzz_count;
        let buzz_count0 = (i - 1) / 5 - fizz_buzz_count0 - buzz_count - fizz_buzz_count;
        fizz_buzz_count += fizz_buzz_count0;
        fizz_count += fizz_count0;
        buzz_count += buzz_count0;

        let num_count0 = i - fizz_count - buzz_count - fizz_buzz_count - num_count;
        num_count += num_count0;
        num_len_sum += (digits + 2) * num_count0;

        assert_eq!(num_count + fizz_count + buzz_count + fizz_buzz_count, i);

        digits += 1;
        if max <= i {
            break;
        }
        i *= 10;
    }

    let fizz_buzz_count0 = n / 15 - fizz_buzz_count;
    let fizz_count0 = n / 3 - fizz_buzz_count0 - fizz_count - fizz_buzz_count;
    let buzz_count0 = n / 5 - fizz_buzz_count0 - buzz_count - fizz_buzz_count;
    fizz_buzz_count += fizz_buzz_count0;
    fizz_count += fizz_count0;
    buzz_count += buzz_count0;

    let num_count0 = n + 1 - fizz_count - buzz_count - fizz_buzz_count - num_count;
    num_count += num_count0;

    num_len_sum += (digits + 2) * num_count0;

    assert_eq!(fizz_count + buzz_count + fizz_buzz_count + num_count, n + 1);

    // -3 for "0\r\n"
    num_len_sum + fizz_count * 6 + buzz_count * 6 + fizz_buzz_count * 10 - 3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(fizz_buzz_length(0), 0);
        assert_eq!(fizz_buzz_length(1), 3);
        assert_eq!(fizz_buzz_length(2), 6);
        assert_eq!(fizz_buzz_length(3), 12); // 3
        assert_eq!(fizz_buzz_length(4), 15);
        assert_eq!(fizz_buzz_length(5), 21); // 5
        assert_eq!(fizz_buzz_length(6), 27); // 3
        assert_eq!(fizz_buzz_length(7), 30);
        assert_eq!(fizz_buzz_length(8), 33);
        assert_eq!(fizz_buzz_length(9), 39); // 3
        assert_eq!(fizz_buzz_length(10), 45); // 5
        assert_eq!(fizz_buzz_length(11), 49);
        assert_eq!(fizz_buzz_length(12), 55); // 3
        assert_eq!(fizz_buzz_length(13), 59);
        assert_eq!(fizz_buzz_length(14), 63);
        assert_eq!(fizz_buzz_length(15), 73); // 35
        assert_eq!(fizz_buzz_length(16), 77);
    }
    #[test]
    fn test_many() {
        let mut l = 0i64;
        for i in 1..=11111 {
            if i % 3 == 0 && i % 5 == 0 {
                l += 10;
            } else if i % 3 == 0 {
                l += 6;
            } else if i % 5 == 0 {
                l += 6;
            } else {
                l += i.to_string().len() as i64 + 2;
            }
            let actual = fizz_buzz_length(i as i64);
            assert_eq!(
                actual, l,
                "fizz_buzz_length({})={}(expected {})",
                i, actual, l
            );
        }
    }
}
