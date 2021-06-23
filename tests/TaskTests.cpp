#include "Task.hpp"

#include "gtest/gtest.h"

TEST( Task, Constructor )
{
  ASSERT_NO_THROW( Task( "Task", "This is a test task" ) );
}

struct TaskTest : public ::testing::Test
{
  Task tm;

  TaskTest()
  {
    tm = Task( "Test", "This is a test Task" );
  }
};

TEST_F( TaskTest, Name )
{
  EXPECT_EQ( "Test", tm.name() );

  EXPECT_NO_THROW( tm.name( "New Name" ) );

  EXPECT_EQ( "New Name", tm.name() );
}

TEST_F( TaskTest, Description )
{
  EXPECT_EQ( "This is a test Task", tm.desc() );

  EXPECT_NO_THROW( tm.desc( "New Desc" ) );

  EXPECT_EQ( "New Desc", tm.desc() );
}

TEST_F( TaskTest, Completed )
{
  EXPECT_FALSE( tm.completed() );

  EXPECT_NO_THROW( tm.completed( true ) );

  EXPECT_TRUE( tm.completed() );
}