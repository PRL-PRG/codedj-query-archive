#!/usr/bin/python

import sys
import time


def main(argv):
    try:
        input_time = int(argv[1])
    except:
        sys.exit("invalid arguments %s" % argv[1:])
    acpi_time = time.strftime("%F %T\n", time.gmtime(input_time))
    open("/proc/acpi/alarm", "w").write(acpi_time)


if __name__ == "__main__":
    main(sys.argv)

