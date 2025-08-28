#include <stdio.h>
#include <stdlib.h>

int main() {
    // Manual memory management in C
    int* numbers = malloc(5 * sizeof(int));  // Allocate memory
    
    // Use the memory
    for (int i = 0; i < 5; i++) {
        numbers[i] = i * 10;
        printf("numbers[%d] = %d\n", i, numbers[i]);
    }
    
    // Must manually free memory to avoid memory leaks
    free(numbers);  // Manual deallocation required!
    
    // If we forget to free, we get a memory leak
    // numbers = malloc(10 * sizeof(int));  // This would leak if we don't free first
    
    return 0;
} 