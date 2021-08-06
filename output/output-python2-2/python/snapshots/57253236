import _pybind

s = _pybind.settings()
print s.get("Exception", "Format")
assert s.get("Exception", "Format") == "None"

s.set("Exception", "Format", "HTML")
assert s.get("Exception", "Format") == "HTML"

