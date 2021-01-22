package room

import "sync"

var rooms = Rooms {}

type Room struct {
	key string
	name string
	owner string
	perms []string
	password string
}

type Rooms struct {
	rooms sync.Map
}

type RoomKeeper struct {
	lock sync.RWMutex
	inner Room
}

func (r *Rooms) GetOrMake(uKey string, rKey string) (*RoomKeeper, bool) {
	room, exists := rooms.rooms.Load(rKey)
	newRoom := false
	if !exists {
		room = Room {
			key: rKey,
			name: "Room",
			owner: uKey,
		}
		newRoom = true
	}
	return room.(*RoomKeeper), newRoom
}