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

#endif

// Constructors and Destructors
template <typename T>
List<T>::List() : head(nullptr), size(0) {}

template <typename T>
List<T>::~List()
{
    Node<T> *current = head;
    while (current != nullptr)
    {
        Node<T> *next = current->getNext();
        delete current;
        current = next;
    }
}

// Operator Overloading
template <typename T>
T &List<T>::operator[](int index)
{
    if (index >= 0 && index < size)
    {
        Node<T> *current = head;
        for (int i = 0; i < index; ++i)
        {
            current = current->getNext();
        }
        return current->getData();
    }
    throw std::out_of_range("Index out of bounds");
}

// Modifiers
template <typename T>
void List<T>::push(T value, int index)
{
    if (index == -1)
    {
        index = getSize();
    }

    if (index < 0 || index > getSize())
    {
        throw std::out_of_range("Index out of bounds");
    }

    if (index == 0)
    {
        head = new Node<T>(value, head);
    }
    else
    {
        Node<T> *before = nullptr;
        Node<T> *current = head;
        for (int i = 0; i < index; ++i)
        {
            before = current;
            current = current->getNext();
        }
        before->setNext(new Node<T>(value, current));
    }
    ++size;
}

template <typename T>
void List<T>::pop(int index)
{
    if (index == -1)
    {
        index = getSize() - 1;
    }

    if (index < 0 || index >= getSize())
    {
        throw std::out_of_range("Index out of bounds");
    }

    Node<T> *toDelete = nullptr;

    if (index == 0)
    {
        toDelete = head;
        head = head->getNext();
    }
    else
    {
        Node<T> *before = nullptr;
        Node<T> *current = head;
        for (int i = 0; i < index; ++i)
        {
            before = current;
            current = current->getNext();
        }
        toDelete = current;
        before->setNext(current->getNext());
    }

    delete toDelete;
    --size;
}

// Others
template <typename T>
int List<T>::getSize() const
{
    return size;
}
