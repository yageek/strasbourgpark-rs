#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A `Client` allowing to download the location and status of the parking.
 */
typedef struct Client Client;

typedef struct LocationOpenData LocationOpenData;

void strasbourg_park_client_free(Client *client);

int strasbourg_park_client_init(const Client **client);

/**
 * Retrieve the identifier from a location
 */
void strasbourgpark_location_get_id(const LocationOpenData *ptr, const char **id, int *length);
