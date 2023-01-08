import sys

def read_coord():
    x, y, d = sys.stdin.readline().rstrip().split()
    dx, dy = {'U' : (0., 1.),
              'D' : (0., -1.),
              'L' : (-1., 0.),
              'R' : (1., 0.)}[d]
    return float(x), float(y), dx, dy

def read_int():
    return int(sys.stdin.readline().rstrip())

testcases = read_int()
for _ in range(0, testcases):
    xe, ye, dxe, dye = read_coord()
    collision = None
    asteroids = read_int()
    for _ in range(0, asteroids):
        xa, ya, dxa, dya = read_coord()
        dxa -= dxe
        dya -= dye
        candidate = None
        if dxa != 0.:
            t = (xe-xa)/dxa
            if ya + t*dya == ye:
                candidate = t
        elif dya != 0.:
            t = (ye-ya)/dya
            if xa + t*dxa == xe:
                candidate = t
        if candidate is not None and candidate > 0:
            if collision is None or collision > candidate:
                collision = candidate
    if collision is None:
        print("SAFE")
    else:
        print("%.1f" % collision)
