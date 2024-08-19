#ifndef DEQUE_HPP
#define DEQUE_HPP

#include "Sequence.hpp"
#include <stdexcept>

template <typename T>
class Deque
{
private:
    Sequence<T> deque;

public:
    // Constructors and Destructors
    Deque(int initialSize = 8);
    ~Deque();

    // Modifiers
    void push_front(T element);
    void pop_front();
    void push_back(T element);
    void pop_back();

    // Data
    T front();
    T back();
    bool isEmpty();
    int getSize();
};

#include "../Templates/Deque.tpp"

#endif