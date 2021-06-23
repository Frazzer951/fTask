#include <algorithm>

#include "User.hpp"


User::User() = default;

User::User( std::string username, std::string password ) :
  _username( std::move( username ) ), _password( std::move( password ) ) {}

std::vector<Task> User::incomplete() const
{
  std::vector<Task> imcomplete;

  for( const auto & task : _tasks )
  {
    if( !task.completed() )
      imcomplete.push_back( task );
  }

  return imcomplete;
}

std::vector<Task> User::complete() const
{
  std::vector<Task> complete;

  for( const auto & task : _tasks )
  {
    if( task.completed() )
      complete.push_back( task );
  }

  return complete;
}

bool User::check_pass( const std::string & guess )
{
  return guess == _password;
}

void User::addTask( const std::string & name, const std::string & desc )
{
  _tasks.emplace_back( name, desc );
}

void User::addTask( const Task & task )
{
  _tasks.push_back( task );
}

void User::completeTask( const Task & task )
{
  auto _task = std::find( _tasks.begin(), _tasks.end(), task );

  if( _task == _tasks.end() )
    throw "Task not found for current user";

  _task->completed( true );
}
