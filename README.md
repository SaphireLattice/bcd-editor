A small utility to dump and eventually edit Windows BCD (Boot Configuration Data) registry hive, in a cross-platform way.

Made it a while ago but I'm not sure if it even works well. Play around with it, honestly

`known_symbols.json` is an attempt to make name resolution data driven

## Notes

Windows has moved from `boot.ini` a long while ago and now uses a registry file (yes, that registry) as a way to store the configuration, called "Boot Configuration Data". Sits in the bootloader partition as `/EFI/Microsoft/Boot/BCD`. It's an odd mix of normal registry style entries mixed with binary blobs.

If you have any notes on how to actually decode the blobs, but this code is my best effort for it.

## Why?

Because I fucked up my windows bootloader when copying the partition over to another disk and didn't want to edit the IDs, so I edited the bootloader settings instead. Took way longer than downloading and booting the recovery image, but at least I avoided that AND learn something new, woo!

I still had to hexedit the binary data in the end and use `chntpw` (which is actually capable of working as registry editor!), as I struggled to figure out how the binary blobs are structured, but making this helped me figure out where the partition GUID is residing within them..

# Links

Huge thanks for information and IDs from:
- https://www.geoffchappell.com/notes/windows/boot/bcd/objects.htm
- http://www.mistyprojects.co.uk/documents/BCDEdit/
- https://github.com/kupiakos/pybcd for the information about Device elements format and test data
- https://doxygen.reactos.org/da/db4/bl_8h_source.html#l00245 - I am not sure where this is from, but that is helpful... -ish

# Support

This is where a kofi link could have been if I coul use it!

(No support, warranty or anything of this kind is provided with this, the MIT license should be read in same tone as if this util was licensed under WTFPL)
