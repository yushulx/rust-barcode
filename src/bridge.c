#include "bridge.h"
#include "DynamsoftBarcodeReader.h"
#include <stdio.h>

RustCallback cb;

int32_t register_callback(RustCallback callback) {
    cb = callback;
    return 1;
}

void c_decodeFile(const char *fileName, const char *pszLicense)
{
    void *hBarcode = DBR_CreateInstance();

    if (hBarcode)
    {
        int ret = DBR_InitLicense(hBarcode, pszLicense);
        STextResultArray *paryResult = NULL;
        ret = DBR_DecodeFile(hBarcode, fileName, "");
        DBR_GetAllTextResults(hBarcode, &paryResult);
        int count = paryResult->nResultsCount;
        printf("Barcode found: %d\n", count);
        int i = 0;
        for (; i < count; i++)
        {
            // printf("Index: %d, Barcode Type: %s, Value: %s\n", i, paryResult->ppResults[i]->pszBarcodeFormatString, paryResult->ppResults[i]->pszBarcodeText);
            if (cb) 
            {
                cb(i, paryResult->ppResults[i]->pszBarcodeFormatString, paryResult->ppResults[i]->pszBarcodeText);
            }
        }
        DBR_FreeTextResults(&paryResult);
        DBR_DestroyInstance(hBarcode);
    }
}