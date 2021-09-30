#include <stdio.h>
#include <stdlib.h>

// node -- element contains an integer(value)., and a node pointer which points to itself. // points to an object of a structure of same type of itself


struct Node{
    int x;// has an value
    struct Node* n; //has an pointer to an other element with the same type as itself.

};

/*
int main(int argc, const char* argv[]){

    struct Node* head, *tmp, *curr;     //

    // first node creation
    tmp = malloc(sizeof(struct Node)); // retu
    tmp->x = 4;
    tmp->n = NULL;

    head = tmp;
    curr = tmp;

    // second node creation

    tmp = malloc(sizeof(struct Node)); // retu
    tmp->x = 5;
    tmp->n = NULL;


    //finding first node
    curr->n = tmp; //stiches second node onto the end of the first node
    curr = tmp;   //curr points at second now 


    return 0;    
}
*/


void print_LL(struct Node* p){

    struct Node* tmp = p;
    //printf("*val: %d \n");
    while(tmp != NULL){
        printf("*val: %d \n", tmp->x);
        tmp = tmp->n;
    }


}
int arr[5]= { 1, 3, 4 ,5 , 6};//fixed size
int main(int argc, const char* argv[]){


    struct Node* head, *tmp, *curr;
    
    for (int i = 0; i < 5; i++)
    {
        tmp = malloc(sizeof(struct Node)); // return
        tmp->x = arr[i];
        tmp->n = NULL;

        if(i == 0){
            head = tmp;
            curr = tmp;
        }
        else
        {
            curr->n = tmp;   // tmp pointer to second node
            curr = tmp;     // new created node.
        }
    }
    print_LL(head);
}