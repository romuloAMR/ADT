#ifndef NODE2_HPP
#define NODE2_HPP

template <typename T>
class Node2
{
private:
    T data;
    Node2 *next;
    Node2 *before;

public:
    // Constructors and Destructor
    Node2(T data, Node2 *before = nullptr, Node2 *next = nullptr);
    ~Node2();

    // Getters and Setters
    T &getData();
    void setData(T data);

    Node2 *getNext();
    void setNext(Node2 *next);

    Node2 *getBefore();
    void setBefore(Node2 *before);
};

#include "../Templates/Node2.tpp"

#endif