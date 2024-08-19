#ifndef NODE_HPP
#define NODE_HPP

template <typename T>
class Node
{
private:
    T data;
    Node *next;

public:
    // Constructors and Destructor
    Node(T data, Node *next = nullptr);
    ~Node();

    // Getters and Setters
    T &getData();
    void setData(T data);

    Node *getNext();
    void setNext(Node *next);
};

#include "../Templates/Node.tpp"

#endif