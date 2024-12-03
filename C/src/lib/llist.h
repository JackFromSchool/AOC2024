#ifndef LLIST_H
#define LLIST_H

#include <stdbool.h>
#include <stdlib.h>

typedef struct llist_node {
   void* data;
   struct llist_node* next;
} llist_node_t;

typedef struct llist {
   llist_node_t* head;
   int length;
} llist_t;

llist_t* llist_new();
void llist_add(llist_t* list, void* data);
void* llist_get(llist_t* list, int i);
bool llist_remove(llist_t*, int i);

void llist_push(llist_t* list, void* data);
void* llist_pop(llist_t* list);

#endif
