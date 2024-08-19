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

#include "../Templates/Sequence.tpp"

#endif