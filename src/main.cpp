#include <iomanip>
#include <iostream>
#include <map>

#include "Task.hpp"
#include "User.hpp"

void printTasks( const User& user, const std::string& option = "" )
{
  std::vector<Task> tasks;
  if( option == "incomplete" )
  {
    tasks = user.incomplete();
  }
  else if( option == "complete" )
  {
    tasks = user.complete();
  }
  else
  {
    tasks = user.tasks();
  }
  std::cout << "Task Name:     Task Description:\n";
  for( const Task& task : tasks )
  {
    std::cout << std::setw( 15 ) << std::left << task.name();
    std::cout << task.desc() << '\n';
  }
}

int main()
{
  User user1( "Frazzer", "admin" );
  Task t1( "Test 1", "Tests Task 1" );
  Task t2( "Test 2", "Tests Task 2" );
  Task t3( "Test 3", "Tests Task 3" );
  Task t4( "Test 4", "Tests Task 4" );
  t1.completed( true );
  user1.addTask( t1 );
  user1.addTask( t2 );
  user1.addTask( t3 );
  user1.addTask( t4 );

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

  int  choice = 0;
  bool exit   = false;

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
      case( 1 ):
        printTasks( user, "incomplete" );
        break;
      case( 2 ):
        printTasks( user );
        break;
      default: continue;
    }
  }
}