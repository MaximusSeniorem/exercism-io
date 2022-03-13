#include "reverse_string.h"

namespace reverse_string {
    std::string reverse_string(std::string const & str){
        return {str.rbegin(), str.rend()};
    }
}  // namespace reverse_string
