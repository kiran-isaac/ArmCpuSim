extern int func2();

int main() {
    char * bingus = "str";
    long x = (long)bingus + func2();
    return 0;
}