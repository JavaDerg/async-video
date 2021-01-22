package room

import (
	"fmt"
	"github.com/gorilla/websocket"
)

var scheduler *chan *websocket.Conn

type Handshake struct {
	room string
	auth_key string
}

func RunScheduler() {
	ch := make(chan *websocket.Conn)
	scheduler = &ch
	hs_ch := make(chan struct {
		ws       *websocket.Conn
		hanshake Handshake
	})
	for true {
		select {
		case conn := <-*scheduler:
			go awaitHandshake(conn, &hs_ch)
		case msg := <- hs_ch:
			_ = msg
		}
	}
}

func Schedule(conn *websocket.Conn) {
	*scheduler <- conn
}

func Disconnect(ukey string, rkey string) {

}

func awaitHandshake(ws *websocket.Conn, ch *chan struct {
	ws       *websocket.Conn
	hanshake Handshake
}) {
	var handshake Handshake
	err := ws.ReadJSON(&handshake)
	if err != nil {
		fmt.Println("Handshake failed: ", err)
		return
	}
	*ch <- struct {
		ws       *websocket.Conn
		hanshake Handshake
	}{ws, handshake}
}
