#include <iostream>
#include <vector>
#include <string>

// C++
int main() {
    std::vector<std::string> v;
    v.push_back("Hello");
    std::string& x = v[0];
    v.push_back("world");
    std::count << x;
}