#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A `Client` allowing to download the location and status of the parking.
 */
typedef struct SPClient SPClient;

typedef struct SPLocationOpenData SPLocationOpenData;

typedef struct {
  double lat;
  double lng;
} SPCoordinate;

/**
 *Retrieve the `id` as const char.
 */
void strasbourgpark_location_get_id(const SPLocationOpenData *ptr, const char **id, int *length);

/**
 *Retrieve the `city` as const char.
 */
void strasbourgpark_location_get_city(const SPLocationOpenData *ptr,
                                      const char **city,
                                      int *length);

/**
 *Retrieve the `zipcode` as const char.
 */
void strasbourgpark_location_get_zipcode(const SPLocationOpenData *ptr,
                                         const char **zipcode,
                                         int *length);

/**
 *Retrieve the `street` as const char.
 */
void strasbourgpark_location_get_street(const SPLocationOpenData *ptr,
                                        const char **street,
                                        int *length);

/**
 *Retrieve the `address` as const char.
 */
void strasbourgpark_location_get_address(const SPLocationOpenData *ptr,
                                         const char **address,
                                         int *length);

/**
 *Retrieve the `url` as const char.
 */
void strasbourgpark_location_get_url(const SPLocationOpenData *ptr, const char **url, int *length);

/**
 *Retrieve the `name` as const char.
 */
void strasbourgpark_location_get_name(const SPLocationOpenData *ptr,
                                      const char **name,
                                      int *length);

/**
 *Retrieve the `deaf_access` as const int.
 */
int strasbourgpark_location_get_deaf_access(const SPLocationOpenData *ptr);

/**
 *Retrieve the `elder_access` as const int.
 */
int strasbourgpark_location_get_elder_access(const SPLocationOpenData *ptr);

/**
 *Retrieve the `wheelchair_access` as const int.
 */
int strasbourgpark_location_get_wheelchair_access(const SPLocationOpenData *ptr);

/**
 *Retrieve the `blind_access` as const int.
 */
int strasbourgpark_location_get_blind_access(const SPLocationOpenData *ptr);

SPCoordinate strasbourgpark_location_get_coordinate(const SPLocationOpenData *ptr);

void strasbourgpark_location_get_description(const SPLocationOpenData *ptr,
                                             const char **description,
                                             int *length);

int strasbourg_park_client_init(const SPClient **client);

void strasbourg_park_client_free(SPClient *client);

void strasbourg_park_client_get_locations(SPClient *client);
