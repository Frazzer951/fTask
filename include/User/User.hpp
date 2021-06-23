#include <string>
#include <utility>

class User
{
  std::string _username;
  std::string _password;

public:
  // Constructor
  User();
  User( std::string username, std::string password );

  // Getter
  [[nodiscard]] std::string username() const { return _username; }

  // Setter
  void username( std::string username ) { _username = std::move( username ); }
  void password( std::string password ) { _password = std::move( password ); }

  // Functions
  bool check_pass( const std::string & guess );
};