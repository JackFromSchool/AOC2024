#include "llist.h"

#include <stdio.h>

llist_t* llist_new() {
   llist_t* list = malloc(sizeof(llist_t));
   list->head = NULL;
   list->length = 0;

   return list;
}

void llist_add(llist_t* list, void* data) {
   int len = list->length - 1;
   llist_node_t* cur = list->head;

   if (len == -1) {
      list->head = malloc(sizeof(llist_node_t));
      list->head->next = NULL;
      list->head->data = data;
      list->length += 1;
      return;
   }

   while(len != 0) {
      cur = cur->next;
      len -= 1;
   }

   list->length += 1;
   cur->next = malloc(sizeof(llist_node_t));
   cur->next->next = NULL;
   cur->next->data = data;
}

void llist_push(llist_t* list, void* data) {
   llist_add(list, data);
}

bool llist_remove(llist_t* list, int i) {
   if (i < 0 || i > list->length-1) {
      return false;
   }
   i -= 1;
   
   llist_node_t* cur = list->head;
   if (i < 0) {
      list->head = list->head->next;
      free(cur);
      list->length -= 1;
      return true;
   }
   
   while(i != 0) {
      cur = cur->next;
      i -= 1;
   }
   
   llist_node_t* to_be_freed = cur->next;
   cur->next = cur->next->next;
   free(to_be_freed);
   list->length -= 1;

   return true;
}

void* llist_pop(llist_t* list) {
   void* data = llist_get(list, list->length - 1);
   llist_remove(list, list->length - 1);
   return data;
}

void* llist_get(llist_t* list, int i) {
   if (i < 0 || i > list->length-1) {
      return NULL;
   }

   
   llist_node_t* cur = list->head;
   while(i != 0) {
      cur = cur->next;
      i -= 1;
   }

   return cur->data;
}
