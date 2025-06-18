#include <project-dirs-c.h>
#include <stdio.h>
#include <string.h>

void project_dirs(char *builder) {
  uint8_t error = 0;
  char *result = project_dirs__from_builder(builder, &error);

  if (error == project_dirs__FromBuilderError_NullInput) {
    printf("Passing nullptr? Really?\n");
  } else if (error) {
    printf("Project dirs error: %d\n", error);
  } else {
    printf("Project dirs: %s\n", result);
    free(result);
  }
}

void project_dirs_print_error() {
  char *unparsable_builder = "{\"application\": \"a\"}";
  uint8_t error = 0;

  const size_t BUFF_SIZE = 2048;

  char mystring[BUFF_SIZE];
  memset(mystring, 0, BUFF_SIZE);

  char *result = project_dirs__from_builder_with_msg(unparsable_builder, &error,
                                                     mystring, BUFF_SIZE);

  if (error) {
    printf("Project dirs failed: %s\n", mystring);
    printf("Project dirs error: %d\n", error);
  } else {
    free(result);
  }
}

int main() {
  char *nullptr_builder = NULL;
  char *ok_builder =
      "{\"application\": \"a\", \"organization\": \"b\", \"qualifier\": \"c\"}";
  char *unparsable_builder = "{\"application\": \"a\"}";

  printf("\n\n -------- Builder fails of some reasons -------- \n");
  project_dirs(nullptr_builder);
  project_dirs(unparsable_builder);

  printf("\n\n -------- Builder is OK -------- \n");
  project_dirs(ok_builder);

  printf("\n\n -------- Retrive error that happened during builder parsing -------- \n");
  project_dirs_print_error();

  printf("\n\n -------- Default simple call -------- \n");
  char *project_dirs = project_dirs__project_dirs("my-app", "ultracorp", "org");
  printf("%s\n", project_dirs);
  free(project_dirs);

  return 0;
}
