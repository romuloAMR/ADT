#ifndef SEQUENCE_HPP
#define SEQUENCE_HPP

#include <stdexcept>
#include <iostream>

template <typename T>
class Sequence
{
private:
    T* sequence;
    int length;
    int size;

    // Private methods
    void add(T value, int index);
    void rm(int index);

    void resize();

public:
    // Constructors and Destructors
    Sequence(int initialSize = 8);
    ~Sequence();

    // Overloading
    T& operator[](int index);

    // Modifiers
    void push(T value, int index = -1);
    void pop(int index = -1);

    // Others
    int getSize() const;
};

// Private Methods

template <typename T>
void Sequence<T>::add(T value, int index)
{
    for (int i = size; i > index; i--)
    {
        sequence[i] = sequence[i - 1];
    }
    sequence[index] = value;
}

template <typename T>
void Sequence<T>::rm(int index)
{
    for (int i = index; i < size - 1; i++)
    {
        sequence[i] = sequence[i + 1];
    }
    sequence[size - 1] = T(); 
}

template <typename T>
void Sequence<T>::resize()
{
    if (size >= length)
    {
        length += 8;
        T* newSequence = new T[length];
        for (int i = 0; i < size; i++)
        {
            newSequence[i] = sequence[i];
        }
        delete[] sequence;
        sequence = newSequence;
    }
}

// Constructor and Destructor

template <typename T>
Sequence<T>::Sequence(int initialSize) : size(0)
{
    this->length = ((initialSize / 8) + 1) * 8;
    sequence = new T[length];
}

template <typename T>
Sequence<T>::~Sequence()
{
    delete[] sequence;
}

// Operator Overloading

template <typename T>
T& Sequence<T>::operator[](int index)
{
    if (index >= 0 && index < size)
    {
        return sequence[index];
    }

    throw std::out_of_range("Index out of bounds");
}

// Modifiers

template <typename T>
void Sequence<T>::push(T value, int index)
{
    if (index == -1)
    {
        index = size;
    }

    if (index >= 0 && index <= size)
    {
        if (size >= length)
        {
            resize();
        }
        add(value, index);
        size++;
    }
    else
    {
        throw std::out_of_range("Index out of bounds");
    }
}

template <typename T>
void Sequence<T>::pop(int index)
{
    if (index == -1)
    {
        index = size - 1;
    }

    if (index >= 0 && index < size)
    {
        rm(index);
        size--;
    }
    else
    {
        throw std::out_of_range("Index out of bounds");
    }
}

// Others

template <typename T>
int Sequence<T>::getSize() const
{
    return size;
}

#endif