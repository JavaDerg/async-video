package main

import (
	"fmt"
	"github.com/gomodule/redigo/redis"
	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
	"net/http"
	"os"
)

var upgrader = websocket.Upgrader {
	ReadBufferSize: 1024,
	WriteBufferSize: 1024,
}
var rdb *redis.Conn

func main() {
	initRedis()
	initWebserver()
}

func initRedis() {
	r, err := redis.DialURL(getOsArg("REDIS_URL"))
	if err != nil {
		fmt.Println("Cant connect to redis: ", err)
		os.Exit(1)
	}
	rdb = &r
}

func initWebserver() {
	r := mux.NewRouter()

	r.HandleFunc("/", websocketRoot)

	fmt.Println("Webserver Running")
	err := http.ListenAndServe(getOsArg("WS_INTERFACE"), r)
	if err != nil {
		fmt.Println("Webserver crashed: ", err)
		os.Exit(1)
	}
}

func websocketRoot(w http.ResponseWriter, r *http.Request) {
	_, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
}

// This function will exit on failure
func getOsArg(key string) string {
	env := os.Getenv(key)
	if len(env) == 0 {
		fmt.Println("Environment variable `", key, "` not set")
		os.Exit(1)
	}
	return env
}