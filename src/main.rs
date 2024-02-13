use chrono::{DateTime, Utc};
use influxdb::{Client, InfluxDbWriteable, ReadQuery, Timestamp};

#[derive(InfluxDbWriteable)]
struct WeatherReading {
    time: DateTime<Utc>,
    humidity: i32,
    #[influxdb(tag)]
    wind_direction: String,

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://8.140.193.107:8086","testdb").with_token("ZYqKVMRr6_cXbsuWggoVdG4MKEX2mg5GIM43WRPYUIEx9LvVoQ0b6UODrj50JfNSdIeazG4mrQQcZk1jeiwxnQ==");
    let write_query = vec![
        WeatherReading{
            time: Utc::now(),
            humidity: 30,
            wind_direction: "north".into(),
        }.into_query("weather"),
        // WeatherReading {
        //     time: Utc::now(),
        //     humidity: 40,
        //     wind_direction: String::from("west"),
        // }.into_query("weather"),
    ];
    let mut  write_query = Vec::new();
    for i in 1..1000 {
        write_query.push(WeatherReading{
            time: Utc::now(),
            humidity:i,
            wind_direction: i.to_string(),
        }.into_query("weather"))
    }
    let write_result = client
        .query(write_query)
        .await;
    println!("{:?}",write_result);
    let read_query = ReadQuery::new("SELECT * FROM weather");
    let read_result = client.query(read_query).await?;
    println!("{}", read_result);
    Ok(())
}
