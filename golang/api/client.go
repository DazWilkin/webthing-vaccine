package api

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

// Client is a type that represents a Webthing Vaccine client
type Client struct {
	client *http.Client
	host   string
}

// NewClient is a function creates a new Client
func NewClient(c *http.Client, host string) *Client {
	log.Printf("[api:NewClient] Creating client: %s", host)
	return &Client{
		client: c,
		host:   host,
	}
}

func (c *Client) GetThings() ([]Thing, error) {
	resp, err := c.client.Get(fmt.Sprintf("http://%s/", c.host))
	log.Println("[api:GetThings] Entered")
	if err != nil {
		return nil, err
	}
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	result := []Thing{}
	err = json.Unmarshal(body, &result)
	if err != nil {
		return nil, err
	}
	return result, nil
}
func (c *Client) GetThing(id int) (Thing, error) {
	log.Printf("[api:GetThing:%d] Entered", id)
	resp, err := c.client.Get(fmt.Sprintf("http://%s/%d", c.host, id))
	if err != nil {
		return Thing{}, err
	}
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return Thing{}, err
	}
	result := Thing{}
	err = json.Unmarshal(body, &result)
	if err != nil {
		return Thing{}, err
	}
	return result, nil
}
func (c *Client) GetProperties(thingID int) (PropertiesValues, error) {
	log.Printf("[api:GetProperties:%d] Entered", thingID)
	resp, err := c.client.Get(fmt.Sprintf("http://%s/%d/properties", c.host, thingID))
	if err != nil {
		return nil, err
	}
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	result := make(PropertiesValues)
	err = json.Unmarshal(body, &result)
	if err != nil {
		return nil, err
	}
	return result, nil
}
func (c *Client) GetProperty(thingID int, property string) (PropertiesValues, error) {
	log.Printf("[api:GetProperty:%d] Entered: %s", thingID, property)
	resp, err := c.client.Get(fmt.Sprintf("http://%s/%d/properties/%s", c.host, thingID, property))
	if err != nil {
		return nil, err
	}
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	result := make(PropertiesValues)
	err = json.Unmarshal(body, &result)
	if err != nil {
		return nil, err
	}
	return result, nil
}
