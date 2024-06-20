use std::error::Error;
use serde_json::Value;

use crate::models::*;

#[derive(Debug)]
pub struct OpenMeteo {
    api_url: String,
    set_coordinates: bool,
    set_time_zone: bool,
    set_start_date: bool,
    set_end_date: bool,
}

impl OpenMeteo {

    // create new instance of open-meteo

    pub fn new() -> Self {
        Self {
            api_url: "https://api.open-meteo.com/v1/forecast?".to_owned(),
            set_coordinates: false,
            set_time_zone: false,
            set_start_date: false,
            set_end_date: false
        }
    }

    // set coordinates 

    pub fn coordinates(mut self, lat: f32, lon: f32) -> Result<OpenMeteo, Box<dyn Error>> {
        if self.set_coordinates {
            return Err("Location is already set".into());
        }

        let url_part = format!("latitude={}&longitude={}", lat, lon);
        self.api_url.push_str(&url_part);
        self.set_coordinates = true;

        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // get coords using place name

    pub async fn location(self, place_name: &str) -> Result<OpenMeteo, Box<dyn Error>> {
        let url = format!("https://geocode.maps.co/search?q={}", place_name);

        let mut response = reqwest::get(url).await?;

        if response.status() != reqwest::StatusCode::OK {
            return Err("Error getting city coordinates from geolocation".into());
        } else {
            response = response.text().await?
        }

        let json: Value = serde_json::from_str(&response).expect("Couldn't parse coordinates using geocode, try using .coordinates() instead".into());

        let mut vec_len: usize = 0;

        match json{
            Value::Array(ref val) => vec_len = val.len(),
            _ => {}
        }
        if vec_len < 1 {
           return Err("Error getting city coordinates. Geolocation did not return any coordinates".into());
        }

        let (lat, lon) = 
            (json[0]["lat"].as_str().unwrap()
                .parse::<f32>().unwrap(), 
             json[0]["lon"].as_str().unwrap()
                .parse::<f32>().unwrap(),);

        Ok(self.coordinates(lat, lon)?)
    }

    // check if location is not set

    fn _check_location(&self) -> Result<(), Box<dyn Error>> {
        if !self.set_coordinates {
            return Err("Location is not set. Please set your location using .location() or .coordinates() method first.".into());
        }
        Ok(())
    }

    // set start date YYYY-MM-DD

    pub fn start_date(mut self, start_date: &str) -> Result<OpenMeteo, Box<dyn Error>> {
        if self.set_start_date {
            return Err("Start date is already set".into());
        }
        self._check_location()?;

        let url_part = format!("&start_date={}", start_date);
        self.api_url.push_str(&url_part);
        self.set_start_date = true;

        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // set end date YYYY-MM-DD

    pub fn end_date(mut self, end_date: &str) -> Result<OpenMeteo, Box<dyn Error>> {
        if self.set_end_date {
            return Err("End date is already set".into());
        }
        self._check_location()?;

        let url_part = format!("&end_date={}", end_date);
        self.api_url.push_str(&url_part);

        self.set_end_date = true;

        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // add current weather to request

    pub fn current_weather(mut self) -> Result<OpenMeteo, Box<dyn Error>> {
        self._check_location()?;
        self.api_url.push_str("&current_weather=true");

        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // add past days weather to request

    pub fn past_days(mut self, past_days: u64) -> Result<OpenMeteo, Box<dyn Error>> {
        self._check_location()?;

        let url_part = format!("&past_days={}", past_days);
        self.api_url.push_str(&url_part);

        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // add forecast days weather to request

    pub fn forecast_days(mut self, forecast_days: u64) -> Result<OpenMeteo, Box<dyn Error>> {
        self._check_location()?;

        let url_part = format!("&forecast_days={}", forecast_days);
        self.api_url.push_str(&url_part);
        
        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // add all hourly variables to request

    pub fn hourly(mut self) -> Result<OpenMeteo, Box<dyn Error>> {
        self.api_url.push_str("&hourly=temperature_2m,relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,precipitation,rain,showers,snowfall,snow_depth,weathercode,pressure_msl,surface_pressure,cloudcover,cloudcover_low,cloudcover_mid,cloudcover_high,visibility,evapotranspiration,et0_fao_evapotranspiration,vapor_pressure_deficit,windspeed_10m,windspeed_80m,windspeed_120m,windspeed_180m,winddirection_10m,winddirection_80m,winddirection_120m,winddirection_180m,windgusts_10m,temperature_80m,temperature_120m,temperature_180m,soil_temperature_0cm,soil_temperature_6cm,soil_temperature_18cm,soil_temperature_54cm,soil_moisture_0_1cm,soil_moisture_1_3cm,soil_moisture_3_9cm,soil_moisture_9_27cm,soil_moisture_27_81cm");
        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // add all daily variables to request

    pub fn daily(mut self) -> Result<OpenMeteo, Box<dyn Error>> {
        if !self.set_time_zone {
            return Err("Specify .timezone() before .daily() method using TimeZone enum".into());
        }
        self.api_url.push_str("&daily=weathercode,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,uv_index_max,uv_index_clear_sky_max,precipitation_sum,rain_sum,showers_sum,snowfall_sum,precipitation_hours,precipitation_probability_max,windspeed_10m_max,windgusts_10m_max,winddirection_10m_dominant,shortwave_radiation_sum,et0_fao_evapotranspiration");
        
        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // set time zone for daily variables

    pub fn time_zone(mut self, time_zone: TimeZone) -> Result<OpenMeteo, Box<dyn Error>> {
        if self.set_time_zone {
            return Err("Time zone is already set".into());
        }
        self.api_url.push_str("&timezone=");
        
        match time_zone {
            TimeZone::AmericaAnchorage => self.api_url.push_str("America%2FAnchorage"),
            TimeZone::AmericaLosAngeles => self.api_url.push_str("America%2FLos_Angeles"),
            TimeZone::AmericaDenver => self.api_url.push_str("America%2FDenver"),
            TimeZone::AmericaChicago => self.api_url.push_str("America%2FChicago"),
            TimeZone::AmericaNewYork => self.api_url.push_str("America%2FNew_York"),
            TimeZone::AmericaSaoPaulo => self.api_url.push_str("America%2FASao_Paulo"),
            TimeZone::GMT0 => self.api_url.push_str("GMT"),
            TimeZone::Auto => self.api_url.push_str("auto"),
            TimeZone::EuropeLondon => self.api_url.push_str("Europe%2FLondon"),
            TimeZone::EuropeBerlin => self.api_url.push_str("Europe%2FBerlin"),
            TimeZone::EuropeMoscow => self.api_url.push_str("Europe%2FMoscow"),
            TimeZone::AfricaCairo => self.api_url.push_str("Africa%2FCairo"),
            TimeZone::AsiaBangkok => self.api_url.push_str("Asia%2FBangkok"),
            TimeZone::AsiaSingapore => self.api_url.push_str("Asia%2FSingapore"),
            TimeZone::AsiaTokyo => self.api_url.push_str("Asia%2FTokio"),
            TimeZone::AustraliaSydney => self.api_url.push_str("Australia%2FSydney"),
            TimeZone::PacificAuckland => self.api_url.push_str("Pacific%2FAuckland")
        }

        self.set_time_zone = true;

        Ok(Self {
            api_url: self.api_url,
            set_coordinates: self.set_coordinates,
            set_time_zone: self.set_time_zone,
            set_start_date: self.set_start_date,
            set_end_date: self.set_end_date,
        })
    }

    // send a request

    pub async fn query(&self) -> Result<OpenMeteoData, Box<dyn Error>> {
        let url = &self.api_url;
        let response = reqwest::get(url).await?.text().await?;
        let data = 
            serde_json::from_str::<OpenMeteoData>(&response);
        if data.is_err() { 
            let err = 
                serde_json::from_str::<OpenMeteoError>(&response)?;
            return Err(err.reason.into());
        }
        Ok(data?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test] 
    async fn test_coordinates() {
        let test = OpenMeteo::new()
            .coordinates(51.0, 0.0).unwrap()
            .query().await;

        assert!(test.is_ok());
    } 

    #[tokio::test] 
    async fn test_location() {
        let data = OpenMeteo::new()
            .location("Moscow").await.unwrap()
            .query().await;

        assert!(data.is_ok());
    } 

    #[tokio::test] 
    async fn test_current_weather() {
        let data = OpenMeteo::new()
            .coordinates(55.0, 37.0).unwrap()
            .current_weather().unwrap()
            .hourly().unwrap()
            .query().await;

        assert!(data.is_ok());
    } 

    #[tokio::test] 
    async fn test_past_days() {
        let data = OpenMeteo::new()
            .coordinates(55.0, 13.0).unwrap()
            .past_days(10).unwrap()
            .hourly().unwrap()
            .query().await;

        assert!(data.is_ok());
    } 

    #[tokio::test] 
    async fn test_forecast_days() {
        let data = OpenMeteo::new()
            .coordinates(55.0, 13.0).unwrap()
            .forecast_days(10).unwrap()
            .hourly().unwrap()
            .query().await;

        assert!(data.is_ok());
    } 

    #[tokio::test] 
    async fn location_not_set_error() {
        let test1 = OpenMeteo::new()
            .current_weather();
        let test2 = OpenMeteo::new()
            .past_days(10);
        let test3 = OpenMeteo::new()
            .forecast_days(10);
        
        eprintln!("{:?} .current_weather()", test1);  
        eprintln!("{:?} .past_days()", test2); 
        eprintln!("{:?} .forecast_days()", test3); 

        assert!(test1.is_err());
        assert!(test2.is_err());
        assert!(test3.is_err());
    } 

    #[tokio::test] 
    async fn location_is_already_set_error() {
        let test1 = OpenMeteo::new()
            .location("Berlin").await.unwrap()
            .forecast_days(10).unwrap()
            .location("Copenhagen").await;
        let test2 = OpenMeteo::new()
            .coordinates(55.0, 37.0).unwrap()
            .forecast_days(10).unwrap()
            .coordinates(55.0, 12.0);
        let test3 = OpenMeteo::new()
            .coordinates(55.0, 37.0).unwrap()
            .forecast_days(10).unwrap()
            .location("London").await;
        eprintln!("{:?} double .location()", test1);
        eprintln!("{:?} double .coordinates()", test2);
        eprintln!("{:?} mixed", test3);   

        assert!(test1.is_err());
        assert!(test2.is_err());
        assert!(test3.is_err());
    } 

    #[tokio::test] 
    async fn test_daily() {
        let test = OpenMeteo::new()
            .location("London").await.unwrap()
            .forecast_days(10).unwrap()
            .time_zone(TimeZone::EuropeLondon).unwrap()
            .daily();
        
        assert!(test.is_ok());
    } 

    #[tokio::test] 
    async fn daily_without_timezone_error() {
        let test = OpenMeteo::new()
            .location("London").await.unwrap()
            .forecast_days(10).unwrap()
            .daily();
        
        eprintln!("{:?}", test);
        assert!(test.is_err());
    } 

    #[tokio::test] 
    async fn timezone_already_set_error() {
        let test = OpenMeteo::new()
            .location("London").await.unwrap()
            .time_zone(TimeZone::EuropeLondon).unwrap()
            .forecast_days(10).unwrap()
            .daily().unwrap()
            .time_zone(TimeZone::EuropeBerlin);

        eprintln!("{:?}", test);    
        assert!(test.is_err());
    } 

    #[tokio::test] 
    async fn forecast_more_than_16_days_error()  {
        let test = OpenMeteo::new()
            .location("London").await.unwrap()
            .forecast_days(17).unwrap()
            .query().await;

        eprintln!("{:?}", test);  
        assert!(test.is_err());
    }

    #[tokio::test] 
    async fn end_date_without_start_date_error()  {
        let test = OpenMeteo::new()
            .location("London").await.unwrap()
            .current_weather().unwrap()
            .end_date("2023-12-12").unwrap()
            .query().await;

        eprintln!("{:?}", test);  
        assert!(test.is_err());
    }
}
