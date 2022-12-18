#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

use util::*;

type N = i32;

type In = [N; 3];
type Out = usize;

fn parse(s: &'static str) -> In {
    s.split(',').map(p).collect::<Vec<_>>().try_into().unwrap()
}

fn surface_area(cubes: &[[N; 3]]) -> usize {
    let mut area = cubes.len() * 6;

    for i in 0..cubes.len() {
        for j in (i + 1)..cubes.len() {
            assert_ne!(cubes[i], cubes[j]);

            let a = cubes[i];
            let b = cubes[j];

            for k0 in 0..3 {
                let k1 = (k0 + 1) % 3;
                let k2 = (k0 + 2) % 3;

                if a[k0] == b[k0] && a[k1] == b[k1] && a[k2].abs_diff(b[k2]) == 1 {
                    area -= 2;
                    break;
                }
            }
        }
    }

    area
}

fn part1(n: &[In]) -> Out {
    surface_area(n)
}

fn part2(n: &[In]) -> Out {
    let min = -1;
    let max = 22;

    for v in n.iter().flatten() {
        assert!(min < *v);
        assert!(max > *v);
    }

    let mut cubes = n.iter().copied().collect::<HashSet<_>>();

    let mut flood = HashSet::new();

    for a in min..=max {
        for b in min..=max {
            flood.extend([
                [a, b, min],
                [a, b, max],
                [min, a, b],
                [max, a, b],
                [a, min, b],
                [a, max, b],
            ]);
        }
    }

    loop {
        let mut new_gen = HashSet::new();

        for &[x, y, z] in &flood {
            for [dx, dy, dz] in [
                [1, 0, 0],
                [-1, 0, 0],
                [0, 1, 0],
                [0, -1, 0],
                [0, 0, 1],
                [0, 0, -1],
            ] {
                let r = min..=max;
                let (x1, y1, z1) = (x + dx, y + dy, z + dz);
                if !r.contains(&x1) || !r.contains(&y1) || !r.contains(&z1) {
                    continue;
                }

                let point = [x1, y1, z1];
                if flood.contains(&point) || cubes.contains(&point) {
                    continue;
                }
                new_gen.insert(point);
            }
        }

        if new_gen.is_empty() {
            break;
        } else {
            flood.extend(new_gen);
        }
    }

    // fill in the holes, anything the flood fill didn't reach
    for x in min..=max {
        for y in min..=max {
            for z in min..=max {
                let p = [x, y, z];
                if !(flood.contains(&p) || cubes.contains(&p)) {
                    cubes.insert(p);
                }
            }
        }
    }

    surface_area(&Vec::from_iter(cubes))
}

util::register!(parse, part1, part2);
