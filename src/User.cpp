#include "User.hpp"

User::User() = default;

User::User( std::string username, std::string password ) :
  _username( std::move( username ) ), _password( std::move( password ) ) {}

bool User::check_pass( std::string guess )
{
  return guess == _password;
}