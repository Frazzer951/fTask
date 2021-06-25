#include <iostream>
#include <map>

#include "Task.hpp"
#include "User.hpp"

int main()
{
  User user1( "Frazzer", "admin" );

  std::map<std::string, User> users;
  users["Frazzer"] = user1;

  std::string username;
  std::string password;
  bool        user_exists = false;
  do
  {
    std::cout << "Enter Username: ";
    std::getline( std::cin, username );
    if( users.find( username ) == users.end() )

      std::cout << "No user with that name exists\n";

    else
      user_exists = true;
  }
  while( !user_exists );

  User user = users[username];

  bool valid_pass = false;
  do {
    std::cout << "Enter Password: ";
    std::getline( std::cin, password );
    if( user.check_pass( password ) )

      valid_pass = true;

    else

      std::cout << "Incorrect Password\n";
  }
  while( !valid_pass );

  int  choice;
  bool exit = false;

  while( !exit )
  {
    std::cout << "Enter an Option of Below: \n";
    std::cout << "1. See Remaining Tasks\n";
    std::cout << "2. See All Task\n";

    std::cout << "0. Exit\n";

    std::cin >> choice;
    std::cin.ignore();

    switch( choice )
    {
      case( 0 ):
        exit = true;
        break;
      default: continue;
    }
  }
}