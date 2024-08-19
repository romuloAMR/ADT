#include "../Headers/Queue.hpp"

// Constructors and Destructors
template <typename T>
Queue<T>::Queue(int initialSize) : queue(initialSize)
{
}

template <typename T>
Queue<T>::~Queue()
{
    // Nothing
}

// Modifiers
template <typename T>
void Queue<T>::push(T element)
{
    queue.push(element);
}

template <typename T>
void Queue<T>::pop()
{
    if (isEmpty())
    {
        throw std::out_of_range("Queue is empty");
    }
    queue.pop(0);
}

// Data
template <typename T>
T Queue<T>::peek()
{
    if (isEmpty())
    {
        throw std::out_of_range("Queue is empty");
    }
    return queue[0];
}

template <typename T>
bool Queue<T>::isEmpty()
{
    return queue.getSize() == 0;
}

template <typename T>
int Queue<T>::getSize()
{
    return queue.getSize();
}