import stepic
from PIL import Image

img = Image.open("upz.png")
hidden_message = stepic.decode(img)
print(hidden_message)
