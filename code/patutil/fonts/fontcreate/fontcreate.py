import json

f = open("fonts/fontcreate/font.json")
data = json.load(f)
f.close()

f = open('fonts/font.dat', 'wb')
for charcode in range(0,255):
    d = str(charcode)
    if not d in data:
        data[d] = [0]*16
    if len(data[d]) != 16:
        print("Height != 16 for ", d)
    for i in reversed(data[d]):
        if len(format(i, 'b')) > 16:
            print("Warning ", i , "contains to many bits")
        i = int(format(i, 'b')[-16:],2)
        
        b = i.to_bytes(2, "little")
        p = format(b[0], 'b')[::-1]
        while len(p) < 8:
            p += '0'
        f.write(int(p[::-1],2).to_bytes(1, 'little'))
        p = format(b[1], 'b')[::-1]
        while len(p) < 8:
            p += '0'
        f.write(int(p[::-1],2).to_bytes(1, 'little'))

print("total:", len(data))

f.close()