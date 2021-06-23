#include "TaskManager.hpp"

TaskManager::TaskManager() :
  _name( "" ), _desc( "" ), _completed( false ) {}

TaskManager::TaskManager( std::string name, std::string desc ) :
  _name( name ), _desc( desc ), _completed( false ) {}