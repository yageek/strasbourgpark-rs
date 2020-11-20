#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

typedef struct SPLocationOpenData SPLocationOpenData;

void strasbourgpark_location_get_id(SPLocationOpenData *location, char **buff, size_t length);