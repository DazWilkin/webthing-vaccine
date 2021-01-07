package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/DazWilkin/webthing-vaccine/api"
)

var (
	host = flag.String("host", "", "WebThing host")
	port = flag.Int("port", 0, "WebThing port")
)

func main() {
	flag.Parse()

	if *host == "" {
		log.Fatal("host address required")
	}
	if *port < 0 || *port > 65535 {
		log.Fatal("port must be 0:65535")
	}

	endpoint := fmt.Sprintf("%s:%d", *host, *port)
	client := api.NewClient(&http.Client{}, endpoint)

	for {
		{
			things, err := client.GetThings()
			if err != nil {
				log.Fatal(err)
			}

			for _, thing := range things {
				log.Printf("%+v", thing)
			}
		}
		{
			thing, err := client.GetThing(0)
			if err != nil {
				log.Fatal(err)
			}

			log.Println(thing)
		}
		{
			properties, err := client.GetProperties(1)
			if err != nil {
				log.Fatal(err)
			}

			for property, value := range properties {
				log.Printf("%s: %v\n", property, value)
			}
		}
		{
			property, err := client.GetProperty(1, "longitude")
			if err != nil {
				log.Fatal(err)
			}

			log.Println(property)
		}
		time.Sleep(5 * time.Second)
	}

}
