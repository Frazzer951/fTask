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