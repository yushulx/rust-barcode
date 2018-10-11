# Rust Barcode Reader
The sample demonstrates how to implement a simple command line barcode reader using [Rust](https://www.rust-lang.org/en-US/) and [Dynamsoft Barcode Reader SDK](https://www.dynamsoft.com/Products/Dynamic-Barcode-Reader.aspx).

## Installation
* [Rust 1.29.1](https://www.rust-lang.org/en-US/install.html)
* [Dynamsoft Barcode Reader 6.3](https://www.dynamsoft.com/Downloads/Dynamic-Barcode-Reader-Download.aspx)
* [Visual Studio](https://visualstudio.microsoft.com/downloads/)

## Usage
### Windows
1. Get a free [trial license](https://www.dynamsoft.com/CustomerPortal/Portal/Triallicense.aspx) of Dynamsoft Barcode Reader.

2. Set the license:

    ```rust
    let license = CString::new("t0068NQAAAFKYHV9xSZDEThUtClXNzxXH9TLSj/vYcY8mSKa0RxaGw3qNynyAMJ9Ib8UPxzFsbAMIugqPO313BvfiOdmZFTY=").unwrap();
    ```

3. Copy `Dynamsoft\Barcode Reader 6.3\Components\C_C++\Redist\x64\DynamsoftBarcodeReaderx64.dll` to `platforms\win\DynamsoftBarcodeReaderx64.dll`. 

    Copy `Dynamsoft\Barcode Reader 6.3\Components\C_C++\Lib\DBRx64.lib` to `platforms\win\DBRx64.lib`

4. Build the app:

    ```
    cargo build 
    ```

5. Run the app:

    ```
    cargo run <image file>
    ```

    ![rust barcode](https://www.codepool.biz/wp-content/uploads/2018/10/rust-barcode.PNG)