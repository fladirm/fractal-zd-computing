use zd_dev_lab::{Probe,adjacency,edge_count,apply_factor,apply_explicit,max_deviation,LocalDelta};
use std::time::Instant;
fn main(){let t=std::env::args().nth(1).and_then(|s|s.parse::<u32>().ok()).unwrap_or(5).min(15);
    let m=1usize<<t;let n=42*m;
    let x:Vec<f32>=(0..n).map(|i|0.15+((i*17%37)as f32)/100.0).collect();
    println!("Fractal Zero-Divisor Computing / t={} / m={}",t,m);
    for p in Probe::ALL {
        let root=adjacency(p);let e=edge_count(&root)as u64*(m as u64)*(m as u64);
        let now=Instant::now();let y=apply_factor(&x,m,&root,LocalDelta::default());let factor=now.elapsed();
        print!("{:11} {:>14} inherited edges | factor {:?}",p.name(),e,factor);
        if m<=128 {let start=Instant::now();let z=apply_explicit(&x,m,&root,LocalDelta::default());print!(" | explicit {:?} | max error {:.4e}",start.elapsed(),max_deviation(&y,&z));}
        println!();
    }
}
