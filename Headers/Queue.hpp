#ifndef QUEUE_HPP
#define QUEUE_HPP

#include "Sequence.hpp"
#include <stdexcept>

template <typename T>
class Queue
{
private:
    Sequence<T> queue;

public:
    // Constructors and Destructors
    Queue(int initialSize = 8);
    ~Queue();

    // Modifiers
    void push(T element);
    void pop();

    // Data
    T peek();
    bool isEmpty();
    int getSize();
};

#include "../Templates/Queue.tpp"

#endif