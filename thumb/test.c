int func2() {
    return 2;
}

int main() {
    char * bingus = "SHPONGLEDONGLE";
    long x = (long)bingus + func2();
    return 0;
}