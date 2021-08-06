import py
mypath = py.magic.autopath().dirpath()

class Directory(py.test.collect.Directory):
    def run(self):
        if self.fspath == mypath:
            return ['doc', 'test']
        return super(Directory, self).run()
