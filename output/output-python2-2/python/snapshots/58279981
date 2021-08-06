#!/usr/bin/python

import doctest

packages = {}

provides = {}


def process_package_list(s):
    """
    >>> process_package_list("a (>= 0.5.8), b (>= 2.3)")
    (['a', 'b'], [])
    >>> process_package_list("a, b, c | d")
    (['a', 'b'], [['c', 'd']])
    """
    def strip_version(item):
        if item.endswith(")"):
            item = item.split(" (", 1)[0]
        return item

    packages = []
    alternatives = []
    
    for item in s.split(", "):
        if " | " in item:
            alternative = []
            for name in item.split(" | "):
                alternative.append(strip_version(name))
            alternatives.append(alternative)
        else:
            packages.append(strip_version(item))
    return packages, alternatives


doctest.testmod()


for line in open("/var/lib/dpkg/status"):
    if line.startswith("Package: "):
        package_name = line[len("Package: "):-1]
        if package_name in packages:
            raise Exception("duplicate package name %s" % package_name)
        package = {"depends": [],
                   "depends_alternatives": [],
                   "rdepends": [],
                   "recommends": [],
                   "recommends_alternatives": [],
                   "rrecommends": []}
        packages[package_name] = package
    elif line.startswith("Status: "):
        if not line.endswith(" installed\n"):
            del packages[package_name]
    elif line.startswith("Provides: "):
        if package_name in packages:
            for provide in process_package_list(line[len("Provides: "):-1])[0]:
                provides.setdefault(provide, []).append(package_name)
    elif line.startswith("Depends: "):
        package["depends"], package["depends_alternatives"] = \
            process_package_list(line[len("Depends: "):-1])
    elif line.startswith("Recommends: "):
        package["recommends"], package["recommends_alternatives"] = \
            process_package_list(line[len("Recommends: "):-1])


for provide in provides:
    for name in provides[provide]:
        if name in packages:
            provides[provide] = name
            break
    else:
        raise Exception("multiple provides %s %s" % (provide,
                                                     provides[provide]))


def resolve_alternatives(name):
    package = packages[name]
    for type_ in ["depends", "recommends"]:
        for dependancy in package["%s_alternatives" % type_]:
            for alternative in dependancy:
                if alternative in packages:
                    package[type_].append(alternative)
                    break
                if alternative in provides:
                    package[type_].append(provides[alternative])
                    break
#            else:
#                print "unresolved %s %s" % (name, dependancy)
#                raise Exception("unresolved %s %s" % (name, dependancy))


def set_reverse_depends(package_name):
    for type_ in ["depends", "recommends"]:
        for name in packages[package_name][type_]:
            if name in provides:
                name = provides[name]
            if name in packages:
                packages[name]["r%s" % type_].append(package_name)


for name in packages:
    resolve_alternatives(name)
    set_reverse_depends(name)


def deps_generator(name, keys, seen):
    depends = []
    for key in keys:
        depends.extend(packages[name][key])
    for depend in depends:
        if depend in provides:
            depend = provides[depend]
        if depend not in seen:
            seen.add(depend)
            yield depend
            for sub_depend in deps_generator(depend, keys, seen):
                yield sub_depend


installed = packages.keys()

def get_recommended_deps(names):
    deps = set(names)
    for name in names:
        deps.update(set(packages[name]["depends"]))
        deps.update(set(packages[name]["recommends"]))
    return deps

standard_packages = get_recommended_deps(["ubuntu-desktop", "ubuntu-standard",
                                          "ubuntu-minimal"])
for package in list(standard_packages):
    try:
        installed.remove(package)
    except ValueError:
        sys.stderr.write('standard package "%s" not installed\n' % package)
        standard_packages.remove(package)

seen = set(standard_packages)
for package in standard_packages:
    for name in deps_generator(package, ("depends",), seen):
        try:
            installed.remove(name)
        except ValueError:
            print "couldn't find %s" % name, name in packages, name in seen
            raise


installed.sort()
for name in installed:
    if len(packages[name]["rdepends"]) == 0:
        print name, packages[name]["rrecommends"]


#print packages["ubuntu-desktop"].depends

