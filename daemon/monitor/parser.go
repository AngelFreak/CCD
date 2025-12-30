package monitor

import (
	"encoding/json"
	"strings"
)

type Parser struct{}

func NewParser() *Parser {
	return &Parser{}
}

func (p *Parser) Parse(data string) (*Conversation, error) {
	var conv Conversation

	// Try to parse as JSON first
	if err := json.Unmarshal([]byte(data), &conv); err != nil {
		// If JSON parsing fails, parse as plain text
		conv = p.parseText(data)
	}

	return &conv, nil
}

func (p *Parser) parseText(data string) Conversation {
	conv := Conversation{
		Messages: []Message{},
	}

	// Simple line-based parsing
	lines := strings.Split(data, "\n")
	var currentMessage *Message

	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		// Detect role markers
		if strings.HasPrefix(line, "User:") || strings.HasPrefix(line, "user:") {
			if currentMessage != nil {
				conv.Messages = append(conv.Messages, *currentMessage)
			}
			currentMessage = &Message{
				Role:    "user",
				Content: strings.TrimPrefix(strings.TrimPrefix(line, "User:"), "user:"),
			}
		} else if strings.HasPrefix(line, "Assistant:") || strings.HasPrefix(line, "assistant:") {
			if currentMessage != nil {
				conv.Messages = append(conv.Messages, *currentMessage)
			}
			currentMessage = &Message{
				Role:    "assistant",
				Content: strings.TrimPrefix(strings.TrimPrefix(line, "Assistant:"), "assistant:"),
			}
		} else if currentMessage != nil {
			currentMessage.Content += "\n" + line
		}
	}

	if currentMessage != nil {
		conv.Messages = append(conv.Messages, *currentMessage)
	}

	return conv
}

func (p *Parser) CountTokens(conv *Conversation) int {
	// Simple token estimation: ~4 characters per token
	total := 0
	for _, msg := range conv.Messages {
		total += len(msg.Content) / 4
	}
	return total
}
