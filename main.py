import sys
import os
from PyQt5.QtGui import QGuiApplication
from PyQt5.QtQml import QQmlApplicationEngine

os.environ["QML_DEBUG"] = "1"

app = QGuiApplication(sys.argv)

engine = QQmlApplicationEngine()
engine.quit.connect(app.quit)
engine.load('layouts/mainbak.qml')

root = engine.rootObjects()[0]
root.showFullScreen()  

sys.exit(app.exec())