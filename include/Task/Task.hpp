
#include <string>
#include <utility>

class Task
{
  std::string _name;
  std::string _desc;
  bool        _completed { false };

public:
  // Constructors
  Task();
  Task( std::string name, std::string desc );

  // Getters
  [[nodiscard]] std::string name() const { return _name; }
  [[nodiscard]] std::string desc() const { return _desc; }
  [[nodiscard]] bool        completed() const { return _completed; }

  // Setters
  void name( std::string name ) { _name = std::move( name ); }
  void desc( std::string desc ) { _desc = std::move( desc ); }
  void completed( bool state ) { _completed = state; }

  // Functions
};