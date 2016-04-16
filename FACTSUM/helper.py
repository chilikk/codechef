import sys
primes=[2]
sys.stdout.write("[2")
for i in range(3,86000):
    prime=True
    for p in primes:
        if i%p==0:
            prime = False
            break
        if p**2>i:
            break
    if prime:
        primes.append(i)
        sys.stdout.write(",%d" % i)
sys.stdout.write("]")

#sys.stdout.write(base64.b64encode(primes.tostring()))
