from setuptools import setup, find_packages

setup(
    name='lister',
    version="0.0.1",
    description="Simple lister",
    author="Yury Yurevich",
    author_email="the.pythy@gmail.com",
    url="http://gorod-omsk.ru/blog/pythy/2007/02/18/understanding-eggs-entrypoints/",
    packages=find_packages(),
    scripts=['scripts/listit'],
    zip_safe=True,
    entry_points="""
    [lister.output]
    raw = lister.output:raw_list
    [lister.input]
    dir = lister.input:dir_list
    """
)
