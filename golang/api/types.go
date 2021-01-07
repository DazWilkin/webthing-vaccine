package api

type Thing struct {
	Context     string     `json:"@context"`
	AtType      []AtType   `json:"@type"`
	Actions     Actions    `json:"actions"`
	Base        string     `json:"base"`
	Description string     `json:"description"`
	Events      Events     `json:"events"`
	HREF        string     `json:"href"`
	ID          string     `json:"id"`
	Links       []Link     `json:"links"`
	Properties  Properties `json:"properties"`
	Title       string     `json:"title"`
}
type AtType struct{}
type Actions struct{}
type Events struct{}
type Link struct {
	HREF string `json:"href"`
	Rel  string `json:"rel"`
}
type Properties map[string]Property
type Property struct {
	AtType      string  `json:"@type"`
	Description string  `json:"description"`
	Links       []Link  `json:"links"`
	Maximum     float32 `json:"maximum,omitempty"`
	Minimum     float32 `json:"minimum,omitempty"`
	Title       string  `json:"title"`
	TType       string  `json:"type"`
}
type PropertiesValues map[string]interface{}
