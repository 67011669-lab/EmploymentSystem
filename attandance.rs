use egui::Ui;
use reqwest::Error;
use serde::Deserialize;
use chrono::{Local, Timelike};
use std::thread;
use std::time::Duration;
use eframe::egui;

use rusqlite::{params,Connection, Result,Row};
use crate::Someapp;


#[derive(Deserialize)]
struct LocationResponse {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    loc: String, 
}

#[derive(Default,Debug,Clone)]
pub struct Point {
    lat: f64,
    lng: f64,
}

pub struct Square {
    top_left: Point,
    bottom_right: Point,
}

impl Square {
    fn contains(&self, point: &Point) -> bool {
        point.lat >= self.top_left.lat &&
        point.lat <= self.bottom_right.lat &&
        point.lng >= self.top_left.lng &&
        point.lng <= self.bottom_right.lng
    }
}

// //shift<time> variable is use to start when the work shift start
// //shiftend<time is when the time is reset>
// //requirement is requirement time ** recomend it to be hours*3600+minute*60 since I record work time in seconds
// impl Someapp{
// }


//part of combo box

//I'm pretty sure combo box already work
impl Someapp {

    pub fn new_mem_location(&self) -> Connection{
        let connloc:Connection = Connection::open("location.db").expect("not found");
        connloc.execute("CREATE TABLE IF NOT EXISTS location(
        department TEXT NOT NULL,
        toplat FLOAT NOT NULL,
        toplng FLOAT NOT NULL,
        lowlat FLOAT NOT NULL,
        lowlng FLOAT NOT NULL

        )", []).expect("no db found");
        connloc

    }
    pub fn insert_new_mem_location(connloc:&Connection,department:&String,toplat:&f64,toplng:&f64,lowlat:&f64,lowlng:&f64) -> Result<()>{
        connloc.execute(
            "INSERT INTO location (department,toplat,toplng,lowlat,lowlng) 
            VALUES (?1, ?2,?3,?4,?5)",
            params![department,toplat,toplng,lowlat,lowlng],
        )?;
        
        Ok(())
    }
    pub fn get_location(connloc:&Connection,department:&String) -> Result<Option<Self>>{
        let mut stmt = connloc.prepare("SELECT toplat,toplng,lowlat,lowlng FROM location WHERE department = ?1")?;
        let some = stmt.query_row(params![department], |row| {
            let toplat:f64 = row.get(0)?;
            let toplng:f64 = row.get(1)?;
            let lowlat:f64 = row.get(2)?;
            let lowlng:f64 = row.get(3)?;
            Ok(Self{
             top:(toplat,toplng,String::new(),String::new()),
             low:(lowlat,lowlng,String::new(),String::new()),
             ..Default::default()
            })
            });
            match some {
                Ok(someapp) => Ok(Some(someapp)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e), // Propagate any other error
            }
             
    }
    
