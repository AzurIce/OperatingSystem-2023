
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
