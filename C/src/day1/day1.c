#include "day1.h"
#include <stdio.h>
#include <string.h>

#include "../lib/llist.h"
#include "../lib/util.h"

int compare_int(void* _context, const void* a, const void* b) {
   return *((int*)a) - *((int*)b);
}

int day1_part_one(char* input) {
   char substr[30];
   llist_t* list = llist_new();
   int substr_index = 0;
   
   for(int i = 0; i <= strlen(input); i++) {
      if (input[i] == '\n' || input[i] == '\0') {
         substr[substr_index] = '\0';
         char* to_be_stored = malloc(sizeof(char)*strlen(substr));
         strcpy_s(to_be_stored, sizeof(char)*(strlen(substr)+1), substr);
         substr_index = 0;
         llist_push(list, to_be_stored);
         continue;
      }

      substr[substr_index] = input[i];
      substr_index += 1;
   }

   int* list1 = malloc(sizeof(int)*list->length);
   int* list2 = malloc(sizeof(int)*list->length);

   for(int i = 0; i < list->length; i++) {
      int num1;
      int num2;
      sscanf_s(llist_get(list, i), "%d %d", &num1, &num2);
      list1[i] = num1;
      list2[i] = num2;
   }

   qsort_s(list1, list->length, sizeof(int), compare_int, NULL);
   qsort_s(list2, list->length, sizeof(int), compare_int, NULL);

   int total_distance = 0;

   for(int i = 0; i < list->length; i++) {
      total_distance += abs(list1[i] - list2[i]);
   }

   return total_distance;
}

int day1_part_two(char* input) {
   char substr[30];
   llist_t* list = llist_new();
   int substr_index = 0;
   
   for(int i = 0; i <= strlen(input); i++) {
      if (input[i] == '\n' || input[i] == '\0') {
         substr[substr_index] = '\0';
         char* to_be_stored = malloc(sizeof(char)*strlen(substr));
         strcpy_s(to_be_stored, sizeof(char)*(strlen(substr)+1), substr);
         substr_index = 0;
         llist_push(list, to_be_stored);
         continue;
      }

      substr[substr_index] = input[i];
      substr_index += 1;
   }

   int* list1 = malloc(sizeof(int)*list->length);
   int* list2 = malloc(sizeof(int)*list->length);

   for(int i = 0; i < list->length; i++) {
      int num1;
      int num2;
      sscanf_s(llist_get(list, i), "%d %d", &num1, &num2);
      list1[i] = num1;
      list2[i] = num2;
   }

   int similarity_score = 0;

   for(int i = 0; i < list->length; i++) {
      int count = 0;
      for(int j = 0; j < list->length; j++) {
         if (list1[i] == list2[j]) {
            count++;
         }
      }
      similarity_score += count*list1[i];
   }

   return similarity_score;
}
