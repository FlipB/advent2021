use std::ops::Add;

use super::input;
use anyhow::Result;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let mut bits = bytes_into_bits(&input::get_input_hex(input)?);
    let (remain_bits, val, ver_sum) = parse_packet(&mut bits);
    println!(
        "ver sum = {}, val = {}, remain = {}",
        ver_sum,
        val,
        remain_bits.len()
    );

    Ok(())
}

pub fn get_input_hex(buf: &str) -> Result<Vec<u8>> {
    let v: Result<Vec<_>, std::num::ParseIntError> = (0..buf.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&buf[i..i + 2], 16))
        .collect();

    v.map_err(|_e| anyhow::Error::msg("bad input"))
}

fn parse_packet(bits: &[u8]) -> (&[u8], u64, Version) {
    let (payload, v, t) = parse_header(bits);
    if t == 4 {
        let (remain, parts) = parse_literal(payload);
        let combined = parts
            .iter()
            .flat_map(|&bits| bits.iter().cloned())
            .collect::<Vec<_>>();
        (remain, bits_into_u32(&combined), v)
    } else {
        let (remain, val, pkts) = parse_operator(t, payload);
        let mut sum_of_pkt_verions = v;
        for ver in pkts {
            sum_of_pkt_verions += ver
        }
        (remain, val, sum_of_pkt_verions)
    }
}

type Version = u64;
type Op = u64;

fn parse_header(bits: &[u8]) -> (&[u8], Version, Op) {
    (
        &bits[6..],
        bits_into_u32(&bits[0..3]),
        bits_into_u32(&bits[3..6]),
    )
}

fn parse_literal(mut bits: &[u8]) -> (&[u8], Vec<&[u8]>) {
    let mut parts = vec![];
    while bits[0] == 1 {
        parts.push(&bits[1..5]);
        bits = &bits[5..];
    }
    parts.push(&bits[1..5]);
    bits = &bits[5..];
    (bits, parts)
}

fn parse_operator(op: Op, mut bits: &[u8]) -> (&[u8], u64, Vec<Version>) {
    let (remainder, values, pkts) = {
        let length_type_id = bits[0];
        bits = &bits[1..];
        match length_type_id {
            0 => {
                let pkts_len = bits_into_u32(&bits[0..15]) as usize;
                bits = &bits[15..];
                let mut pkts_bits = &bits[0..pkts_len];
                let remain = &bits[pkts_len..];
                let mut pkts = vec![];
                let mut values = vec![];
                while !pkts_bits.is_empty() {
                    let pkt = parse_packet(pkts_bits);
                    pkts_bits = pkt.0;
                    pkts.push(pkt.2);
                    values.push(pkt.1)
                }
                (remain, values, pkts)
            }
            1 => {
                let num_pkts = bits_into_u32(&bits[0..11]) as usize;
                let mut pkts_bits = &bits[11..];
                let mut pkts = vec![];
                let mut values = vec![];
                for _ in 0..num_pkts {
                    let pkt = parse_packet(pkts_bits);
                    pkts_bits = pkt.0;
                    pkts.push(pkt.2);
                    values.push(pkt.1)
                }
                (pkts_bits, values, pkts)
            }
            _ => unreachable!(),
        }
    };
    (remainder, do_op(op, &values), pkts)
}

fn do_op(op: Op, val: &[u64]) -> u64 {
    match op {
        0 => val.iter().sum(),
        1 => val[1..].iter().fold(val[0], |acc, x| acc * x),
        2 => *val.iter().min().unwrap(),
        3 => *val.iter().max().unwrap(),
        4 => val[0], // shouldn't end up here...
        5 => {
            if val[0] > val[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if val[0] < val[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if val[0] == val[1] {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}

fn bits_into_u32(bits: &[u8]) -> u64 {
    let mut v = 0u64;
    for i in 0..bits.len() {
        let bit = bits[bits.len() - 1 - i];
        v = v | ((bit as u64) << i);
    }
    v
}

fn bytes_into_bits(bytes: &[u8]) -> Vec<u8> {
    bytes
        .into_iter()
        .map(|&byte| {
            let mut bits = [0u8; 8];
            for i in 0..8 {
                let bit = (byte >> (7 - i)) & 0b1;
                bits[i] = bit;
            }
            bits
        })
        .flatten()
        .collect()
}

#[test]
fn test_bytes_into_bits() {
    let bytes = vec![3u8];
    let bits = bytes_into_bits(&bytes);
    assert_eq!(&bits, &[0, 0, 0, 0, 0, 0, 1, 1]);

    let bytes = vec![255u8];
    let bits = bytes_into_bits(&bytes);
    assert_eq!(&bits, &[1, 1, 1, 1, 1, 1, 1, 1]);

    let bytes = vec![2u8, 255u8];
    let bits = bytes_into_bits(&bytes);
    assert_eq!(&bits, &[0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_bits_into_u32() {
    assert_eq!(3u64, bits_into_u32(&[0, 0, 0, 0, 0, 0, 1, 1]));
    assert_eq!(255u64, bits_into_u32(&[1, 1, 1, 1, 1, 1, 1, 1]));
    assert_eq!(255u64, bits_into_u32(&[1, 1, 1, 1, 1, 1, 1, 1]));
    assert_eq!(
        512 + 255,
        bits_into_u32(&[0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1])
    );
}
