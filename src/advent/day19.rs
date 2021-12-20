use std::collections::HashSet;

use anyhow::Result;

pub fn print_result(mut input: impl std::io::Read) -> Result<()> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let scanners: Vec<_> = buf.split("\n\n").map(Scanner::parse).collect();

    let count = get_total_beacons(scanners)?;

    println!("Number of beacons = {}", count);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<Scanner>> {
    let v: Vec<_> = input.split("\n\n").map(Scanner::parse).collect();
    Ok(v)
}

fn manhattan() {
    let precomputed_pos = r"0,0,0
-63,-1315,-111
-143,-2477,-86
-26,-1217,1093
-94,-2376,-1345
22,-3572,-1159
-106,-3706,-2449
-1221,-3606,-1168
-2464,-3573,-1227
-2540,-2433,-1155
-1224,-4801,-1291
-2544,-4815,-1258
-1333,-3618,-24
-2506,-3612,-2528
-2543,-4769,-88
-3607,-3589,-2467
-2521,-3593,-3700
-2350,-5964,-1282
-2418,-3746,37
-1219,-3654,1060
-4867,-3702,-2531
-3620,-6069,-1317
-1233,-3560,-3625
-3728,-7224,-1252
-2476,-3556,-4761
-2437,-7347,-1231
-3722,-6019,-87
-2434,-7343,40";
    let positions = parse_beacons(precomputed_pos.lines());
    let mut max_dist = 0;
    for i in 0..positions.len() {
        for j in 0..positions.len() {
            let distance = (positions[i][0] - positions[j][0]).abs()
                + (positions[i][1] - positions[j][1]).abs()
                + (positions[i][2] - positions[j][2]).abs();
            max_dist = if distance > max_dist {
                distance
            } else {
                max_dist
            }
        }
    }
    println!("Maxiumum distace = {}", max_dist)
}

fn parse_beacons<'a>(s: impl Iterator<Item = &'a str>) -> Vec<Beacon> {
    s.map(|line| {
        let mut nums = line.split(',').map(|x| x.parse::<i32>().unwrap());
        [
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ]
    })
    .collect::<Vec<_>>()
}

fn get_total_beacons(mut scanners: Vec<Scanner>) -> Result<i32> {
    scanners[0].pos = Some([0, 0, 0]);
    let mut reference_scanners = vec![scanners[0].clone()];
    let mut beacons: HashSet<Beacon> = HashSet::new();
    for &beacon in reference_scanners[0].beacons.iter() {
        beacons.insert(beacon);
    }

    while scanners.iter().filter(|s| s.pos.is_none()).count() > 0 {
        for scanner in scanners.iter_mut().filter(|s| s.pos.is_none()) {
            println!("Attempting to match scanner {}", scanner.id);
            let matched_scanner =
                while_none(reference_scanners.iter(), scanner, |scanner, reference| {
                    if scanner.find_and_update_position(reference, 12) {
                        Some(scanner.clone())
                    } else {
                        None
                    }
                });
            match matched_scanner {
                Some(scanner) => {
                    for &beacon in scanner.beacons.iter() {
                        beacons.insert(beacon);
                    }
                    println!("Matched scanner {}", scanner.id);
                    reference_scanners.push(scanner)
                }
                None => continue,
            }
        }
    }

    for scanner in reference_scanners.iter() {
        let p = scanner.pos.unwrap();
        println!("{},{},{}", p[0], p[1], p[2]);
    }

    Ok(beacons.len() as i32)
}

fn while_none<T, U, R>(
    iter: impl Iterator<Item = T>,
    arg: &mut U,
    f: fn(&mut U, T) -> Option<R>,
) -> Option<R> {
    for val in iter {
        if let Some(v) = f(arg, val) {
            return Some(v);
        }
    }
    None
}

