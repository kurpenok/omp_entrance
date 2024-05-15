import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

import app 1.0

Window {
    title: qsTr("Test app")
    visible: true
    width: 640
    height: 480

    TestObject {
        id: test_object
    }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        spacing: 10

        Button {
            text: qsTr("Test button")

            onClicked: test_object.testButton()
        }
    }
}
