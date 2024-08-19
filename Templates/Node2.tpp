#include "../Headers/Node2.hpp"

// Constructors and Destructor
template <typename T>
Node2<T>::Node2(T data, Node2 *before, Node2 *next) : data(data), before(before), next(next)
{
}

template <typename T>
Node2<T>::~Node2()
{
    if (before != nullptr)
    {
        before->setNext(next);
    }
    if (next != nullptr)
    {
        next->setBefore(before);
    }
}

// Getters and Setters
template <typename T>
T &Node2<T>::getData()
{
    return data;
}

template <typename T>
void Node2<T>::setData(T data)
{
    this->data = data;
}

template <typename T>
Node2<T> *Node2<T>::getNext()
{
    return next;
}

template <typename T>
void Node2<T>::setNext(Node2 *next)
{
    this->next = next;
}

template <typename T>
Node2<T> *Node2<T>::getBefore()
{
    return before;
}

template <typename T>
void Node2<T>::setBefore(Node2 *before)
{
    this->before = before;
}