#[derive(Debug, Clone)]
struct Scanner {
    id: i32,
    // relative to reference
    pos: Option<[i32; 3]>,
    // rotation
    rotation: Option<Rotation>,
    // relative to scanner
    beacons: Vec<Beacon>,
}

type Beacon = [i32; 3];

impl Scanner {
    fn new(b: Vec<[i32; 3]>) -> Self {
        Self {
            id: 0,
            pos: None,
            rotation: None,
            beacons: b,
        }
    }

    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let header = lines.next().unwrap();
        let id = header.split(' ').nth(2).unwrap();
        let beacons: Vec<Beacon> = parse_beacons(lines);
        Self {
            id: id.parse().unwrap(),
            beacons: beacons,
            pos: None,
            rotation: None,
        }
    }

    fn find_and_update_position(&mut self, reference: &Scanner, require_matches: usize) -> bool {
        if let Some((pos, rotation)) = self.find_matches(reference, require_matches) {
            self.rebase(pos, rotation);
            true
        } else {
            false
        }
    }

    /// set scanner's position, rotation and update beacons position to be in line with the
    /// provided reference value.
    fn rebase(&mut self, position: [i32; 3], rotation: Rotation) {
        self.pos = Some(position);
        self.rotation = Some(rotation.clone());
        self.beacons.iter_mut().for_each(|p| {
            *p = rotation.rotate_vector_position(*p);
            *p = Self::add(*p, position);
        });
    }

    fn find_matches(
        &self,
        reference: &Scanner,
        minimum_matches: usize,
    ) -> Option<([i32; 3], Rotation)> {
        for (rotation, vectors) in self.rotated_beacons() {
            // We need to compare offsets between positions, since no known reference position exists.
            // Both scanners do not necessarily have the same set of beacons, so we have to try multiple
            // positions as reference point.
            for (reference_offset, reference_beacons) in
                Scanner::position_offsets(reference.beacons.iter().cloned())
            {
                for (offset, beacons) in Scanner::position_offsets(vectors.clone()) {
                    let overlap_count: usize = reference_beacons.iter().fold(0, |acc, p| {
                        if beacons.contains(p) {
                            acc + 1
                        } else {
                            acc
                        }
                    });
                    /*
                    let mut matches = vec![];
                    for rp in reference_beacons.clone() {
                        for p in beacons.clone() {
                            if rp == p {
                                matches.push(p)
                            }
                        }
                    }
                    */
                    if overlap_count >= minimum_matches {
                        let ref_pos = reference_offset;
                        let scan_pos = Self::sub(offset, ref_pos);

                        return Some((scan_pos, rotation));
                    }
                }
            }
        }
        None
    }

    /// rotate the observed beacons in the different possible orientations.
    fn rotated_beacons<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Rotation, impl Iterator<Item = [i32; 3]> + Clone + 'a)> + Clone + 'a
    {
        let mut r = 0usize;
        std::iter::from_fn(move || {
            if let Some(rotation) = Rotation::new(r) {
                r += 1;
                let rot = rotation.clone();
                let points = self
                    .beacons
                    .iter()
                    .map(move |&pos| rot.rotate_vector_position(pos));
                Some((rotation, points))
            } else {
                None
            }
        })
    }

    /// returns an iterator of iterators of position offsets (between the positions).
    /// also includes the reference position used to generate the offset for every iterator.
    fn position_offsets(
        pos: impl Iterator<Item = Beacon> + Clone,
    ) -> impl Iterator<Item = (Beacon, HashSet<Beacon>)> + Clone {
        pos.clone().map(move |r| {
            (
                r,
                pos.clone()
                    .map(move |p| Scanner::sub(r, p))
                    .collect::<HashSet<_>>(),
            )
        })
    }

    fn sub(p1: Beacon, p2: Beacon) -> Beacon {
        [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]]
    }

    fn add(p1: Beacon, p2: Beacon) -> Beacon {
        [p2[0] + p1[0], p2[1] + p1[1], p2[2] + p1[2]]
    }
}

