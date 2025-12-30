package extractor

import (
	"strings"

	"github.com/angelfreak/ccd/daemon/types"
)

type Fact struct {
	Type       string
	Content    string
	Importance int
}

func ExtractFacts(conv *types.Conversation) []Fact {
	var facts []Fact

	for _, msg := range conv.Messages {
		if msg.Role != "assistant" {
			continue
		}

		content := msg.Content

		// Extract decisions
		if containsAny(content, []string{"decided to", "chose to", "going with", "will use"}) {
			facts = append(facts, Fact{
				Type:       "decision",
				Content:    extractSentence(content, []string{"decided to", "chose to", "going with", "will use"}),
				Importance: 4,
			})
		}

		// Extract blockers
		if containsAny(content, []string{"blocked by", "can't proceed", "error:", "failed to"}) {
			facts = append(facts, Fact{
				Type:       "blocker",
				Content:    extractSentence(content, []string{"blocked by", "can't proceed", "error:", "failed to"}),
				Importance: 5,
			})
		}

		// Extract todos
		if containsAny(content, []string{"TODO:", "need to", "should", "must"}) {
			facts = append(facts, Fact{
				Type:       "todo",
				Content:    extractSentence(content, []string{"TODO:", "need to", "should", "must"}),
				Importance: 3,
			})
		}

		// Extract file changes
		if containsAny(content, []string{"created", "modified", "updated", "deleted"}) &&
			containsAny(content, []string{".ts", ".tsx", ".js", ".jsx", ".go", ".py", ".java"}) {
			facts = append(facts, Fact{
				Type:       "file_change",
				Content:    extractSentence(content, []string{"created", "modified", "updated", "deleted"}),
				Importance: 2,
			})
		}

		// Extract dependencies
		if containsAny(content, []string{"installed", "added dependency", "npm install", "go get"}) {
			facts = append(facts, Fact{
				Type:       "dependency",
				Content:    extractSentence(content, []string{"installed", "added dependency", "npm install", "go get"}),
				Importance: 3,
			})
		}

		// Extract insights
		if containsAny(content, []string{"discovered", "found that", "interesting", "note that"}) {
			facts = append(facts, Fact{
				Type:       "insight",
				Content:    extractSentence(content, []string{"discovered", "found that", "interesting", "note that"}),
				Importance: 3,
			})
		}
	}

	return facts
}

func containsAny(text string, keywords []string) bool {
	lowerText := strings.ToLower(text)
	for _, keyword := range keywords {
		if strings.Contains(lowerText, strings.ToLower(keyword)) {
			return true
		}
	}
	return false
}

func extractSentence(text string, keywords []string) string {
	sentences := strings.Split(text, ".")
	for _, sentence := range sentences {
		if containsAny(sentence, keywords) {
			return strings.TrimSpace(sentence)
		}
	}
	return ""
}
