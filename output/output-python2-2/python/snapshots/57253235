import _pybind

def test():
    s = _pybind.settings()
    print s.get("Exception", "Format")
    assert s.get("Exception", "Format") == "None"

    s.set("Exception", "Format", "HTML")
    assert s.get("Exception", "Format") == "HTML"

test()
n = _pybind.settings()
assert n.get("Exception", "Format") == "None"