#[derive(Debug, Clone)]
struct Rotation((i32, i32, i32, usize, usize, usize));

impl Rotation {
    fn new(i: usize) -> Option<Self> {
        let o = Self::get_vector_rotation_factors(i)?;
        Some(Self(o))
    }

    fn rotate_vector_position(&self, pos: [i32; 3]) -> [i32; 3] {
        [
            pos[self.0 .3] * self.0 .0,
            pos[self.0 .4] * self.0 .1,
            pos[self.0 .5] * self.0 .2,
        ]
    }

    fn get_vector_rotation_factors(i: usize) -> Option<(i32, i32, i32, usize, usize, usize)> {
        match i {
            i @ 0..=23 => Some(Self::VECTOR_ROTATION[i]),
            _ => return None,
        }
    }

    const VECTOR_ROTATION: [(i32, i32, i32, usize, usize, usize); 24] = [
        (1, 1, 1, 0, 1, 2),
        (1, 1, 1, 1, 2, 0),
        (1, 1, 1, 2, 0, 1),
        (1, 1, -1, 2, 1, 0),
        (1, 1, -1, 1, 0, 2),
        (1, 1, -1, 0, 2, 1),
        (1, -1, -1, 0, 1, 2),
        (1, -1, -1, 1, 2, 0),
        (1, -1, -1, 2, 0, 1),
        (1, -1, 1, 2, 1, 0),
        (1, -1, 1, 1, 0, 2),
        (1, -1, 1, 0, 2, 1),
        (-1, 1, -1, 0, 1, 2),
        (-1, 1, -1, 1, 2, 0),
        (-1, 1, -1, 2, 0, 1),
        (-1, 1, 1, 2, 1, 0),
        (-1, 1, 1, 1, 0, 2),
        (-1, 1, 1, 0, 2, 1),
        (-1, -1, 1, 0, 1, 2),
        (-1, -1, 1, 1, 2, 0),
        (-1, -1, 1, 2, 0, 1),
        (-1, -1, -1, 2, 1, 0),
        (-1, -1, -1, 1, 0, 2),
        (-1, -1, -1, 0, 2, 1),
    ];
}

mod test {
    use super::*;

    #[test]
    fn test_manhattan() {
        manhattan()
    }

    #[test]
    fn test_find_positions_input() {
        let mut scanners = parse(INPUT).expect("parse ok");
        // Set scanner 0 as reference position
        scanners[0].pos = Some([0, 0, 0]);

        let s0 = scanners[0].clone();
        let mut s1 = scanners[1].clone();
        s1.find_and_update_position(&s0, 12);

        let mut s4 = scanners[4].clone();
        s4.find_and_update_position(&s1, 12);

        assert_eq!([68, -1246, -43], s1.pos.expect("s1 has gotten position"));
        assert_eq!([-20, -1133, 1061], s4.pos.expect("s4 has gotten position"));
    }

    #[test]
    fn test_find_position() {
        /*
         2   B
         1     B
         0 S
        -1
        -2 S
           0 1 2 3 4
         */
        let mut s1 = Scanner::new(vec![[3, 3, 3], [2, 1, 0], [1, 2, 0]]);
        s1.pos = Some([0, 0, 0]);
        let mut s2 = Scanner::new(vec![[1, 4, 0], [7, 6, 5], [2, 3, 0]]);
        let matches = s2.find_and_update_position(&s1, 2);

        assert_eq!(s2.pos.expect("position set"), [0, -2, 0]);
        assert_eq!(matches, true);
        assert_eq!(s2.beacons.as_slice(), &[[1, 2, 0], [7, 4, 5], [2, 1, 0]])
    }

    #[test]
    fn test_input() {
        let scanners = parse(INPUT).expect("parse ok");
        let beacon_count = get_total_beacons(scanners).expect("beacon count");

        assert_eq!(beacon_count, 79);
    }

    const INPUT: &str = r"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";
}
