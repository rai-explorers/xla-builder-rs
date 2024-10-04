#include <stddef.h>

#ifdef __cplusplus
#include "absl/status/status.h"
#include "xla/hlo/builder/xla_builder.h"

typedef absl::Status Status;
typedef xla::XlaBuilder XlaBuilder;
typedef xla::XlaComputation XlaComputation;
typedef xla::XlaOp XlaOp;
typedef xla::PrimitiveType PrimitiveType;

extern "C"
{
#else
typedef struct Status Status;
typedef struct XlaBuilder XlaBuilder;
typedef struct XlaComputation XlaComputation;
typedef struct XlaOp XlaOp;
typedef int PrimitiveType;
#endif

    XlaBuilder *xla_builder_create(const char *name, size_t name_size);
    void xla_builder_destroy(XlaBuilder *builder);

#ifdef __cplusplus
}
#endif