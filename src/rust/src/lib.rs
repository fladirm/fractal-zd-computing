//! Developer reference for the exact K16 7-color root, five probes,
//! factorized A0 ⊗ J_m and true O(m²) reference. No external crates.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Probe { Zd, NonZd, Commutator, Associator, Alternator }
impl Probe {
    pub const ALL: [Probe; 5] = [Probe::Zd, Probe::NonZd, Probe::Commutator, Probe::Associator, Probe::Alternator];
    pub fn name(self) -> &'static str { match self {
        Probe::Zd=>"zd", Probe::NonZd=>"non-zd",Probe::Commutator=>"commutator",Probe::Associator=>"associator",Probe::Alternator=>"alternator"
    }}
}
pub fn epsilon(n:usize, a:usize,b:usize)->i32 {
    assert!(n.is_power_of_two() && a<n && b<n);
    if n==1 {return 1}
    let h=n/2;
    if a<h && b<h {epsilon(h,a,b)}
    else if a<h {epsilon(h,b-h,a)}
    else if b<h {(if b==0 {1}else{-1})*epsilon(h,a-h,b)}
    else {(if b==h {-1}else{1})*epsilon(h,b-h,a-h)}
}
fn product(x:&[(usize,i32)],y:&[(usize,i32)])->[i32;16] {
    let mut out=[0;16];
    for &(a,sa) in x {for &(b,sb) in y {out[a^b]+=sa*sb*epsilon(16,a,b)}}
    out
}
fn is_nonzero(x:&[i32;16])->bool{x.iter().any(|v|*v!=0)}
fn subtract(x:[i32;16],y:[i32;16])->[i32;16] {
    std::array::from_fn(|i|x[i]-y[i])
}
fn sparse(x:&[i32;16])->Vec<(usize,i32)> {x.iter().enumerate().filter(|&(_,v)|*v!=0).map(|(i,v)|(i,*v)).collect()}
pub fn plaque(p:(usize,usize),q:(usize,usize))->bool {
    let (a,b)=p;let(c,d)=q;
    if a==0||a>=b||b>=16||c==0||c>=d||d>=16||(a^b)!=(c^d) {return false}
    if a==c||a==d||b==c||b==d {return false}
    epsilon(16,a,c)*epsilon(16,b,d)*epsilon(16,a,d)*epsilon(16,b,c)==1
}
pub fn probe(p:(usize,usize),q:(usize,usize),channel:Probe)->bool {
    let x=[(p.0,1),(p.1,1)];let y=[(q.0,1),(q.1,1)];
    match channel {
        Probe::Zd=>plaque(p,q),Probe::NonZd=>!plaque(p,q),
        Probe::Commutator=>is_nonzero(&subtract(product(&x,&y),product(&y,&x))),
        Probe::Associator=>{let z=[(1,1)];
            is_nonzero(&subtract(product(&sparse(&product(&x,&z)),&y),product(&x,&sparse(&product(&z,&y)))))
        },
        Probe::Alternator=>is_nonzero(&subtract(product(&sparse(&product(&x,&x)),&y),product(&x,&sparse(&product(&x,&y)))))
    }
}
pub fn roots()->Vec<(usize,Vec<(usize,usize)>)> {
    let mut roots=Vec::new();
    for chi in 9..=15 {
        let cand:Vec<(usize,usize)>=(1..16).filter_map(|a|{let b=a^chi;if a<b&&b<16{Some((a,b))}else{None}}).collect();
        let active:Vec<_>=cand.iter().copied().filter(|&p|cand.iter().any(|&q|plaque(p,q))).collect();
        let mut seen=vec![false;active.len()];let mut ordered=Vec::new();
        for i in 0..active.len(){if seen[i]{continue}
            let j=(0..active.len()).find(|&j|j!=i&&!seen[j]&&!plaque(active[i],active[j])).expect("K222 pairing");
            ordered.push(active[i]);ordered.push(active[j]);seen[i]=true;seen[j]=true;
        }
        assert_eq!(ordered.len(),6);
        roots.push((chi,ordered));
    }
    roots
}
pub type RootMask=[[u8;6];42];
pub fn adjacency(channel:Probe)->RootMask {
    let mut root=[[0u8;6];42];
    for (r,(_,pairs)) in roots().iter().enumerate(){
        for i in 0..6{for j in i+1..6{if probe(pairs[i],pairs[j],channel){root[r*6+i][j]=1;root[r*6+j][i]=1;}}}
    }
    root
}
pub fn edge_count(root:&RootMask)->usize {root.iter().map(|r|r.iter().map(|&x|x as usize).sum::<usize>()).sum::<usize>()/2}
pub fn e_count(n:u128)->u128 {if n<16 {0} else {(n-2)*(n-4)*(n-8)/16}}
pub fn birth(n:u128)->u128 {e_count(n)-4*e_count(n/2)}
pub fn q_moment(n:u128)->f64{32.0*(e_count(n) as f64)/((n as f64).powi(2)*((n+2) as f64).powi(2))}

