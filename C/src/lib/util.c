#include "util.h"

llist_t* deliminate_str(char* string, char* deliminator) {
   llist_t* list = llist_new();
   rsize_t max = sizeof string;
   char** context;
   char* current_string = strtok(string, deliminator);
   
   while(current_string != NULL) {
      llist_push(list, current_string);
      strtok(NULL, deliminator);
   }

   return list;
}
