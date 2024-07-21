## Hello world in C

```c
#include <stdio.h>

int
main() {
    printf("Hello, World!\n");
    return 0;
}
```

## how to run it?

To run a C program, follow these steps:

1. **Save the Code**: Save the code in a file with a `.c` extension, e.g., `hello.c`.

2. **Open a Terminal**: Open a terminal or command prompt.

3. **Compile the Code**: Use a C compiler like `gcc` to compile the code. You can run the following command:
   ```bash
   gcc hello.c -o hello
   ```
   This will create an executable file named `hello`.

4. **Run the Executable**: Execute the compiled program with:
   ```bash
   ./hello
   ```
   You should see the output:
   ```
   Hello, World!
   ```

> Model used: gpt-4o-mini
