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

// Constructors and Destructors
template <typename T>
Stack<T>::Stack(int initialSize) : stack(initialSize)
{
}

template <typename T>
Stack<T>::~Stack()
{
    // Nothing
}

// Modifiers
template <typename T>
void Stack<T>::push(T element)
{
    stack.push(element);
}

template <typename T>
void Stack<T>::pop()
{
    if (isEmpty())
    {
        throw std::out_of_range("Stack is empty");
    }
    stack.pop();
}

// Data
template <typename T>
T Stack<T>::top()
{
    if (isEmpty())
    {
        throw std::out_of_range("Stack is empty");
    }
    return stack[stack.getSize() - 1];
}

template <typename T>
bool Stack<T>::isEmpty()
{
    return stack.getSize() == 0;
}

template <typename T>
int Stack<T>::getSize()
{
    return stack.getSize();
}

#endif