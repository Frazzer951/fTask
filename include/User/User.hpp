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
  [[nodiscard]] std::string       username() const { return _username; }
  [[nodiscard]] std::vector<Task> tasks() const { return _tasks; }
  [[nodiscard]] std::vector<Task> incomplete() const;
  [[nodiscard]] std::vector<Task> complete() const;


  // Setter
  void username( std::string username ) { _username = std::move( username ); }
  void password( std::string password ) { _password = std::move( password ); }

  // Functions
  bool check_pass( const std::string & guess );
  void addTask( const std::string & name, const std::string & desc );
  void addTask( const Task & task );
  void completeTask( const Task & task );
};