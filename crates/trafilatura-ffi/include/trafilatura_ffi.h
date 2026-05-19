#ifndef TRAFILATURA_FFI_H
#define TRAFILATURA_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct TrafilaturaResult {
    char *data;
    char *error;
} TrafilaturaResult;

TrafilaturaResult trafilatura_extract_text(const char *html);
TrafilaturaResult trafilatura_extract_json_for_mcp(const char *html);
TrafilaturaResult trafilatura_extract_with_options_json(
    const char *html,
    const char *options_json
);

void trafilatura_free_string(char *ptr);
void trafilatura_free_result(TrafilaturaResult result);

#ifdef __cplusplus
}
#endif

#endif
