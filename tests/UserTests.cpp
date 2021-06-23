#include "Task.hpp"
#include "User.hpp"

#include "gtest/gtest.h"

TEST( User, Constructor )
{
  ASSERT_NO_THROW( User( "Test_Name", "Password" ) );
}

struct UserTest : public ::testing::Test
{
  User user;

  UserTest()
  {
    user = User( "Username", "Password" );
  }
};

TEST_F( UserTest, Username )
{
  EXPECT_EQ( "Username", user.username() );

  EXPECT_NO_THROW( user.username( "New Username" ) );

  EXPECT_EQ( "New Username", user.username() );
}

TEST_F( UserTest, Password )
{
  EXPECT_TRUE( user.check_pass( "Password" ) );
  EXPECT_FALSE( user.check_pass( "Hello" ) );

  EXPECT_NO_THROW( user.password( "Test" ) );

  EXPECT_FALSE( user.check_pass( "Password" ) );
  EXPECT_TRUE( user.check_pass( "Test" ) );
}

TEST_F( UserTest, AddTask_1 )
{
  user.addTask( "Test", "This is a test task" );

  Task task = user.tasks()[0];

  EXPECT_EQ( "Test", task.name() );
  EXPECT_EQ( "This is a test task", task.desc() );
  EXPECT_FALSE( task.completed() );
}

TEST_F( UserTest, AddTask_2 )
{
  user.addTask( Task( "Test", "This is a test task" ) );

  Task task = user.tasks()[0];

  EXPECT_EQ( "Test", task.name() );
  EXPECT_EQ( "This is a test task", task.desc() );
  EXPECT_FALSE( task.completed() );
}

struct UserTestTasks : public ::testing::Test
{
  User user;
  Task t1;
  Task t2;
  Task t3;
  Task t4;

  UserTestTasks()
  {
    user = User( "Username", "Password" );

    t1 = Task( "Test 1", "Tests Task 1" );
    t2 = Task( "Test 2", "Tests Task 2" );
    t3 = Task( "Test 3", "Tests Task 3" );
    t4 = Task( "Test 4", "Tests Task 4" );

    user.addTask( t1 );
    user.addTask( t2 );
    user.addTask( t3 );
    user.addTask( t4 );
  }
};

TEST_F( UserTestTasks, Tasks )
{
  auto tasks = user.tasks();

  EXPECT_EQ( t1, tasks[0] );
  EXPECT_EQ( t2, tasks[1] );
  EXPECT_EQ( t3, tasks[2] );
  EXPECT_EQ( t4, tasks[3] );
}

TEST_F( UserTestTasks, Incomplete )
{
  auto tasks = user.incomplete();

  EXPECT_EQ( t1, tasks[0] );
  EXPECT_EQ( t2, tasks[1] );
  EXPECT_EQ( t3, tasks[2] );
  EXPECT_EQ( t4, tasks[3] );
}

TEST_F( UserTestTasks, Complete )
{
  Task t5( "Test 5", "Tests Task 5" );
  Task t6( "Test 6", "Tests Task 6" );
  t5.completed( true );
  t6.completed( true );

  user.addTask( t5 );
  user.addTask( t6 );

  auto incomplete = user.incomplete();
  auto complete   = user.complete();

  EXPECT_EQ( t1, incomplete[0] );
  EXPECT_EQ( t2, incomplete[1] );
  EXPECT_EQ( t3, incomplete[2] );
  EXPECT_EQ( t4, incomplete[3] );
  EXPECT_EQ( t5, complete[0] );
  EXPECT_EQ( t6, complete[1] );
}

TEST_F( UserTestTasks, CompleteTask )
{
  auto tasks = user.tasks();

  user.completeTask( tasks[1] );
  user.completeTask( tasks[3] );

  tasks = user.tasks();

  auto incomplete = user.incomplete();
  auto complete   = user.complete();

  EXPECT_EQ( tasks[0], incomplete[0] );
  EXPECT_EQ( tasks[2], incomplete[1] );
  EXPECT_EQ( tasks[1], complete[0] );
  EXPECT_EQ( tasks[3], complete[1] );
}

TEST_F( UserTestTasks, CompleteTaskThrow )
{
  Task t( "Goto Store", "Buy food" );

  EXPECT_ANY_THROW( user.completeTask( t ) );
}