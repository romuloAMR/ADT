#include "../Headers/Node.hpp"

// Constructors and Destructor
template <typename T>
Node<T>::Node(T data, Node *next) : data(data), next(next)
{
}

template <typename T>
Node<T>::~Node()
{
    // Nothing
}

// Getters and Setters
template <typename T>
T &Node<T>::getData()
{
    return data;
}

template <typename T>
void Node<T>::setData(T data)
{
    this->data = data;
}

template <typename T>
Node<T> *Node<T>::getNext()
{
    return next;
}

template <typename T>
void Node<T>::setNext(Node *next)
{
    this->next = next;
}