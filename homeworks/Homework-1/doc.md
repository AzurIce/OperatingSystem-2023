# 报告

> 21301021 肖斌

## 一、Pthread

代码：[./pthreads.cpp](./pthreads.cpp)

内容如下：

```cpp
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
```

`bool isPrime(int x)` 用于判断 `x` 是否为质数，
`void* printPrime(void* arg)` 为线程函数
`struct ThreadArgs` 为线程参数，包含最大值信息。

## 二、Java

代码： [./PrimeThread.java](./PrimeThread.java)

内容如下：

```java
public class PrimeThread implements Runnable {
    boolean isPrime(int x) {
        for (int i = 2; i * i <= x; i++) {
            if (x % i == 0) return false;
        }
        return true;
    }

    private int m;

    public PrimeThread(int m) {
        this.m = m;
    }

    public void run() {
        for (int i = 2; i <= m; i++) {
            if (isPrime(i)) System.out.println(i);
        }
        System.out.println();
    }

    public static void main(String[] args) {
        java.util.Scanner scanner = new java.util.Scanner(System.in);
        int x = scanner.nextInt();

        PrimeThread primeThread = new PrimeThread(x);
        Thread t = new Thread(primeThread);
        t.start();
    }
}

```

通过实现了 `Runnable` 接口的 `PrimeThread` 类来创建 `Thread`。