extern crate clipboard_win;

use std::str;

use clipboard_win::Clipboard;
use clipboard_win::formats;
use clipboard_win::raw;

#[test]
fn seq_num() {
    let result = raw::seq_num();

    assert!(result.is_some());
    assert!(result.unwrap() != 0);
}

#[test]
fn set_data() {
    let format = formats::CF_TEXT;
    let text = "For my waifu!\0"; //For text we need to pass C-like string
    let data = text.as_bytes();
    let mut buff = [0u8; 52];
    let mut small_buff = [0u8; 4];

    let clipboard = Clipboard::new();
    assert!(clipboard.is_ok());
    let clipboard = clipboard.unwrap();

    let result = clipboard.empty();
    assert!(result.is_ok());
    let format_num = clipboard.enum_formats().count();
    assert_eq!(format_num, 0);
    let result = Clipboard::count_formats();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let seq_num_before = Clipboard::seq_num();

    let result = clipboard.set(format, data);
    assert!(result.is_ok());

    let seq_num_after = Clipboard::seq_num();
    assert!(seq_num_before != seq_num_after);

    let result = clipboard.get(format, &mut buff);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, data.len());
    let result = str::from_utf8(&buff[..result]).unwrap();
    assert_eq!(text, result);

    let result = clipboard.get(format, &mut small_buff);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, small_buff.len());
    let result = str::from_utf8(&buff[..result]).unwrap();
    assert_eq!(&text[..small_buff.len()], result);

}
