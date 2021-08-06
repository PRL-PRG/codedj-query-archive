import cv2
import numpy as np

def getGripperCenter(img):

    def getDistance(X, Y):
        dx = X[0] - Y[0]
        dy = X[1] - Y[1]
        return dx * dx + dy * dy

    hsvImg = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)
    
    lower_red_hue_range = cv2.inRange(img, (0, 0, 150), (100, 100, 255))
    upper_red_hue_range = cv2.inRange(hsvImg, (160, 100, 50), (185, 255, 255))
    red_hue_image = cv2.addWeighted(lower_red_hue_range, 1.0, upper_red_hue_range, 1.0, 0.0)
    redImg = red_hue_image.copy()
    
    lower_yel_hue_range = cv2.inRange(img, (0, 150, 0), (100, 255, 100))
    upper_yel_hue_range = cv2.inRange(hsvImg, (20, 120, 0), (115, 255, 180))
    yel_hue_image = cv2.addWeighted(lower_yel_hue_range, 1.0, upper_yel_hue_range, 1.0, 0.0)
    yelImg = yel_hue_image.copy()
    
    redImg = cv2.medianBlur(redImg, 5)
    yelImg = cv2.medianBlur(yelImg, 5)

    redImg = cv2.GaussianBlur(redImg, (9, 9), 0)
    yelImg = cv2.GaussianBlur(yelImg, (9, 9), 0)

    redOut = cv2.connectedComponentsWithStats(redImg, 8, cv2.CV_32S)
    cenRed = redOut[3] 
    print (len(cenRed))

    yelOut = cv2.connectedComponentsWithStats(yelImg, 8, cv2.CV_32S)
    cenYel = yelOut[3] 
    print (len(cenYel))

    minD = -1
    result = (0, 0)
    for a in cenRed:
        for b in cenYel:
            d = getDistance(a, b)
            if (minD < 0 or d < minD):
                minD = d
                result = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)

    return list(map(int, result))

