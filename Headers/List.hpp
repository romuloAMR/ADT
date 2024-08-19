#ifndef LIST_HPP
#define LIST_HPP

#include "Node.hpp"
#include <stdexcept>

template <typename T>
class List
{
private:
    Node<T> *head;
    int size;

public:
    // Constructors and Destructors
    List();
    ~List();

    // Overloading
    T &operator[](int index);

    // Modifiers
    void push(T value, int index = -1);
    void pop(int index = -1);

    // Others
    int getSize() const;
};

#include "../Templates/List.tpp"

#endif