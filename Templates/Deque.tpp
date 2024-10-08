#include "../Headers/Deque.hpp"

// Constructors and Destructors
template <typename T>
Deque<T>::Deque(int initialSize) : deque(initialSize)
{
}

template <typename T>
Deque<T>::~Deque()
{
    // Nothing
}

// Modifiers
template <typename T>
void Deque<T>::push_front(T element)
{
    deque.push(element, 0);
}

template <typename T>
void Deque<T>::pop_front()
{
    if (isEmpty())
    {
        throw std::out_of_range("Deque is empty");
    }
    deque.pop(0);
}

template <typename T>
void Deque<T>::push_back(T element)
{
    deque.push(element);
}

template <typename T>
void Deque<T>::pop_back()
{
    if (isEmpty())
    {
        throw std::out_of_range("Deque is empty");
    }
    deque.pop();
}
// Data
template <typename T>
T Deque<T>::front()
{
    if (isEmpty())
    {
        throw std::out_of_range("Deque is empty");
    }
    return deque[0];
}

template <typename T>
T Deque<T>::back()
{
    if (isEmpty())
    {
        throw std::out_of_range("Deque is empty");
    }
    return deque[deque.getSize()-1];
}

template <typename T>
bool Deque<T>::isEmpty()
{
    return deque.getSize() == 0;
}

template <typename T>
int Deque<T>::getSize()
{
    return deque.getSize();
}