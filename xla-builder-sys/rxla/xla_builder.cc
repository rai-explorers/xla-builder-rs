#include "rxla/xla_builder.h"

XlaBuilder *xla_builder_create(const char *name, size_t name_size)
{
    const std::string name_str(name, name_size);
    return new XlaBuilder(name_str);
}

void xla_builder_destroy(XlaBuilder *builder)
{
    delete builder;
}