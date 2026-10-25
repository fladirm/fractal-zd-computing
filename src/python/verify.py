"""Independent exact oracle for signed CD incidence and Higher-pi rational invariants."""
from functools import lru_cache
from fractions import Fraction
from math import comb

@lru_cache(None)
def eps(n,a,b):
    assert n >= 1 and n & (n-1) == 0 and 0 <= a < n and 0 <= b < n
    if n == 1: return 1
    h = n//2
    if a < h and b < h: return eps(h,a,b)
    if a < h: return eps(h,b-h,a)
    if b < h: return eps(h,a-h,b) * (1 if b == 0 else -1)
    return eps(h,b-h,a-h) * (-1 if b == h else 1)

def edges(n):
    count = 0
    candidates=0
    for chi in range(1,n):
        A = [(a,a^chi) for a in range(1,n) if (a^chi)>a and (a^chi)<n]
        for i,(a,b) in enumerate(A):
            for c,d in A[i+1:]:
                if len({a,b,c,d}) != 4: continue
                candidates += 1
                count += eps(n,a,c)*eps(n,b,d)*eps(n,a,d)*eps(n,b,c) == 1
    return count,candidates

def e_formula(n): return 0 if n<16 else (n-2)*(n-4)*(n-8)//16
def q(n): return Fraction(32*e_formula(n),n*n*(n+2)**2)
def q_phase(n): return Fraction(32*e_formula(n),(n-1)**2*(n+1)**2)
def c(n): return comb(n,n//2)*(1+q(n))**2/(1+q(2*n))

def mul(n,x,y):
    z={}
    for a,sa in x:
        for b,sb in y:
            idx=a^b
            z[idx]=z.get(idx,0)+sa*sb*eps(n,a,b)
    return {k:v for k,v in z.items() if v}

def run():
    assert mul(16,[(2,1),(11,1)],[(4,1),(13,-1)]) == {}
    x,y=[(2,1),(11,1)],[(4,1),(13,-1)]
    assert mul(16,y,x) == {}
    for key,expected in ((1,{7:-4}),(8,{})):
        left=mul(16,list(mul(16,x,[(key,1)]).items()),y)
        right=mul(16,x,list(mul(16,[(key,1)],y).items()))
        difference={i:left.get(i,0)-right.get(i,0) for i in left.keys()|right.keys()}
        assert {i:v for i,v in difference.items() if v} == expected
    assert e_formula(2048) == 533_208_060
    for i in [0,16]:
        for j in [0,16]:
            assert mul(32,[(2+i,1),(11+i,1)],[(4+j,1),(13+j,-1)]) == {}
            assert mul(32,[(4+j,1),(13+j,-1)],[(2+i,1),(11+i,1)]) == {}
    for n in (16,32,64):
        val,candidates=edges(n)
        assert val == e_formula(n)
        print(f'K{n}: ZD={val}, candidates={candidates}, PASS')
    assert edges(16)==(84,315)
    assert q(16)==Fraction(7,216)
    assert q(32)==Fraction(315,9248)
    assert q_phase(16)==Fraction(896,21675)
    assert c(8)==Fraction(15120,223)
    assert c(16)==Fraction(10275751915,774603)
    print('Signed witness, associators, bidirectional 4-lift, K2048 census and Higher-pi fractions: PASS')

if __name__=='__main__':run()
