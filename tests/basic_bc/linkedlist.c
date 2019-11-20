#include <stdlib.h>

struct SimpleLinkedList {
  int val;
  struct SimpleLinkedList* next;
};

int simple_linked_list(int x) {
  struct SimpleLinkedList list = { x, NULL };
  list.val += 2;
  struct SimpleLinkedList list_1 = { x - 3, NULL };
  struct SimpleLinkedList list_2 = { x * 5, NULL };
  struct SimpleLinkedList list_3 = { x / 2, NULL };
  struct SimpleLinkedList list_4 = { x / 100, NULL };
  list.next = &list_1;
  list_1.next = &list_2;
  list_2.next = &list_3;
  list_3.next = &list_4;
  list_4.next = &list;
  return list.next->next->next->next->next->next->next->next->next->next->val;
}

// this type is indirectly recursive, unlike the directly recursive SimpleLinkedList type above
struct NodeB;
struct NodeA {
  int val;
  struct NodeB* b;
};

struct NodeB {
  int val;
  struct NodeA* a;
};

int indirectly_recursive_type(int x) {
  struct NodeA a = { x, NULL };
  struct NodeB b = { x + 3, NULL };
  struct NodeA a_1 = { x / 4, NULL };
  a.b = &b;
  b.a = &a_1;
  a_1.b = &b;
  return a.b->a->b->a->b->a->b->val;
}

// also test opaque structs
struct SomeOpaqueStruct;

int takes_opaque_struct(struct SomeOpaqueStruct* s) {
  return s != NULL;
}
