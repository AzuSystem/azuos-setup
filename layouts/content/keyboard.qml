import QtQuick 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.12

Item {
    anchors.fill: parent

    Column {
        id: content
        spacing: 12
        width: parent.width
        anchors.horizontalCenter: parent.horizontalCenter

        Image {
            source: "../../assets/logo.png"
            width: 164
            height: 150
            anchors.horizontalCenter: parent.horizontalCenter
        }

        Text {
            text: "Keyboard Layout"
            font.weight: Font.Medium
            color: "white"
            font.pixelSize: 40
            anchors.horizontalCenter: parent.horizontalCenter
        }

        Text {
            text: "Select your preferred keyboard layout"
            font.weight: Font.Light
            color: "#80ffffff"
            font.pixelSize: 20
            anchors.horizontalCenter: parent.horizontalCenter
        }
    }
}
