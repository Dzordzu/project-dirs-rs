#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum project_dirs__FromBuilderError {
  project_dirs__FromBuilderError_NoError = 0,
  project_dirs__FromBuilderError_NullInput = 1,
  project_dirs__FromBuilderError_NonStrInput = 2,
  project_dirs__FromBuilderError_BuilderParsingFailed = 3,
  project_dirs__FromBuilderError_ResultSerializationFailed = 4,
};
typedef uint8_t project_dirs__FromBuilderError;

char *project_dirs__from_builder(const char *s, project_dirs__FromBuilderError *error);

char *project_dirs__project_dirs(const char *application,
                                 const char *organization,
                                 const char *qualifier);

char *project_dirs__from_builder_with_msg(const char *s,
                                          project_dirs__FromBuilderError *error,
                                          char *buf_error_msg,
                                          uintptr_t buf_error_len);
