# WebThings Vaccine Distribution Simulator

[Akri Scenario: Distribution of Vaccines](https://docs.google.com/document/d/1zNWUzaTuvTPcHsH_GNlNh6p1XN2QDZF5sfcKuFTknjU/edit#heading=h.9m8u7fba07v)

![Things](./images/things.png)

Refrigerators (currently) have a temperature sensor:

![Refrigerator](./images/refrigerator.png)

Truck have latitude and longitude:

![Truck](./images/truck.png)

## mDNS|DNS-SD

WebThings devices are published as `_webthing._tcp` services.

## Gateway

```bash
docker run \
--rm \
--interactive --tty \
--name webthings-gateway \
--env=TZ=America/Los_Angeles \
--volume=${PWD}/webthings:/home/node/.webthings \
--network=host \
--log-opt max-size=1m \
--log-opt max-file=10 \
webthingsio/gateway:latest
```

## Run

```bash
cargo run
```

Running with `RUST_LOG=debug`, you'll get the random walk details of the refrigerator temperature and truck latitude|longitude:

```bash
RUST_LOG=debug cargo run
```

Yields:

```console
[2021-01-07T00:54:56Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00003051757812
[2021-01-07T00:54:57Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00008392333984
[2021-01-07T00:54:58Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (47.12375259399414,-122.4013442993164)
[2021-01-07T00:54:58Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00000762939453
[2021-01-07T00:54:59Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00004577636719
[2021-01-07T00:55:00Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99994659423828
[2021-01-07T00:55:00Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (48.146942138671875,-122.58728790283203)
[2021-01-07T00:55:01Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99998474121094
[2021-01-07T00:55:02Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00007629394531
[2021-01-07T00:55:03Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (48.414024353027344,-122.95532989501953)
[2021-01-07T00:55:03Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99998474121094
[2021-01-07T00:55:04Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00005340576172
[2021-01-07T00:55:05Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99992370605469
[2021-01-07T00:55:05Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (47.09706115722656,-121.50337219238281)
[2021-01-07T00:55:06Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00006103515625
[2021-01-07T00:55:07Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00000762939453
[2021-01-07T00:55:08Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (47.62933349609375,-122.49585723876953)
[2021-01-07T00:55:08Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99993133544922
[2021-01-07T00:55:09Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99993896484375
[2021-01-07T00:55:10Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00005340576172
[2021-01-07T00:55:10Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (48.0540885925293,-122.32648468017578)
[2021-01-07T00:55:11Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00006866455078
[2021-01-07T00:55:12Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00005340576172
[2021-01-07T00:55:13Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (47.66055679321289,-122.80746459960938)
[2021-01-07T00:55:13Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99994659423828
[2021-01-07T00:55:14Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -80.00007629394531
[2021-01-07T00:55:15Z TRACE vaccine_things] [main::thread::Refrigerator] Refrigerator Temp(c): -79.99990844726562
[2021-01-07T00:55:15Z TRACE vaccine_things] [main::thread::Truck] Truck's location: (47.87299728393555,-121.82833099365234)
```

Then:

```bash
curl --silent http://localhost:8888/ \
| jq .
```

Yields:

```JSON
[
  {
    "@context": "https://iot.mozilla.org/schemas",
    "@type": [],
    "actions": {},
    "base": "http://localhost:8888/0",
    "description": "IIoT Vaccine Refrigerator",
    "events": {},
    "href": "/0",
    "id": "refrigerator",
    "links": [
      {
        "href": "/0/properties",
        "rel": "properties"
      },
      {
        "href": "/0/actions",
        "rel": "actions"
      },
      {
        "href": "/0/events",
        "rel": "events"
      },
      {
        "href": "ws://localhost:8888/0",
        "rel": "alternate"
      }
    ],
    "properties": {
      "temperature": {
        "@type": "TempProperty",
        "description": "Temperature of the Refrigerator",
        "links": [
          {
            "href": "/0/properties/temperature",
            "rel": "property"
          }
        ],
        "title": "Temperature",
        "type": "number"
      }
    },
    "security": "nosec_sc",
    "securityDefinitions": {
      "nosec_sc": {
        "scheme": "nosec"
      }
    },
    "title": "Vaccine Refrigerator"
  },
  {
    "@context": "https://iot.mozilla.org/schemas",
    "@type": [],
    "actions": {},
    "base": "http://localhost:8888/1",
    "description": "Truck",
    "events": {},
    "href": "/1",
    "id": "truck",
    "links": [
      {
        "href": "/1/properties",
        "rel": "properties"
      },
      {
        "href": "/1/actions",
        "rel": "actions"
      },
      {
        "href": "/1/events",
        "rel": "events"
      },
      {
        "href": "ws://localhost:8888/1",
        "rel": "alternate"
      }
    ],
    "properties": {
      "latitude": {
        "@type": "LatitudeProperty",
        "description": "Latitude of the Truck",
        "links": [
          {
            "href": "/1/properties/latitude",
            "rel": "property"
          }
        ],
        "title": "Latitude",
        "type": "number"
      },
      "longitude": {
        "@type": "LongitudeProperty",
        "description": "Longitude of the Truck",
        "links": [
          {
            "href": "/1/properties/longitude",
            "rel": "property"
          }
        ],
        "title": "Longitude",
        "type": "number"
      }
    },
    "security": "nosec_sc",
    "securityDefinitions": {
      "nosec_sc": {
        "scheme": "nosec"
      }
    },
    "title": "Truck"
  }
]
```

## Client (Golang)

```bash
HOST="localhost"
PORT="8888"
```

Either:

```bash
go run ./golang/cmd --host=${HOST} --port=${PORT}
```

Or:

```bash
TAG="$(git rev-parse HEAD)"

docker run \
--name=client \
--rm --interactive --tty \
--net=host \
ghcr.io/dazwilkin/webthing-vaccine-client:${TAG} \
  --host=${HOST} \
  --port=${PORT}
```

> **NOTE** The Rust server currently only serves on `:8888` so the client's port is effectively constant

Client repeatedly iterates over API calls (`/`, `/x`, `/x/properites`, `/x/properties/longitude`) then sleeps

```console
[api:NewClient] Creating client: localhost:8888
longitude[api:GetThings] Entered
longitude{Context:https://iot.mozilla.org/schemas AtType:[] Actions:{} Base:http://localhost:8888/0 Description:IIoT Vaccine Refrigerator Events:{} HREF:/0 ID:refrigerator Links:[{HREF:/0/properties Rel:properties} {HREF:/0/actions Rel:actions} {HREF:/0/events Rel:events} {HREF:ws://localhost:8888/0 Rel:alternate}] Properties:map[temperature:{AtType:TempProperty Description:Temperature of the Refrigerator Links:[{HREF:/0/properties/temperature Rel:property}] Maximum:0 Minimum:-273 Title:Temperature TType:number}] Title:Vaccine Refrigerator}
longitude{Context:https://iot.mozilla.org/schemas AtType:[] Actions:{} Base:http://localhost:8888/1 Description:Truck Events:{} HREF:/1 ID:truck Links:[{HREF:/1/properties Rel:properties} {HREF:/1/actions Rel:actions} {HREF:/1/events Rel:events} {HREF:ws://localhost:8888/1 Rel:alternate}] Properties:map[latitude:{AtType:LatitudeProperty Description:Latitude of the Truck Links:[{HREF:/1/properties/latitude Rel:property}] Maximum:90 Minimum:-90 Title:Latitude TType:number} longitude:{AtType:LongitudeProperty Description:Longitude of the Truck Links:[{HREF:/1/properties/longitude Rel:property}] Maximum:180 Minimum:-180 Title:Longitude TType:number}] Title:Truck}
longitude[api:GetThing:0] Entered
longitude{https://iot.mozilla.org/schemas [] {} http://localhost:8888/0 IIoT Vaccine Refrigerator {}  refrigerator [{/0/properties properties} {/0/actions actions} {/0/events events} {ws://localhost:8888/0 alternate}] map[temperature:{TempProperty Temperature of the Refrigerator [{/0/properties/temperature property}] 0 -273 Temperature number}] Vaccine Refrigerator}
longitude[api:GetProperties:1] Entered
longitudelatitude: 47.51549530029297
longitudelongitude: -121.44660949707031
longitude[api:GetProperty:1] Entered: longitude
longitudemap[longitude:-121.44660949707031]
longitude[api:GetThings] Entered
longitude{Context:https://iot.mozilla.org/schemas AtType:[] Actions:{} Base:http://localhost:8888/0 Description:IIoT Vaccine Refrigerator Events:{} HREF:/0 ID:refrigerator Links:[{HREF:/0/properties Rel:properties} {HREF:/0/actions Rel:actions} {HREF:/0/events Rel:events} {HREF:ws://localhost:8888/0 Rel:alternate}] Properties:map[temperature:{AtType:TempProperty Description:Temperature of the Refrigerator Links:[{HREF:/0/properties/temperature Rel:property}] Maximum:0 Minimum:-273 Title:Temperature TType:number}] Title:Vaccine Refrigerator}
longitude{Context:https://iot.mozilla.org/schemas AtType:[] Actions:{} Base:http://localhost:8888/1 Description:Truck Events:{} HREF:/1 ID:truck Links:[{HREF:/1/properties Rel:properties} {HREF:/1/actions Rel:actions} {HREF:/1/events Rel:events} {HREF:ws://localhost:8888/1 Rel:alternate}] Properties:map[latitude:{AtType:LatitudeProperty Description:Latitude of the Truck Links:[{HREF:/1/properties/latitude Rel:property}] Maximum:90 Minimum:-90 Title:Latitude TType:number} longitude:{AtType:LongitudeProperty Description:Longitude of the Truck Links:[{HREF:/1/properties/longitude Rel:property}] Maximum:180 Minimum:-180 Title:Longitude TType:number}] Title:Truck}
longitude[api:GetThing:0] Entered
longitude{https://iot.mozilla.org/schemas [] {} http://localhost:8888/0 IIoT Vaccine Refrigerator {}  refrigerator [{/0/properties properties} {/0/actions actions} {/0/events events} {ws://localhost:8888/0 alternate}] map[temperature:{TempProperty Temperature of the Refrigerator [{/0/properties/temperature property}] 0 -273 Temperature number}] Vaccine Refrigerator}
longitude[api:GetProperties:1] Entered
longitudelatitude: 46.927162170410156
longitudelongitude: -122.0382080078125
longitude[api:GetProperty:1] Entered: longitude
longitudemap[longitude:-122.0382080078125]
```

## Akri

```console
[zeroconf:main] Entered
[zeroconf:new] Entered
[zeroconf:main] Service: kind: _webthing._tcp
name: Vaccine Things
host: akri.local
addr: 10.138.0.2
port: 8888
TXTs:
  AKRI_ZEROCONF_DEVICE_PATH: /

[zeroconf:main:loop] Sleep
[zeroconf:main:loop] check_device(Service { kind: "_webthing._tcp", name: "Vaccine Things", host: "akri.local", addr: "10.138.0.2", port: 8888, txts: Some({"AKRI_ZEROCONF_DEVICE_PATH": "/"}) })
[zeroconf:read_device] Entered: Service { kind: "_webthing._tcp", name: "Vaccine Things", host: "akri.local", addr: "10.138.0.2", port: 8888, txts: Some({"AKRI_ZEROCONF_DEVICE_PATH": "/"}) }
[zeroconf:main:loop] Sleep
[zeroconf:main:loop] check_device(Service { kind: "_webthing._tcp", name: "Vaccine Things", host: "akri.local", addr: "10.138.0.2", port: 8888, txts: Some({"AKRI_ZEROCONF_DEVICE_PATH": "/"}) })
[zeroconf:read_device] Entered: Service { kind: "_webthing._tcp", name: "Vaccine Things", host: "akri.local", addr: "10.138.0.2", port: 8888, txts: Some({"AKRI_ZEROCONF_DEVICE_PATH": "/"}) }
[zeroconf:main:loop] Sleep
```

And:

```bash
curl --silent akri.local:8888/0/properties/temperature | jq .
{
  "temperature": -80.00003051757812
}

curl --silent akri.local:8888/1/properties | jq .
{
  "latitude": 48.516845703125,
  "longitude": -122.60163116455078
}
```