#[derive(Clone,Copy,Debug,Default)]
pub struct LocalDelta {pub enabled:bool,pub region:usize,pub source:usize,pub dest:usize,pub weight:f32}
fn patch(y:&mut[f32],x:&[f32],m:usize,delta:LocalDelta){if delta.enabled {
    assert!(delta.region<7&&delta.source<6&&delta.dest<6);
    y[(delta.region*6+delta.dest)*m]+=delta.weight*x[(delta.region*6+delta.source)*m];
}}
/// O(42*m + 7*6²), working memory O(42*m); root + sparse delta model.
pub fn apply_factor(x:&[f32],m:usize,root:&RootMask,delta:LocalDelta)->Vec<f32>{
    assert!(m>0&&x.len()==42*m);
    let mut sums=[0.0f64;42];let mut y=vec![0.0f32;x.len()];
    for g in 0..42 {for i in 0..m {sums[g]+=x[g*m+i] as f64}}
    for g in 0..42 {let base=(g/6)*6;
        let v=(0..6).map(|j|root[g][j] as f64*sums[base+j]).sum::<f64>()/(4*m) as f64;
        y[g*m..(g+1)*m].fill(v as f32);
    }
    patch(&mut y,x,m,delta);y
}
/// True O(m²) reference: one loop iteration per replicated root edge, m≤128.
pub fn apply_explicit(x:&[f32],m:usize,root:&RootMask,delta:LocalDelta)->Vec<f32>{
    assert!(m>0&&m<=128&&x.len()==42*m);
    let mut y=vec![0.0f32;x.len()];
    for r in 0..7{for a in 0..6{for t in 0..m{
        let mut s=0.0f64;
        for b in 0..6{if root[r*6+a][b]!=0{for donor in 0..m{s+=x[(r*6+b)*m+donor] as f64}}}
        y[(r*6+a)*m+t]=(s/(4*m) as f64) as f32;
    }}}patch(&mut y,x,m,delta);y
}
pub fn max_deviation(a:&[f32],b:&[f32])->f32 {assert_eq!(a.len(),b.len());a.iter().zip(b).map(|(x,y)|(x-y).abs()).fold(0.0,f32::max)}

#[cfg(test)] mod tests {use super::*;
 #[test]fn roots_and_probes(){let roots=roots();assert_eq!(roots.len(),7);for probe in Probe::ALL {
   let mask=adjacency(probe);
   assert!(edge_count(&mask)>0);
 }assert_eq!(edge_count(&adjacency(Probe::Zd)),84);assert_eq!(edge_count(&adjacency(Probe::NonZd)),21);
 }
 #[test]fn moments(){assert_eq!(e_count(16),84);assert_eq!(e_count(32),1260);assert_eq!(birth(32),924);assert!((q_moment(16)-7.0/216.0).abs()<1e-12);}
 #[test]fn factors(){for probe in Probe::ALL{let mask=adjacency(probe);for m in [1usize,2,4,8,16,32]{
    let x:Vec<f32>=(0..42*m).map(|i|((i*17%31) as f32)/31.0).collect();
    for delta in [LocalDelta::default(),LocalDelta{enabled:true,region:2,source:1,dest:4,weight:0.75}]{
      let a=apply_explicit(&x,m,&mask,delta);let b=apply_factor(&x,m,&mask,delta);
      assert!(max_deviation(&a,&b)<1e-5,"{} m={}",probe.name(),m);
    }
 }}}
 #[test]fn signed_witness_and_associator(){
    let x=[(2,1),(11,1)];let y=[(4,1),(13,-1)];
    assert_eq!(product(&x,&y),[0;16]);assert_eq!(product(&y,&x),[0;16]);
    for (key,expected) in [(1,vec![(7,-4)]),(8,vec![])] {
      let z=[(key,1)];
      assert_eq!(sparse(&subtract(product(&sparse(&product(&x,&z)),&y),product(&x,&sparse(&product(&z,&y))))),expected);
    }
 }
 #[test]fn census_and_signed_lifts(){
    let mul=|n:usize,x:&[(usize,i32)],y:&[(usize,i32)]|{
      let mut out=vec![0;n];for &(a,sa) in x {for &(b,sb) in y {out[a^b]+=sa*sb*epsilon(n,a,b)}}out
    };
    for n in [16usize,32,64] {
      let mut count=0;
      for chi in 1..n {
        let pairs:Vec<_>=(1..n).filter_map(|a|{let b=a^chi;if a<b {Some((a,b))}else{None}}).collect();
        for (i,&(a,b)) in pairs.iter().enumerate(){for &(c,d) in &pairs[i+1..]{
          let qualifies=epsilon(n,a,c)*epsilon(n,b,d)*epsilon(n,a,d)*epsilon(n,b,c)==1;
          let tau=-epsilon(n,a,c)*epsilon(n,b,d);
          assert_eq!(mul(n,&[(a,1),(b,1)],&[(c,1),(d,tau)]).iter().all(|&v|v==0),qualifies);
          count+=usize::from(qualifies);
        }}
      }
      assert_eq!(count as u128,e_count(n as u128));
    }
    for i in [0,16] {for j in [0,16] {
      let x=[(2+i,1),(11+i,1)];let y=[(4+j,1),(13+j,-1)];
      assert_eq!(mul(32,&x,&y),vec![0;32]);assert_eq!(mul(32,&y,&x),vec![0;32]);
    }}
    assert_eq!(e_count(2048),533_208_060);
    assert!(!plaque((2,11),(4,12)));assert!(!plaque((2,2),(4,13)));
 }
}
