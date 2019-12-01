#include <stdio.h>
#include <stdlib.h>

#define NUM_ARGS 1

long module_fuel (long mass) {
  long fuel = (mass / 3) - 2;
  return (fuel > 0) ? fuel + module_fuel(fuel) : 0;
}

int main(int argc, char* argv[]) 
{
  FILE *masses;
  char *mass_str = NULL;
  long fuel = 0;
  size_t mass_str_len = 0;
  ssize_t read_len;

  if (argc != NUM_ARGS + 1)
  {
    printf("ERROR: Incorrect number of arguments (%d)\n", argc);
    exit(-1);
  }
  
  if ((masses = fopen(argv[1], "r")) == NULL) 
  {
    printf("ERROR: Could not open file\n");
    exit(-1);
  }

  while ((read_len = getline(&mass_str, &mass_str_len, masses)) != -1)
  {
    long current_mass = atoi(mass_str);
    long fuel_req = module_fuel(current_mass);
    fuel += fuel_req;
  }

  printf("TOTAL FUEL REQ: %ld\n", fuel);

  free(mass_str);
}
