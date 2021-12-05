use super::input;
use anyhow::Result;

pub fn print_power_consumption_and_life_support_rating(input: impl std::io::Read) -> Result<()> {
    let lines = input::get_input_lines(input)?;

    let (g, e) = get_gamma_epsilon(lines.clone())?;
    println!(
        "Gamma = {}, Epsilon = {}, PowerConsumption = {}",
        g,
        e,
        g * e
    );

    let rows: Vec<Vec<char>> = lines
        .clone()
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let oxygen = get_row_by_column_operation(rows.clone(), most_common)?;

    let co2 = get_row_by_column_operation(rows.clone(), least_common)?;
    println!(
        "Life support rating = {}",
        i64::from_str_radix(oxygen.into_iter().collect::<String>().as_str(), 2)?
            * i64::from_str_radix(co2.into_iter().collect::<String>().as_str(), 2)?
    );

    Ok(())
}

pub fn get_gamma_epsilon(lines: Vec<String>) -> Result<(i64, i64)> {
    let columns = input::lines_as_char_columns(lines);
    let (gamma_str, epsilon_str) =
        columns
            .map(most_least_common_bits)
            .fold((String::new(), String::new()), |acc, x| {
                let mut acc = acc;
                acc.0.push(x.0);
                acc.1.push(x.1);
                acc
            });

    Ok((
        i64::from_str_radix(gamma_str.as_str(), 2)?,
        i64::from_str_radix(epsilon_str.as_str(), 2)?,
    ))
}

fn most_least_common_bits(chars: Vec<char>) -> (char, char) {
    let n_ones = chars.iter().filter(|c| **c == '1').count();
    let n_zeroes = chars.iter().filter(|c| **c == '0').count();
    if n_ones >= n_zeroes {
        ('1', '0')
    } else {
        ('0', '1')
    }
}

fn most_common(column_data: Vec<char>) -> char {
    let ones = column_data.iter().filter(|bit| **bit == '1').count();
    let zeroes = column_data.iter().filter(|bit| **bit == '0').count();
    if ones >= zeroes {
        '1'
    } else {
        '0'
    }
}

fn least_common(column_data: Vec<char>) -> char {
    let ones = column_data.iter().filter(|bit| **bit == '1').count();
    let zeroes = column_data.iter().filter(|bit| **bit == '0').count();
    if ones < zeroes {
        '1'
    } else {
        '0'
    }
}

#[test]
fn test_get_gamma_epsilon() {
    let s = r"111
110
101
011";

    let (g, e) = get_gamma_epsilon(input::get_input_lines(s.as_bytes()).unwrap()).unwrap();

    assert_eq!((g, e), (7, 0));
}

type Row = Vec<char>;
type Column = Vec<char>;

fn get_row_by_column_operation(input: Vec<Row>, op: impl Fn(Column) -> char) -> Result<Row> {
    let n_columns = input[0].len();
    let mut input: Vec<Option<Row>> = input.into_iter().map(|x| Some(x)).collect();

    for col in 0..n_columns {
        let column_data: Column = input
            .clone()
            .into_iter()
            .filter_map(|x| x)
            .map(|x| x[col])
            .collect();
        // Find the bit to keep.
        let bit_to_keep = op(column_data);

        // Eliminate rows not starting with correct bit
        for row in 0..input.len() {
            if input.iter().filter(|x| x.is_some()).count() == 1 {
                // Only one candidate - stop.
                break;
            }
            if input[row].is_none() {
                continue;
            }

            let tmp = input[row].clone().unwrap();
            let row_data = tmp.as_slice();
            if row_data[col] != bit_to_keep {
                input[row] = None;
            }
        }
    }

    let output: Vec<Row> = input.into_iter().filter_map(|x| x).collect();
    if output.len() != 1 {
        panic!("too many left = {}", output.len())
    }

    Ok(output[0].clone())
}

#[test]
fn test_somthing() {
    let s = r"111
100
101
001";
    let rows = input::get_input_lines(s.as_bytes())
        .unwrap()
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let result = get_row_by_column_operation(rows, |column_data| {
        // Find the bit to keep.
        let ones = column_data.iter().filter(|bit| **bit == '1').count();
        let zeroes = column_data.iter().filter(|bit| **bit == '0').count();
        if ones >= zeroes {
            '1'
        } else {
            '0'
        }
    })
    .unwrap();

    println!("{}", result.into_iter().collect::<String>());
}
