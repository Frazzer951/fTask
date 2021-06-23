#pragma once

#include <string>
#include <utility>
#include <vector>

#include "Task.hpp"

class User
{
  std::string       _username;
  std::string       _password;
  std::vector<Task> _tasks;

public:
  // Constructor
  User();
  User( std::string username, std::string password );

  // Getter
  [[nodiscard]] std::string username() const { return _username; }
  std::vector<Task>         tasks() const { return _tasks; }
  std::vector<Task>         incomplete() const;
  std::vector<Task>         complete() const;


  // Setter
  void username( std::string username ) { _username = std::move( username ); }
  void password( std::string password ) { _password = std::move( password ); }

  // Functions
  bool check_pass( const std::string & guess );
  void addTask( std::string name, std::string desc );
  void addTask( Task task );
  void completeTask( Task task );
};