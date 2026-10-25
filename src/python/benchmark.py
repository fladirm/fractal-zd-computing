#!/usr/bin/env python3
import argparse,csv,time,random

def inherited(a,x):
    s=[sum(row) for row in x]
    v=[sum(a[i][j]*s[j] for j in range(len(a))) for i in range(len(a))]
    return [[v[i]]*len(x[i]) for i in range(len(a))]

def explicit(a,x):
    r=len(a);m=len(x[0]); out=[[0]*m for _ in range(r)]
    for i in range(r):
        for u in range(m):
            out[i][u]=sum(a[i][j]*x[j][v] for j in range(r) for v in range(m))
    return out

def main():
    pa=argparse.ArgumentParser();pa.add_argument('--output',default='benchmark.csv');pa.add_argument('--repeat',type=int,default=3);args=pa.parse_args()
    a=[[0 if i==j or i//2==j//2 else 1 for j in range(6)] for i in range(6)]
    rng=random.Random(20261025)
    rows=[]
    for m in [1,2,4,8,16,32,64,128]:
        x=[[rng.randrange(-100,101) for _ in range(m)] for _ in range(6)]
        assert explicit(a,x)==inherited(a,x)
        for impl,fn in [('explicit',explicit),('factorized',inherited)]:
            times=[]
            for _ in range(args.repeat):
                t=time.perf_counter_ns();y=fn(a,x);times.append(time.perf_counter_ns()-t)
            rows.append(dict(m=m,root=6,method=impl,ns_median=sorted(times)[len(times)//2],dense_entries=(6*m)**2,root_entries=36,checksum=sum(map(sum,y))))
    with open(args.output,'w',newline='') as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
    print('PASS exact operator equality for all m; output:',args.output)
if __name__=='__main__': main()
