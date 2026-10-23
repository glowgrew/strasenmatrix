use std::time::Instant;

use strasen::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut algoritmy = vec![("classic", 0)];
    let razmery: Vec<usize>;

    if args.len() == 2 && args[1] == "thresholds" {
        for t in [16, 32, 64, 96, 128, 192, 256, 512] {
            algoritmy.push(("strasen_OPTIMIEZED", t));
        }
        razmery = (100..=2000).step_by(50).collect();
    } else if args.len()==3 && args[1] == "memory" {
        let porog: usize = args[2].parse().unwrap();
        for t in [16, porog] {
            algoritmy.push(("strasen_OPTIMIEZED", t));
            algoritmy.push(("strasen_DEFOLT", t));
        }
        razmery = (200..=2000).step_by(200).collect();
    } else {
        eprintln!("usage: strasen thresholds > results/thresholds.csv");
        eprintln!("       strasen memory THRESHOLD > results/memory.csv");
        return;
    }

    println!("algo,threshold,n,ms");
    for n in razmery {
        let a = rand_matr(n, 1);
        let b = rand_matr(n, 2);
        let mut pravilno = vec![0.0; n * n];
        let mut c = vec![0.0; n*n];
        classicMultiply(&a, &b, &mut pravilno, n);

        let mut best = vec![f64::MAX; algoritmy.len()];
        let povtorov = if n <= 500 { 20 } else { 5 };
        for r in 0..povtorov {
            for k in 0..algoritmy.len() {
                let (algo, t) = algoritmy[k];
                let start = Instant::now();
                match algo {
                    "classic" => classicMultiply(&a, &b, &mut c, n),
                    "strasen_OPTIMIEZED" => strasen_OPTIMIEZED(&a, &b, &mut c, n, t),
                    _ => strasen_DEFOLT(&a, &b, &mut c, n, t),
                }
                let ms = start.elapsed().as_secs_f64()*1000.0;
                if ms < best[k] {
                    best[k] = ms;
                }
                if r == 0 && maxDiff(&c, &pravilno) > 1e-9 * n as f64 {
                    panic!("wrong result: {algo} threshold={t} n={n}");
                }
            }
        }

        for k in 0..algoritmy.len() {
            let (algo, t) = algoritmy[k];
            println!("{},{},{},{:.3}", algo, t, n, best[k]);
        }
        eprintln!("n = {n} done");
    }
}
