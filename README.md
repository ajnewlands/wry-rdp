# RDP-WRY - A proof of concept webview based RDP client

This repository began as an effort to learn the workings of the libfreerdp2 library and to test the use of the WRY webview library from the Tauri project.

The combined result is a basic and somewhat incomplete RDP client.

The front end is implemented as a react/redux app presented by the WRY webview.  The back end (including the RDP client) is a rust binary. IPC between the two halves is primarily via websocket to avoid running into webview gotchas. 

The program builds to a single file binary (not counting shared libraries e.g. for libfreerdp2 and the platform native webview) with the javascript front end baked in.

There are no plans to complete the functionality and productionize the client as this was a learning exercise, which is preserved here for future reference (e.g. in the event I decide to write my memoirs).