#ifndef NODE_HPP
#define NODE_HPP

template <typename T>
class Node
{
private:
    T data;
    Node *next;

public:
    // Constructors and Destructor
    Node(T data, Node *next = nullptr);
    ~Node();

    // Getters and Setters
    T &getData();
    void setData(T data);

    Node *getNext();
    void setNext(Node *next);
};

#endif

// Constructors and Destructor
template <typename T>
Node<T>::Node(T data, Node *next) : data(data), next(next)
{
}

template <typename T>
Node<T>::~Node()
{
    // Nothing
}

// Getters and Setters
template <typename T>
T &Node<T>::getData()
{
    return data;
}

template <typename T>
void Node<T>::setData(T data)
{
    this->data = data;
}

template <typename T>
Node<T> *Node<T>::getNext()
{
    return next;
}

template <typename T>
void Node<T>::setNext(Node *next)
{
    this->next = next;
}