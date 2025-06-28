#include <iostream>
#include <chrono>

int main()
{
    int NUM_ITERATIONS = 1000000000;
    int counter = 0;

    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < NUM_ITERATIONS; ++i)
    {
        counter++;
    }
    // int i = 0;
    // while (i < NUM_ITERATIONS)
    // {
    //     counter++;
    //     i++;
    // }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Counter: " << counter << std::endl;
    std::cout << "Time taken: " << elapsed.count() << " seconds" << std::endl;

    return 0;
}