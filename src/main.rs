use actix_rt;
use env_logger;
use log::trace;
use rand::Rng;
use serde_json::{json, value};
use std::sync::{Arc, RwLock, Weak};
use std::{thread, time};
use uuid::Uuid;
use webthing::{
    property::ValueForwarder, server::ActionGenerator, Action, BaseProperty, BaseThing, Thing,
    ThingsType, WebThingServer,
};

// Duvall WA
const LATITUDE: f32 = 47.7423;
const LONGITUDE: f32 = -121.9857;

// Storage Temperature of Vaccine
const TEMPERATURE: f32 = -80.0;

struct Generator;
impl ActionGenerator for Generator {
    fn generate(
        &self,
        thing: Weak<RwLock<Box<dyn Thing>>>,
        name: String,
        input: Option<&serde_json::Value>,
    ) -> Option<Box<dyn Action>> {
        // TODO(dazwilkin) does nothing
        None
    }
}

struct TempValueForwarder;
impl ValueForwarder for TempValueForwarder {
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
        println!("Temp: {}", value);
        Ok(value)
    }
}
struct Refrigerator;
impl Refrigerator {
    fn new() -> Refrigerator {
        Refrigerator {}
    }
    fn to_thing(&self) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
        let mut thing = BaseThing::new(
            "refrigerator".to_owned(),
            "Vaccine Refrigerator".to_owned(),
            // TODO(dazwilkin) What goes here?
            Some(vec![]),
            Some("IIoT Vaccine Refrigerator".to_owned()),
        );

        // Properties
        let temp_description = json!({
            "@type":"TempProperty",
            "title":"Temperature",
            "type":"number",
            "description":"Temperature of the Refrigerator",
            "minimum": -273.0,
            "unit":"°C",

        });
        let temp_description = temp_description.as_object().unwrap().clone();
        thing.add_property(Box::new(BaseProperty::new(
            "temperature".to_owned(),
            json!(0),
            Some(Box::new(TempValueForwarder)),
            Some(temp_description),
        )));

        // Return
        Arc::new(RwLock::new(Box::new(thing)))
    }
}
struct LatitudeValueForwarder;
impl ValueForwarder for LatitudeValueForwarder {
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
        println!("Latitude: {}", value);
        Ok(value)
    }
}

struct LongitudeValueForwarder;
impl ValueForwarder for LongitudeValueForwarder {
    fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
        println!("Longitude: {}", value);
        Ok(value)
    }
}
struct Truck;
impl Truck {
    fn new() -> Truck {
        Truck {}
    }
    fn to_thing(&self) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
        let mut thing = BaseThing::new(
            "truck".to_owned(),
            "Truck".to_owned(),
            // TODO(dazwilkin) What goes here?
            Some(vec![]),
            Some("Truck".to_owned()),
        );

        // Properties
        // TODO(dazwilkin) Should (lat,lng) be a single property?
        let latitude_description = json!({
            "@type":"LatitudeProperty",
            "title":"Latitude",
            "type":"number",
            "description":"Latitude of the Truck",
            "minimum": -90.0,
            "maximum": 90.0,
        });
        let latitude_description = latitude_description.as_object().unwrap().clone();
        thing.add_property(Box::new(BaseProperty::new(
            "latitude".to_owned(),
            json!(0),
            Some(Box::new(LatitudeValueForwarder)),
            Some(latitude_description),
        )));

        let longitude_description = json!({
            "@type":"LongitudeProperty",
            "title":"Longitude",
            "type":"number",
            "description":"Longitude of the Truck",
            "minimum": -180.0,
            "maximum": 180.0,
        });
        let longitude_description = longitude_description.as_object().unwrap().clone();
        thing.add_property(Box::new(BaseProperty::new(
            "longitude".to_owned(),
            json!(0),
            Some(Box::new(LongitudeValueForwarder)),
            Some(longitude_description),
        )));

        // Return
        Arc::new(RwLock::new(Box::new(thing)))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let refrigerator = Refrigerator::new().to_thing();
    let truck = Truck::new().to_thing();

    let mut things = Vec::new();
    things.push(refrigerator.clone());
    things.push(truck.clone());

    // Refrigerator
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        // Do update stuff
        trace!("[main::thread::Refrigerator] Entering");
        loop {
            thread::sleep(time::Duration::from_millis(1000));

            let refrigerator = refrigerator.clone();

            let new_temp = TEMPERATURE + rng.gen_range(-0.0001..0.0001);
            let new_temp = json!(new_temp);

            trace!(
                "[main::thread::Refrigerator] Refrigerator Temp(c): {}",
                new_temp
            );
            {
                let mut refrigerator = refrigerator.write().unwrap();
                let prop = refrigerator
                    .find_property(&"temperature".to_owned())
                    .unwrap();
                let _ = prop.set_cached_value(new_temp.clone());
            }

            let mut refrigerator = refrigerator.write().unwrap();
            refrigerator.property_notify("temperature".to_owned(), new_temp);
        }
    });

    // Truck
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        // Do update stuff
        trace!("[main::thread::Truck] Entering");
        loop {
            thread::sleep(time::Duration::from_millis(2500));

            let truck = truck.clone();
            // Simulate truck moving
            let new_lat = LATITUDE + rng.gen_range(-1.0..1.0);
            let new_lat = json!(new_lat);

            let new_lng = LONGITUDE + rng.gen_range(-1.0..1.0);
            let new_lng = json!(new_lng);

            trace!(
                "[main::thread::Truck] Truck's location: ({},{})",
                new_lat,
                new_lng
            );
            {
                let mut truck = truck.write().unwrap();
                let prop = truck.find_property(&"latitude".to_owned()).unwrap();
                let _ = prop.set_cached_value(new_lat.clone());
                let prop = truck.find_property(&"longitude".to_owned()).unwrap();
                let _ = prop.set_cached_value(new_lng.clone());
            }

            let mut truck = truck.write().unwrap();
            truck.property_notify("latitude".to_owned(), new_lat);
            truck.property_notify("longitude".to_owned(), new_lng);
        }
    });

    trace!("[main] Creating WebThingServer");
    let mut server = WebThingServer::new(
        ThingsType::Multiple(things, "Vaccine Things".to_owned()),
        Some(8888),
        None,
        None,
        Box::new(Generator),
        None,
        None,
    );

    trace!("[main] Starting WebThingServer");
    server.start(None).await
}
