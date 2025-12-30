package types

import "time"

type Conversation struct {
	Messages []Message `json:"messages"`
}

type Message struct {
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}
