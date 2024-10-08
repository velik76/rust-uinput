#
# Parses the events.rs file to get the KEY_* values from it into rust enum in keys_enum.rs
#
import re
import os

cwd = os.path.dirname(os.path.realpath(__file__))

with open(os.path.join(cwd, 'events.rs')) as f:
    lines = f.readlines()

f = open(os.path.join(cwd, "../src/keys_enum.rs"), "w")
f.write("/// AUTOMATIC GENERATED. DO NOT EDIT!\n\n")
f.write("use serde::{Deserialize, Serialize};\n\n")
f.write("#[allow(non_camel_case_types)]\n\n")
f.write("#[derive(Serialize, Deserialize, Debug, Clone, Copy)]\n")
# f.write("#[repr(u32)]\n")
f.write("pub enum Keys {\n")
f.write("\tBTN_LEFT = 0x110,\n")
f.write("\tBTN_RIGHT = 0x111,\n")

for line in lines:
    # Replace multiple spaces by only one space
    line = re.sub(r" {2,}", " ", line)
    parts = line.split(" ")
    if len(parts) < 6:
        continue
    if parts[0] == "pub" and parts[2].startswith("KEY_"):
        key = parts[2]
        key = key.replace(":", "")
        value = parts[5]
        value = value.replace(";", "")
        value = value.replace("\n", "")

        # Do not generate in enums values referenced onto another
        if not value.startswith("0x"):
            try:
                int(value)
            except ValueError:
                print(f"Skip: {line}")
                continue

        f.write(f"\t{key} = {value},\n")

f.write("}\n")
f.close()
