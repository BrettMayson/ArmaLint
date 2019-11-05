#define NAME Brett
#define QUOTE(s) #s
#define APPEND(a,b) a##b

name = QUOTE(NAME);
tagged_name = QUOTE(APPEND(USER_,NAME));
