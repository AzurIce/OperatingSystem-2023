#include <iostream>
#include <pthread.h>
#include <cmath>

using namespace std;

bool isPrime(int x) {
    for (int i = 2; i <= sqrt(x); i++) {
        if (x % i == 0) return false;
    }
    return true;
}

struct ThreadArgs {
    int m;
};

void *printPrime(void* arg) {
    ThreadArgs* args = static_cast<ThreadArgs*>(arg);
    for (int i = 2; i <= args->m; i++) {
        if (isPrime(i)) cout << i << " ";
    }
    cout << endl;

    delete args;
    pthread_exit(nullptr);
}

int main() {
    int x; cin >> x;

    ThreadArgs* args = new ThreadArgs;
    args->m = x;

    pthread_t t;
    int res = pthread_create(&t, nullptr, printPrime, static_cast<void*>(args));

    if (res) {
        cerr << "failed to create thread: " << res << endl;
        return 1;
    }

    pthread_join(t, nullptr);

    return 0;
}
