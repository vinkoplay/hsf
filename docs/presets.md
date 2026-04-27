# Presets
## How to apply?
### Find a preset
To list installed presets in the system
```sh
hsf preset list
```
or you can write **path to file**
### Applying preset

To apply a preset, run
```sh
sudo hsf preset <name/path>
# or
sudo hsf preset load <name/path>
```
for example
```sh
sudo hsf preset base
```

## How to create my own preset?
### Preset syntax
It uses basic hosts file syntax
example:
```
0.0.0.0 example.com
```

but you can use some tricks.
### Include
**C-style include**
It copies the file and pastes.
```
#include <base>
```
### Require
This method is used after **include**. 

> [!NOTE]
> Note that **include** does not work inside a require content.

It fetches the site content and pastes the response into a file.
A full address with the protocol **(http/https) is required**.
Example:
```
#require <https://example.com>
```
Or you can use IP addresses:
```
#require <http://127.0.0.1>
```
### Verification
You can quickly check the result using:
```
hsf preset format <name/path>
```
This will output the text that will be written to the **hosts file** during the load process.
### What else you should know
program uses priority:
1. `~/.config/hsf/presets`
2. `/etc/hsf/presets`

The program **drops root privileges** when working with the network for better security.