package main

import (
	"fmt"
	"github.com/gomodule/redigo/redis"
	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
	"github.com/gorilla/sessions"
	"net/http"
	"os"
)

var upgrader = websocket.Upgrader {
	ReadBufferSize: 1024,
	WriteBufferSize: 1024,
}
var store = sessions.NewCookieStore([]byte(getOsArg("SESSION_KEY")))
var rdb *redis.Conn

func main() {
	go run_sched()
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

	r.HandleFunc("/room/enter/{}", enterRoom)
	r.HandleFunc("/room/{}/stream", startStream)

	fmt.Println("Webserver Running")
	err := http.ListenAndServe(getOsArg("WS_INTERFACE"), r)
	if err != nil {
		fmt.Println("Webserver crashed: ", err)
		os.Exit(1)
	}
}

func enterRoom(w http.ResponseWriter, r *http.Request) {
	session, _ := store.Get(r, "asv-store")
	vars := mux.Vars(r)

	key, present := vars["key"]

	if len(key) == 0 || !present {
		http.Error(w, "No key", http.StatusBadRequest)
		return
	}

	ses_key := session.Values["key"]
	if ses_key != key {
		// Disconnect old stream if present
		session.Values["key"] = key
	}
}

func startStream(w http.ResponseWriter, r *http.Request) {
	cann, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	*scheduler <- cann
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