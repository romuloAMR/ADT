#include "../Headers/Stack.hpp"

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