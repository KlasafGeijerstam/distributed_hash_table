#include <inttypes.h>

uint8_t read_uint8(uint8_t **buffer) {
    uint8_t ret = **buffer;
    *buffer += sizeof(uint8_t);
    return ret;
}

uint16_t read_uint16(uint8_t **buffer) {
    uint16_t ret = *(uint16_t*)*buffer;
    *buffer += sizeof(uint16_t);
    return ret;
}

uint32_t read_uint32(uint8_t **buffer) {
    uint32_t ret = *(uint32_t*)*buffer;
    *buffer += sizeof(uint32_t);
    return ret;
}

void write_uint8(uint8_t **buffer, uint8_t value) {
    **buffer = value;
    *buffer += sizeof(uint8_t);
}

void write_uint16(uint8_t **buffer, uint16_t value) {
    *(uint16_t*)*buffer = value;
    *buffer += sizeof(uint16_t);
}

void write_uint32(uint8_t **buffer, uint32_t value) {
    *(uint32_t*)*buffer = value;
    *buffer += sizeof(uint32_t);
}
