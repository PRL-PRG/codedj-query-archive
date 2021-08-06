import _settings

def test():
    s = _settings.settings()
    print s.get("Exception", "Format")
    assert s.get("Exception", "Format") == "None"

    s.set("Exception", "Format", "HTML")
    assert s.get("Exception", "Format") == "HTML"

test()
n = _settings.settings()
assert n.get("Exception", "Format") == "None"
