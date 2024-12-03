#include <stdio.h>
#include <stdlib.h>

#include "day1/day1.h"

char* get_string_from_file(char* path);

int main() {
   
   char* day1_input = get_string_from_file("input.txt");
   printf("%d\n", day1_part_one(day1_input));
   printf("%d\n", day1_part_two(day1_input));

}

char* get_string_from_file(char* path) {
   FILE* f;
   char* str = NULL;
   long length;

   fopen_s(&f, path, "rb");

   if (f == NULL) {
      printf("File could not be opened.");
      exit(1);
   }

   fseek(f, 0, SEEK_END);
   length = ftell(f);
   fseek(f, 0, SEEK_SET);
   str = malloc(length);
   if (str) {
      fread(str, 1, length, f);
   }
   fclose(f);

   str[length-1] = '\0';

   return str;
}
