import QtQuick 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.12

ApplicationWindow {
    visible: true
    width: 1280
    height: 720
    title: "AzuOS Setup"

    Image {
        id: background
        anchors.fill: parent
        source: "../assets/wallpaper.jpg"
        fillMode: Image.PreserveAspectCrop
        z: -10
    }

    Rectangle {
        id: welcome
        anchors.centerIn: parent
        width: 650
        height: parent.height * 0.65
        color: "#900D0514"
        border.width: 1
        border.color: "#33FFFFFF"
        radius: 7
        layer.enabled: true
        layer.effect: DropShadow {
            anchors.centerIn: parent
            source: welcome
            color: "#80000000"
            radius: 35
            spread: 0.0
            samples: 64
            horizontalOffset: 0
            verticalOffset: 12
        }

        Image {
            anchors.horizontalCenter: parent.horizontalCenter
            y: 75
            source: "../assets/logo.png"
            width: 164
            height: 150
        }


        Text {
            text: "Welcome to AzuOS"
            anchors.centerIn: parent
            color: "white"
            font.pixelSize: 24
        }
    }

    FastBlur {
        anchors.fill: welcome
        source: background
        radius: 60
        transparentBorder: false
        z: -5
    }
}