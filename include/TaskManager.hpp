
#include <string>

class TaskManager
{
  std::string _name;
  std::string _desc;
  bool        _completed;

public:
  // Constructors
  TaskManager();
  TaskManager( std::string name, std::string desc );

  // Getters
  std::string name() { return _name; }
  std::string desc() { return _desc; }
  bool        completed() { return _completed; }

  // Setters
  void name( std::string name ) { _name = name; }
  void desc( std::string desc ) { _desc = desc; }
  void completed( bool state ) { _completed = state; }

  // Functions
};