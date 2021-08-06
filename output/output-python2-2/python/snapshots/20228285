import time
import datetime
import re
import subprocess

# def update():
#     date = datetime.date(2008, 11, 18)
#     return another_date

# print(update())

def readFile():
    with open("generate.py") as f:
        return f.read()

# print(newStr)

def writeFile(newStr):
    with open("generate.py", "w") as f:
        f.write(newStr)

def process(new_date):
    string = readFile()
    newStr = re.sub(r'datetime\.date\([0-9]+, [0-9]+, [0-9]+\)', repr(new_date), string)
    writeFile(newStr)

    currentTime = datetime.datetime.combine(new_date, datetime.time()).strftime("%c") # .strftime('%a %b %-d %H:%M:%S ')
    subprocess.check_output(['env', f'GIT_COMMITTER_DATE={currentTime}', 'git', 'commit', f'--date={currentTime}', '-am', 'message'])

def main():
    start_date = datetime.date(2008, 11, 18)
    end_date = datetime.date.today()
    while start_date < end_date:
        diff_date = datetime.timedelta(days=1)
        start_date = start_date + diff_date
        process(start_date)

main()