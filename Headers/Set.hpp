#ifndef SET_HPP
#define SET_HPP

#include <vector>
#include <algorithm>
#include <iostream>

template <typename T>
class Set
{
private:
    std::vector<T> set;
public:
    Set();
    ~Set();
    void add(const T& element);
    void pop(const T& element);
    bool contains(const T& element) const;
    int len() const;

    template <typename U>
    friend std::ostream& operator<<(std::ostream& os, const Set<U>& s);
};

#include "../Templates/Set.tpp"

#endif