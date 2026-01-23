#include <stdio.h>

enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
};

int main()
{
    enum WEEK today;
    today = Sunday;
    printf("%d\n", today);
    today = 22;
    printf("%d\n", today);
    return 0;
}