    pub fn show(&mut self, ctx: &egui::Context,ui:&mut Ui) {
            
            
            self.new_mem_location();
            // -------------------------under is frontend
            if self.ui_page_attan == 1{
              if self.login_string == "Employer"{
                ui.label(format!("Your Department now: {} ",self.department));
                ui.label("");
                ui.label("Use Google Map to locate your department by use");
                ui.label("Latitude and Longtitube for topleft to bottomright to form a square of the area");
                ui.label("You Must Insert Decimal 3 Positions");

                ui.label("Topleft Longtitude");
                let mut toplat = ui.add(egui::TextEdit::singleline(&mut self.top.2).hint_text("Enter Topleft Longtitude"));
                ui.label("Topleft Latitude");
                let mut toplng = ui.add(egui::TextEdit::singleline(&mut self.top.3).hint_text("Enter Topleft Latitude"));
                ui.label("Topleft Bottomright Longtitude");
                let mut lowlat = ui.add(egui::TextEdit::singleline(&mut self.low.2).hint_text("Enter Bottomright Longtitude"));
                ui.label("Topleft Bottomright Latitude");
                let mut lowlng = ui.add(egui::TextEdit::singleline(&mut self.low.3).hint_text("Enter Bottomright Latitude"));
                if ui.add_sized([450.0,25.0], egui::Button::new(egui::RichText::from("SET LOCATION").size(17.5))).clicked(){
                    
                    match self.top.2.parse::<String>(){
                       Ok(num) => self.top.0 = self.top.2.parse().unwrap(),
                       Err(e) => println!("Error: {} ",e)
                    }
                    match self.top.3.parse::<String>(){
                        Ok(num) => self.top.1 = self.top.3.parse().unwrap(),
                        Err(e) => println!("Error: {} ",e)
                    }
                    match self.low.2.parse::<String>(){
                        Ok(num) => self.low.0 = self.low.2.parse().unwrap(),
                        Err(e) => println!("Error: {} ",e)
                    }
                    match self.low.3.parse::<String>(){
                        Ok(num) => self.low.1 = self.low.3.parse().unwrap(),
                        Err(e) => println!("Error: {} ",e)
                    }
                    
                    Self::insert_new_mem_location(&self.clone().new_mem_location(), &self.department, &self.top.0, &self.top.1, &self.low.0, &self.low.1);
                    self.ui_page_attan = 2 ;
                
               }
               if self.login_string == "Employee"{
                let login_text = egui::RichText::new("You don't have any department, Wait for to be employ").size(25.0).strong().underline();
                ui.put(egui::Rect::from_min_size(egui::Pos2::new(650.0, 350.0),egui::Vec2::new(200.0, 300.0)),egui::Label::new(login_text),);
               }
              }
            }//page1

            let sometr:Connection = self.new_mem();
            let employees_vec:Vec<Someapp> = Someapp::get_all_result(&sometr,&String::from("Employee"),&self.department).expect("");
               
            
            if self.ui_page_attan == 2 {
                
                ui.label("Employees Attandances list");
                
                for person in employees_vec{
                    ui.horizontal(|ui|{
                 if ui.add_sized([700.0, 30.0],egui::Button::new(egui::RichText::new(format!("{} {}",person.name,person.last_name)).size(15.0))).clicked() {
                    self.ui_page_attan =3;
                    self.stat_keep1.push(person);

                        }
                    ui.label("                                          ");
                    if self.contained == true{
                        ui.label(egui::RichText::new("Counting Worktime").color(egui::Color32::from_rgba_premultiplied(71,250,84,1)));
                    }
                    else if self.contained == false && self.shiftstart.0 != 0 && self.shiftend.0 != 0{
                        ui.label(egui::RichText::new("Not in the Area").color(egui::Color32::RED));

                    }
                    else {
                        ui.label("Not Counting Worktime ");
                    }
                });
                }
                  
                
             
            
            }
            
           
            if self.ui_page_attan == 3 {
            // ComboBox shift start
            
             println!("{:?}",self.stat_keep1);
              if self.login_string == "Employer"{
               for mut person in self.stat_keep1.clone(){
                
                       
                       ui.label("SET TIME FOR EMPLOYEES");
                       ui.label("");
                       ui.label(format!("Start time: {}:{}:00",self.shiftstart.0,self.shiftstart.1));
                       ui.horizontal(|ui|{
                        egui::ComboBox::from_label("start hour")
                            .selected_text(&self.shiftstart.0.to_string()) // Show the current selected role
                            .show_ui(ui, |ui|{
                            for i in 1..=24{
                                ui.selectable_value(&mut self.shiftstart.0, i, i.to_string());
                            }});
                        egui::ComboBox::from_label("start minute")
                            .selected_text(&self.shiftstart.1.to_string()) // Show the current selected role
                            .show_ui(ui, |ui|{
                                for i in 1..=59{
                                ui.selectable_value(&mut self.shiftstart.1, i, i.to_string());
                                }});
                            });
                    // ComboBox shift end
                    ui.label(format!("End time: {}:{}:00",self.shiftstart.0,self.shiftstart.1));
                    ui.horizontal(|ui|{
                        egui::ComboBox::from_label("end hour")
                                .selected_text(&self.shiftend.0.to_string()) // Show the current selected role
                                .show_ui(ui, |ui|{
                                for i in 0..=24{
                                    ui.selectable_value(&mut self.shiftend.0, i, i.to_string());
                                }});
                        egui::ComboBox::from_label("end minute")
                                .selected_text(&self.shiftend.1.to_string()) // Show the current selected role
                                .show_ui(ui, |ui|{
                                for i in 0..=59{
                                    ui.selectable_value(&mut self.shiftend.1, i, i.to_string());
                                }});
                            });
                        ui.label(format!("Req: {}:00:00",self.requiretime));
                        egui::ComboBox::from_label("requirement")
                            .selected_text(&self.requiretime.to_string()) // Show the current selected role
                            .show_ui(ui, |ui|{
                                for i in 1..=24{
                                ui.selectable_value(&mut self.requiretime, i, i.to_string());
                                }});


                    // Display selected values
                        if ui.button("Set time").clicked() {
                            let some_req:u32 = self.requiretime * 3600;
                            Self::insert_time(&self.new_mem(),&person.username,&self.shiftstart.0,&self.shiftstart.1,&self.shiftend.0,&self.shiftend.1,&some_req).expect("");
                        }
                        ui.label("");
                        if ui.add_sized([300.0,22.5],egui::Button::new("Back")).clicked() {
                            self.ui_page_attan =2;
                            self.stat_keep1.clear();
                }

              
            
            }
        }//Employer

             if self.login_string == "Employee"{
                for mut person in self.stat_keep1.clone(){
                    
                        ui.label("TIMESHIFT FOR EMPLOYEES");
                        ui.label("");
                        ui.label(format!("Start time: {}:{}:00",person.shiftstart.0,person.shiftstart.1));
                        ui.label(format!("End time: {}:{}:00",person.shiftstart.0,person.shiftstart.1));
                        ui.label(format!("Req: {}:00:00",person.requiretime));
                        if ui.add_sized([300.0,22.5],egui::Button::new("Back")).clicked() {
                            self.ui_page_attan =2;
                            self.stat_keep1.clear();
                        }


                    
                }
             }
              
         }
           
         } 
          
