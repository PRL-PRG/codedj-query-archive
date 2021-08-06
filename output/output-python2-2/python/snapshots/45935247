textual = {}
textual[1] = "one"
textual[2] = "two"
textual[3] = "three"
textual[4] = "four"
textual[5] = "five"
textual[6] = "six"
textual[7] = "seven"
textual[8] = "eight"
textual[9] = "nine"
textual[10] = "ten"
textual[11] = "eleven"
textual[12] = "twelve"
textual[13] = "thirteen"
textual[14] = "fourteen"
textual[15] = "fifteen"
textual[16] = "sixteen"
textual[17] = "seventeen"
textual[18] = "eighteen"
textual[19] = "nineteen"
textual[20] = "twenty"
textual[30] = "thirty"
textual[40] = "forty"
textual[50] = "fifty"
textual[60] = "sixty"
textual[70] = "seventy"
textual[80] = "eighty"
textual[90] = "ninety"

def get_textual(i):
  if textual.has_key(i):
    return textual[i]

  if 10 < i < 100:
    num = int(str(i)[0] + "0")
    return get_textual(num) + get_textual(i - num)
  elif 100 <= i < 1000:
    num = int(str(i)[0] + "00")
    n = get_textual(i / 100) + "hundred"
    if i - num > 0:
      n += "and" + get_textual(i - num)
    return n
  else:
    return "onethousand"

print len("".join([get_textual(i) for i in range(1, 1001)]))
