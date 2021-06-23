#include "TaskManager.hpp"

#include "gtest/gtest.h"

TEST( TaskManager, Constructor )
{
  EXPECT_NO_THROW( TaskManager( "Task", "This is a test task" ) );
}

struct TaskManagerTest : public ::testing::Test
{
  TaskManager tm;

  TaskManagerTest()
  {
    tm = TaskManager( "Test", "This is a test Task" );
  }
};

TEST_F( TaskManagerTest, Name )
{
  EXPECT_EQ( "Test", tm.name() );

  EXPECT_NO_THROW( tm.name( "New Name" ) );

  EXPECT_EQ( "New Name", tm.name() );
}

TEST_F( TaskManagerTest, Description )
{
  EXPECT_EQ( "This is a test Task", tm.desc() );

  EXPECT_NO_THROW( tm.desc( "New Desc" ) );

  EXPECT_EQ( "New Desc", tm.desc() );
}

TEST_F( TaskManagerTest, Completed )
{
  EXPECT_FALSE( tm.completed() );

  EXPECT_NO_THROW( tm.completed( true ) );

  EXPECT_TRUE( tm.completed() );
}