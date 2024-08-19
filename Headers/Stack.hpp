#ifndef STACK_HPP
#define STACK_HPP

#include "Sequence.hpp"
#include <stdexcept>

template <typename T>
class Stack
{
private:
    Sequence<T> stack;

public:
    // Constructors and Destructors
    Stack(int initialSize = 8);
    ~Stack();

    // Modifiers
    void push(T element);
    void pop();

    // Data
    T top();
    bool isEmpty();
    int getSize();
};

#include "../Templates/Stack.tpp"

#endif