typedef struct Barcode {
    char* barcode_type;
    char* barcode_value;
} Barcode;

typedef __int32 int32_t;
typedef void (*RustCallback)(int32_t, const char *, const char *);

int32_t register_callback(RustCallback callback);
void c_decodeFile(const char *fileName, const char *pszLicense);