#include "Task.hpp"

Task::Task() = default;

Task::Task( std::string name, std::string desc ) :
  _name( std::move( name ) ), _desc( std::move( desc ) ) {}