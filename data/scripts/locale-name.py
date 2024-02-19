#! /usr/bin/python3

import gi
gi.require_version('GnomeDesktop', '4.0')
from gi.repository import GnomeDesktop
import sys


locale_name = GnomeDesktop.get_language_from_locale(sys.argv[1], None)
if locale_name is not None:
    print(locale_name, end="")
else:
    print("Unknown Language", end="")