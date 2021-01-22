package main

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"github.com/gomodule/redigo/redis"
	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
	"github.com/gorilla/websocket"
	"log"
	"net/http"
	"os"
	"async-video/room"
)

var upgrader = websocket.Upgrader {
	ReadBufferSize: 1024,
	WriteBufferSize: 1024,
}
var store = sessions.NewCookieStore([]byte(getOsArg("SESSION_KEY")))
var rdb *redis.Conn

func main() {
	go room.RunScheduler()
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

	uKey, save := getUKey(session)

	key, present := vars["key"]

	if len(key) == 0 || !present {
		http.Error(w, "No key", http.StatusBadRequest)
		return
	}

	sesKey := session.Values["key"]
	if sesKey != key {
		go room.Disconnect(uKey, key)
		// Disconnect old stream if present
		session.Values["key"] = key
	}

	if save {
		if err := session.Save(r, w); err != nil {
			http.Error(w, fmt.Sprint(err), http.StatusInternalServerError)
		}
	}
}

func getUKey(ses *sessions.Session) (string, bool) {
	key, present := ses.Values["ukey"]
	if len(key.(string)) == 0 || !present {
		b := make([]byte, 16)
		_, err := rand.Read(b)
		if err != nil {
			log.Fatal(err)
		}
		ses.Values["ukey"] = hex.EncodeToString(b)
		return ses.Values["ukey"].(string), true
	}

	return key.(string), false
}

func startStream(w http.ResponseWriter, r *http.Request) {
	cann, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	room.Schedule(cann)
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