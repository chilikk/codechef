import sys,time,math
primes=[2,3]
def next(pointer, primes):
  pointer+=1
  if pointer<len(primes):
    return (pointer,primes[pointer])
  i = primes[-1]+2
  while True:
    prime=True
    isqrt = math.sqrt(i)
    for p in primes:
      if i%p==0:
        prime = False
        break
      if p>isqrt:
        break
    if prime:
      primes.append(i)
      return (pointer,i)
    i+=2
  return (pointer-1,False)
pointer=0
prime=2
const=2*10**19+1
while True:
    pointerstart=pointer
    start=time.time()
    i=0
    while i<100000:
        pointer,prime=next(pointer,primes)
        i+=1
    end=time.time()
    pointerend=pointer
    print "prime %d time %s" % (prime, end-start)
    i=pointerstart
    start=time.time()
    while i<=pointerend:
        const%primes[i]
        i+=1
    end=time.time()
    print "prime scan time %s" % (end-start)
    i=primes[pointerstart]
    start=time.time()
    while i<=primes[pointerend]:
        const%i
        i+=1
    end=time.time()
    print "scan time %s" % (end-start)
