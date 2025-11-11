import QtQuick 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.12

ApplicationWindow {
    visible: true
    width: 1280
    height: 720
    title: "AzuOS Setup"
    font.family: "Inter"
    // visibility: Window.FullScreen

    Image {
        id: background
        anchors.fill: parent
        source: "../assets/wallpaper.jpg"
        fillMode: Image.PreserveAspectCrop
        z: -10
    }

    Rectangle {
        color: "#33FFFFFF"
        width: 1
        height: parent.height
        x: 595
    }

    Rectangle {
        id: panel
        anchors.left: parent.left
        width: 595
        height: parent.height
        color: "#900D0514"
        layer.enabled: true
        layer.effect: DropShadow {
            anchors.left: parent.left
            source: panel
            color: "#80000000"
            radius: 35
            spread: 0.0
            samples: 64
            horizontalOffset: 7
            verticalOffset: 12
        }

        Loader {
            anchors.fill: parent
            id: keyboardLayout
            source: "content/keyboard.qml"
        }

        Row {
            anchors.bottom: parent
            z: 1
            Text {
                text: "Copyright @ AzuSystem 2025 - AzuOS v2025.11\nhttps://github.com/AzuSystem"
                font.weight: Font.Light
                color: "#80ffffff"
                font.pixelSize: 12
            }

           Button {
                background: Rectangle {
                    color: "#00ffffff"
                    border.color: "#80ffffff"
                    border.width: 1
                    radius: 24
                }

                contentItem: Text {
                    text: "Previous"
                    anchors.centerIn: parent
                    color: "#80ffffff"
                    font.pixelSize: 12
                }
            }

           Button {
                background: Rectangle {
                    color: "#00ffffff"
                    border.color: "#80ffffff"
                    border.width: 1
                    radius: 24
                }

                contentItem: Text {
                    text: "Next"
                    anchors.centerIn: parent
                    color: "#80ffffff"
                    font.pixelSize: 12
                }
            }

            // Button {
            //     width: 100
            //     height: 30

            //     background: Rectangle {
            //         color: "#00ffffff"
            //         border.color: "#80ffffff"
            //         border.width: 1
            //         radius: 24
            //     }

            //     contentItem: Text {
            //         text: "Previous"
            //         anchors.centerIn: parent
            //         color: "#80ffffff"
            //         font.pixelSize: 12
            //     }
            // }
        }
    }

    FastBlur {
        anchors.fill: panel
        source: background
        radius: 60
        transparentBorder: false
        z: -5
    }
}