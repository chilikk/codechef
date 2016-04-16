#include <stdio.h>

int f_max(int x) {
    return (x+2)*(x+1)/2-1;
}

int f_min(int x) {
    if (x==0) return 0;
    int x2 = x/2;
    return x+1+f_min(x2)+f_min(x-x2-1);
}

void do_test() {
    int n,m;
    scanf("%i %i",&n,&m);
    int nmax = f_max(n);
    if (nmax < m) {
        printf("%i\n",m-nmax);
    } else {
        int nmin=f_min(n);
        if (nmin > m) {
            printf("-1\n");
        } else {
            printf("0\n");
        }
    }
}

int test(int t) {
    if (t>0) {
        do_test();
        return test(t-1);
    }
    return 0;
}

int main(void) {
    int t;
    scanf("%i",&t);
    return test(t);
}
