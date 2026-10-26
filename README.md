# copy-send-paste (csp)

Utility tool for local text sharing through Wi-Fi, written in Rust.

## What is `copy-send-paste`?

`copy-send-paste` (csp) is a lightweight utility tool that allows users to share text between devices on the same Wi-Fi network. It's not a messaging app. Its main purpose is only for sharing text content between devices. It was created for those moments where you have a text/URL on a device, but you need it on another device.

## How does it work?

`csp` works with the device's clipboard or with files. There are 2 main operations you can do with `csp`: **sending** and **receiving** text. You need to have `csp` installed (or running in any way) on all devices that you want to share text with. `csp` is a run-and-done program. You run it once, it does its job and it's fnished. There are no background processes running all the time in the background.

### Receiving text

Receiving text with `csp` involves calling the program simply with `csp rcv <receive_type>`. "rcv" stands for "receive" and `<receive_type>` can be either `text` or `file`. 

`csp rcv text` will take whatever is being received and will copy it to the device's clipboard, overwriting whatever was in the clipboard before.

`csp rcv file <file_path>` will take whatever text has been received into a text file at the specified `<file_path>`.

### Sending text

Sending text with `csp` can be done in several ways:

1. `csp send clip` -> sends the text from the clipboard
2. `csp send file <file_path>` -> sends the text from the file at the specified file path. It only sends the contents of the file as a byte stream, not the file itself.

Whenever the **send** command is called, it will prompt the user to select what is the target device (if there are more devices waiting in **receive mode**).
