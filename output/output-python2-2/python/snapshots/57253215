import _settings

def test():
    """
        This function will create only local settings which will disappear after it terminates
    """
    s = _settings.settings()
    print s.get("Exception", "Format")
    assert s.get("Exception", "Format") == '"None"'

    s.set("Exception", "Format", "HTML")
    assert s.get("Exception", "Format") == '"HTML"'

test()
n = _settings.settings()
assert n.get("Exception", "Format") == '"None"'

# Load an ini file and make sure we can read a value from it
n.file("Cpp/fost-settings-py/test.ini")
assert n.get("Test ini", "A value") == 'true'

# Make sure grabbing non-existant value throws
try:
    n.get("Not a section name", "Not a value name")
    assert False
except:
    pass
