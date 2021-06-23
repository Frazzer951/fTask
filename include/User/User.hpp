#include <string>

class User
{
  std::string _username;
  std::string _password;

public:
  // Constructor
  User();
  User( std::string username, std::string password );

  // Getter
  std::string username() const { return _username; }

  // Setter
  void username( std::string username ) { _username = username; }
  void password( std::string password ) { _password = password; }

  // Functions
  bool check_pass( std::string guess );
};