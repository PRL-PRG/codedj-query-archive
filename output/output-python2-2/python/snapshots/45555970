from setuptools import setup, find_packages

setup(
    name='pylonsex',
    version="",
    #description="",
    #author="",
    #author_email="",
    #url="",
    install_requires=["Pylons>=0.9.3"],
    packages=find_packages(),
    include_package_data=True,
    test_suite = 'nose.collector',
    package_data={'pylonsex': ['i18n/*/LC_MESSAGES/*.mo']},
    entry_points="""
    [paste.app_factory]
    main=pylonsex:make_app
    [paste.app_install]
    main=paste.script.appinstall:Installer
    """,
)