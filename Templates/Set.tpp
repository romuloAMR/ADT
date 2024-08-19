#include "../Headers/Set.hpp"

template <typename T>
Set<T>::Set() = default;

template <typename T>
Set<T>::~Set() = default;

template <typename T>
void Set<T>::add(const T& element)
{
    if (!contains(element))
    {
        set.push_back(element);
    }
}

template <typename T>
void Set<T>::pop(const T& element)
{
    auto it = std::find(set.begin(), set.end(), element);
    if (it != set.end())
    {
        set.erase(it);
    }
}

template <typename T>
bool Set<T>::contains(const T& element) const
{
    return std::find(set.begin(), set.end(), element) != set.end();
}

template <typename T>
int Set<T>::len() const
{
    return set.size();
}

template <typename U>
std::ostream &operator<<(std::ostream &os, const Set<U> &s)
{
    os << "{ ";
    for (const auto& elem : s.set) {
        os << elem << " ";
    }
    os << "}";
    return os;
}