        pub async fn timelocate(&mut self,toplat:f64,toplng:f64,lowlat:f64,lowlng:f64) -> Result<(), Error> {
            
            let now = Local::now();
            if (now.hour() == self.shiftstart.0 && now.minute()>= self.shiftstart.1) || now.hour() > self.shiftstart.0 {
            let url = "https://ipinfo.io/json";// wifi ip information f
            let response = reqwest::get(url).await?;// location form your ip based from transmitter
            let location: LocationResponse = response.json().await?;
            let coords: Vec<&str> = location.loc.split(',').collect();
            let square = Square {
                top_left: Point { lat: toplat, lng:toplng },
                bottom_right: Point { lat: lowlat, lng: lowlng},
            };

            if coords.len() == 2 {
            let cl1 = coords[0];
            let cl2 = coords[1];
            println!("{}/{}",cl1,cl2);
            let latitude = cl1.parse::<f64>();
            let longitude = cl2.parse::<f64>();
            let ypoint = Point{
                lat: latitude.expect("REASON"),
                lng: longitude.expect("REASON"),
                };
            self.ypoint = ypoint.clone();
            self.contained = square.contains(&ypoint);
            
            
            
                
            if self.contained // this itself is already a boolean square.contains(&ypoint) will return true if point is in square square in line 54 and point y 
                {
                    self.worktime += 1;
                }
            
                if self.worktime == self.requiretime {
                    
                    self.attan_stat.0 += 1;
                    self.worktime = 0;
                }
            if now.hour() == self.shiftend.0{
                if now.minute()== self.shiftend.1{
                    if now.second()==0{
                        if self.worktime == 0
                        {
                            self.attan_stat.2 += 1 // not come 2day
                        }else{
                            self.attan_stat.1 += 1;
                            
                        }
                      
                        self.worktime = 0;
                    }
                }
            }
            Self::insert_attan(&self.clone().new_mem(), &self.username, &self.attan_stat.0,&self.attan_stat.1, &self.attan_stat.2);
        }}
        thread::sleep(Duration::from_millis(950));//sleep this thread for 1 sec
        return Ok(());
        }
   
  
}