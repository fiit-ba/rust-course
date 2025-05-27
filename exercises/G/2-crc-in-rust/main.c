#include <inttypes.h> // uint32_t, uint8_t
#include <stddef.h> // size_t
#include <stdio.h> // printf
#include "crc_in_rust.h"

int main() { 
    uint8_t data[] = { 0,1,2,3,4,5,6 };
    size_t data_length = 7;

    uint32_t hash = crc32(data, data_length);

    printf("Hash: 0x%d\n", hash);

    return 0;
}
