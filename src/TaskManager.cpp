#include "TaskManager.hpp"

TaskManager::TaskManager() = default;

TaskManager::TaskManager( std::string name, std::string desc ) :
  _name( std::move( name ) ), _desc( std::move( desc ) ) {}