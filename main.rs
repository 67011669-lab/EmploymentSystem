use attandance::Point;
use eframe::{egui::{self, CentralPanel,widgets::{Image,Label},Layout,Vec2,Align,Pos2,Color32}, run_native, App, NativeOptions};
use egui::{ Button, RichText, Widget};
use rusqlite::{params,Connection, Result,Row};
use rusqlite::OptionalExtension;
use chrono::{Datelike, Local};
use std::env;


mod attandance;


#[tokio::main]
async fn main(){
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
   run_native("What", options, Box::new(|cc|{
    egui_extras::install_image_loaders(&cc.egui_ctx);
    Ok(Box::new(Someapp::new(cc)))} )).expect("REASON");

    
}




// APP 
#[derive(Default,Debug,Clone)]
pub struct Someapp{
  
    username:String,
    password:String,
    name: String,
    last_name:String,
    age: u32,
    birth_data:(u8,u8,u32),
    department:String,
    login_string:String,
    
    login_true:bool,
    register_true:bool,
    login_c:bool,// check if you login or not 

    
    stat_string:String,// check err stat
    stat_keep:Vec<Self>,
    stat_keep1:Vec<Self>,
    shiftstart: (u32,u32),
    shiftend: (u32,u32),
    requiretime: u32,
    worktime:u32,

    top:(f64,f64,String,String),//two string for input 
    low:(f64,f64,String,String),
    
    attan_stat:(u32,u32,u32),
    
    ui_page:u8,
    ui_page_attan:u8,

    ypoint:Point,
    contained:bool
  
}

impl Someapp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        Self::default()
    }

    fn reset(&mut self){
        
        self.username = String::new();
        self.password = String::new();
        self.name = String::new();
        self.last_name = String::new();
        self.age = 0;
        self.birth_data = (0,0,0);
        self.department = String::new();
        self.login_string = String::new();
        self.top.0 = 0.0;
        self.top.1 = 0.0;
        self.top.2 = String::new();
        self.top.3 = String::new();
        self.low.0 = 0.0;
        self.low.1 = 0.0;
        self.low.2 = String::new();
        self.low.3 = String::new();
    }

    fn insert(conn: &Connection, username: &String, password:&String,name:&String,last_name:&String,age:&u32,birth_date:&(u8,u8,u32),department:&String,login_string:&String) -> Result<()> {
      println!("Username: {}", username);
      println!("Password: {}", password);
      println!("Name: {}", name);
      println!("Age: {}", age);
      println!("Birth Date: {}/{}/{}", birth_date.0, birth_date.1, birth_date.2);
      println!("Department: {}", department);
      println!("Mode: {}", login_string);
      conn.execute(
          "INSERT INTO login (username,password,name,last_name,age,birth_day,birth_month,birth_year,department,mode,shifthour,shiftminutes,shiftendhour,shiftendminutes,reqtime,worktime,full,notfull,notcome) 
          VALUES (?1, ?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
          params![username, password,name,last_name,age,birth_date.0,birth_date.1,birth_date.2,department,login_string,0,0,0,0,0,0,0,0,0],
      )?;
      Ok(())
  
  }

  fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?; // Column name is in the second position (index 1)
        if name == column {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Adds missing columns if they don’t exist
fn ensure_columns_exist(conn: &Connection) -> Result<()> {
    // Add columns if they do not exist
    if !Self::column_exists(conn, "login", "shifthour")? {
        conn.execute("ALTER TABLE login ADD COLUMN shifthour INTEGER;", [])?;
    }
    if !Self::column_exists(conn, "login", "shiftminutes")? {
        conn.execute("ALTER TABLE login ADD COLUMN shiftminutes INTEGER;", [])?;
    }
    if !Self::column_exists(conn, "login", "shiftendhour")? {
        conn.execute("ALTER TABLE login ADD COLUMN shiftendhour INTEGER;", [])?;
    }
    if !Self::column_exists(conn, "login", "shiftendminutes")? {
        conn.execute("ALTER TABLE login ADD COLUMN shiftendminutes INTEGER;", [])?;
    }
    if !Self::column_exists(conn, "login", "reqtime")? {
        conn.execute("ALTER TABLE login ADD COLUMN reqtime INTEGER;", [])?;
    }
    if !Self::column_exists(conn, "login", "full")? {
        conn.execute("ALTER TABLE login ADD COLUMN full INTEGER;", [])?;
    }
    if !Self::column_exists(conn, "login", "notfull")? {
        conn.execute("ALTER TABLE login ADD COLUMN notfull INTEGER;", [])?;
    }
    if !Self::column_exists(conn, "login", "notcome")? {
        conn.execute("ALTER TABLE login ADD COLUMN notcome INTEGER;", [])?;
    }

    Ok(())
}

fn insert_time(
    conn: &Connection,
    username: &String,
    shifthour: &u32,
    shiftminutes: &u32,
    shiftendhour: &u32,
    shiftendminutes: &u32,
    reqtime: &u32,
) -> Result<()> {
    // Ensure required columns exist
    Self::ensure_columns_exist(conn)?;

    // Check if the username already exists
    let mut stmt = conn.prepare("SELECT username FROM login WHERE username = ?1")?;
    let exists: Option<String> = stmt.query_row(params![username], |row| row.get(0)).optional()?;

    // Insert if the username doesn't already exist
    if exists.is_none() {
        conn.execute(
            "INSERT INTO login (username, shifthour, shiftminutes, shiftendhour, shiftendminutes, reqtime) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![username, shifthour, shiftminutes, shiftendhour, shiftendminutes, reqtime],
        )?;
        println!("Inserted shift time for user: {}", username);
    } else {
        println!("User {} already exists, not inserting shift time.", username);
    }

    Ok(())
}

fn insert_attan(
    conn: &Connection,
    username: &String,
    full: &u32,
    not_full: &u32,
    not_come: &u32,
) -> Result<()> {
    // Ensure required columns exist
    Self::ensure_columns_exist(conn)?;

    // Check if the username already exists
    let mut stmt = conn.prepare("SELECT username FROM login WHERE username = ?1")?;
    let exists: Option<String> = stmt.query_row(params![username], |row| row.get(0)).optional()?;

    // Insert if the username doesn't already exist
    if exists.is_none() {
        conn.execute(
            "INSERT INTO login (username, full, notfull, notcome) 
             VALUES (?1, ?2, ?3, ?4)",
            params![username, full, not_full, not_come],
        )?;
        println!("Inserted attendance for user: {}", username);
    } else {
        println!("User {} already exists, not inserting attendance.", username);
    }

    Ok(())
}
  

  fn get(conn: &Connection,username: &String,password:&String,mode:&String) -> Result<u8> {
    let mut stmt = conn.prepare("SELECT username,password,mode FROM login WHERE username = ?1 AND password = ?2 AND mode = ?3")?;
    match stmt.query_row(params![username,password,mode], |row| {
      let usersql:String = row.get(0)?;
      let passsql:String = row.get(1)?;
      let mode:String = row.get(2)?;

      Ok((usersql,passsql,mode))
      }) 
      {
        Ok(_) => Ok((0)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok((2)),
        Err(e) => {
          println!("{}",e);
          Ok((1))},
        

      } 
         
    
  
    }
    fn get_all(conn: &Connection,username: &String) -> Result<Option<Self>>{
      let mut stmt = conn.prepare("SELECT username,password,name,last_name,age,birth_day,birth_month,birth_year,department,mode FROM login WHERE
      username = ?1")?;
      let some = stmt.query_row(params![username], |row|{
        Ok(Self{
        username:row.get(0)?,
        password:row.get(1)?,
        name: row.get(2)?,
        last_name:row.get(3)?,
        age: row.get(4)?,
        birth_data:(row.get(5)?,row.get(6)?,row.get(7)?),
        department:row.get(8)?,
        login_string:row.get(9)?,
        ..Default::default()
  
        })
      });

      
      Ok(Some(some?))
    }
  
    fn get_all_result(conn: &Connection,role: &String,department: &String) -> Result<Vec<Self>>{

      let mut keep :Vec<Self>= Vec::new();
      let mut stmt = conn.prepare("SELECT username,password,name,last_name,age,birth_day,birth_month,birth_year,department,mode,full,notfull,notcome,shifthour,shiftminutes,shiftendhour,shiftendminutes,reqtime,worktime FROM login WHERE
       mode = ?1 AND department = ?2")?;
      let some = stmt.query_map(params![role,department], |row|{
        Ok(Self{
        username:row.get(0)?,
        password:row.get(1)?,
        name: row.get(2)?,
        last_name:row.get(3)?,
        age: row.get(4)?,
        birth_data:(row.get(5)?,row.get(6)?,row.get(7)?),
        department:row.get(8)?,
        login_string:row.get(9)?,
        attan_stat:(row.get(10)?,row.get(11)?,row.get(12)?),
        shiftstart:(row.get(12)?,row.get(13)?),
        shiftend:(row.get(14)?,row.get(15)?),
        requiretime:row.get(16)?,
        worktime:row.get(17)?,
        ..Default::default()
  
        })
      })?;
      for person in some{
        keep.push(person.unwrap());
      }

    
      
      Ok(keep)
    }

  fn calculate_age(birth_date: (u8, u8, u32)) -> u32 {
    let (birth_day, birth_month, birth_year) = birth_date;

  
    let today = Local::now().date_naive(); 
  
    let mut age = today.year() as u32 - birth_year;

  
    if (today.month() as u8, today.day() as u8) < (birth_month, birth_day) {
      age -= 1;
    }

    age
    }

    fn new_mem(&self) -> Connection{
       //memory
           let conn:Connection = Connection::open("login.db").expect("not found");
           conn.execute(
                 "CREATE TABLE IF NOT EXISTS login(
                 username TEXT NOT NULL,
                 password TEXT NOT NULL,
                 name TEXT NOT NULL,
                 last_name TEXT NOT NULL,
                 age INTEGER NOT NULL,
                 birth_day INTEGER NOT NULL,
                 birth_month INTEGER NOT NULL,
                 birth_year INTEGER NOT NULL,
                 department TEXT NOT NULL,
                 mode TEXT NOT NULL,
                 shifthour INTEGER,
                 shiftminutes INTEGER,
                 shiftendhour INTEGER,
                 shiftendminutes INTEGER,
                 reqtime INTEGER NOT NULL,
                 worktime INTEGER NOT NULL,
                 full INTEGER NOT NULL,
                 notfull INTEGER NOT NULL,
                 notcome INTEGER NOT NULL
                )",
                 []
                 ).expect("no db found");
           //Condition
          conn
    }
    
    
}
impl App for Someapp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {


          

            
            
            
            ui.add(egui::Image::new(egui::include_image!("Clove.png"))
              .max_size(Vec2::new(200.0, 300.0)));

            // login not yet
            if self.login_c == false{
              ui.horizontal(|ui|{
              if ui.put(egui::Rect::from_min_size(Pos2::new(1200.0, 10.0),Vec2::new(70.0, 20.0)),egui::Button::new("Login"),).clicked(){
                
                
                 if self.login_true == false {
                    self.login_true = true;
                 }
                 else{self.login_true = false}

                 if self.register_true == true {
                    self.register_true = false;
                    self.stat_string = String::new();
                 }
                 
                 
                
              }
              if ui.put(egui::Rect::from_min_size(Pos2::new(1275.0, 10.0),Vec2::new(100.0, 20.0)),egui::Button::new("Register"),).clicked(){
                if self.register_true == false {
                    self.register_true = true;
                 }
                 else{self.register_true = false}
                 if self.login_true == true {
                    self.login_true = false;
                    self.stat_string = String::new();
                 }
              }

            });
            }
            // login success
            if self.login_c == true{
              if let Ok(Some(user_data)) = Someapp::get_all(&self.clone().new_mem(), &self.username) {
                self.username = user_data.username;
                self.password = user_data.password;
                self.name = user_data.name;
                self.last_name = user_data.last_name;
                self.age = user_data.age;
                self.birth_data = user_data.birth_data;
                self.department = user_data.department;
                self.login_string = user_data.login_string;
                self.attan_stat = user_data.attan_stat;
                

             };
             if let Ok(Some(user_data)) = Self::get_location(&self.clone().new_mem_location(),&self.department){
              self.top.0 = user_data.top.0;
              self.top.1 = user_data.top.1;
              self.low.0 = user_data.low.0;
              self.low.1 = user_data.low.1;
             }

      

              ui.horizontal(|ui|{
                if ui.put(egui::Rect::from_min_size(Pos2::new(1200.0, 10.0),Vec2::new(70.0, 20.0)),egui::Button::new("Log Out"),).clicked(){
                  self.reset();
                  self.login_c = false;
                  
                  
                }
                ui.put(egui::Rect::from_min_max(Pos2::new(1275.0, 10.0),Pos2::new(1315.0, 45.0)),egui::Image::new(egui::include_image!("assets/user.png"))
                     .max_size(Vec2::new(35.0, 35.0)));
                
                ui.put(egui::Rect::from_min_max(Pos2::new(1310.0, 10.0),Pos2::new(1500.0, 30.0)),egui::Label::new((format!("{} {}",self.name.clone(),self.last_name.clone()))));


              });
            }
            

            //LOGIN WINDOW
            if self.login_true == true{
                egui::Window::new("").fixed_size(Vec2::new(500.0,300.0))
                .fade_in(true).fade_out(true).show(ctx, |ui| {

                  let login_t = RichText::new("Login").size(25.0).strong();
                  ui.label(login_t);      
                  egui::ComboBox::from_id_salt("Role")
                  .selected_text(&self.login_string) // Show the current selected role
                  .show_ui(ui, |ui| {
                  ui.selectable_value(&mut self.login_string, String::from("Employee"), "Employee");
                  ui.selectable_value(&mut self.login_string, String::from("Employer"), "Employer");
                  
                   });//combo box
                 if self.login_string == "Employee"{
                   ui.label("Username");
                   let mut name_emp = ui.add(egui::TextEdit::singleline(&mut self.username).hint_text("Enter some text..."));
                   ui.label("Password");
                   let mut password = ui.add(egui::TextEdit::singleline(&mut self.password).hint_text("Enter some text..."));
          
                 }
                 if self.login_string == "Employer"{
                    ui.label("Username");
                    let mut name_emp = ui.add(egui::TextEdit::singleline(&mut self.username).hint_text("Enter some text..."));
                    ui.label("Password");
                    let mut password = ui.add(egui::TextEdit::singleline(&mut self.password).hint_text("Enter some text..."));
           
                  }
                  ui.label("");
                  ui.label("");

                  if ui.button(RichText::new("LOCK IN").size(20.0).color(Color32::from_rgba_premultiplied(57, 114, 196, 1))).clicked(){
                      
                      let checkthis_log = Self::get(&self.clone().new_mem(), &self.username, &self.password,&self.login_string);
          
                      if checkthis_log == Ok(0){
                        if self.login_true == true {
                          self.login_true = false;
                          self.login_c = true;
                          println!("{}",self.login_c);
         
                       }  
                      }
                      if checkthis_log == Ok(1) {
                          self.stat_string = String::from("Error1");
                      }
                      if checkthis_log == Ok(2) {
                        self.stat_string = String::from("Username or Password is incorrect or Your assigned role not match");
                    }
                  }
                  ui.label(self.stat_string.clone());
                  


                  });//window 
            
                  

                 
            }

            //WINDOW REGISTER
            if self.register_true{
                egui::Window::new("").fixed_size(Vec2::new(500.0,300.0))
                .fade_in(true).fade_out(true).show(ctx, |ui|{

                    let login_t = RichText::new("Register").size(25.0).strong();
                    ui.label(login_t);      
                    egui::ComboBox::from_id_salt("Role")
                    .selected_text(&self.login_string) // Show the current selected role
                    .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.login_string, String::from("Employee"), "Employee");
                    ui.selectable_value(&mut self.login_string, String::from("Employer"), "Employer");
                    
                     });//combo box
                     
                   if self.login_string == "Employee"{
                     let login_t = RichText::new("Personal Data").size(20.0).strong().color(Color32::GRAY);
                     ui.label(login_t);
                     ui.label("Username");
                     let mut username_emp = ui.add(egui::TextEdit::singleline(&mut self.username).hint_text("Enter some text..."));
                     ui.label("Password");
                     let mut password = ui.add(egui::TextEdit::singleline(&mut self.password).hint_text("Enter some text..."));
                     ui.horizontal(|ui|{
                       let mut name_emp = ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
                       let mut lastname_emp = ui.add(egui::TextEdit::singleline(&mut self.last_name).hint_text("Last Name"));

                     });
                     
                     ui.horizontal(|ui|{
                       ui.label("Day");
                       ui.label("   ");
                       ui.label("Month");
                       ui.label("   ");
                       ui.label("Year")

                     });
                     ui.horizontal(|ui|{
                      egui::ComboBox::from_id_salt("Day")
                      .width(20.0)
                      .height(20.0)
                      .selected_text(&self.birth_data.0.to_string()) 
                      .show_ui(ui, |ui| {
                       for i in 1..=31{
                         ui.selectable_value(&mut self.birth_data.0, i, i.to_string());
                       } 
                        
                      });
                      egui::ComboBox::from_id_salt("Month")
                      .width(40.0)
                      .height(20.0)
                      .selected_text(&self.birth_data.1.to_string()) 
                      .show_ui(ui, |ui| {
                       for i in 1..=12{
                         ui.selectable_value(&mut self.birth_data.1, i, i.to_string());
                       } 
                        
                      });
                      egui::ComboBox::from_id_salt("Year")
                      .width(50.0)
                      .height(20.0)
                      .selected_text(&self.birth_data.2.to_string()) // Show the current selected role
                      .show_ui(ui, |ui| {
                       for i in 0..200{
                         ui.selectable_value(&mut self.birth_data.2, 2024-i, (2024-i).to_string());
                       } 
                        
                      })

                      });

                      ui.add(egui::TextEdit::singleline(&mut self.department).hint_text("Department"));

            
                   }//Employee

                   if self.login_string == "Employer"{
                    let login_t = RichText::new("Personal Data").size(20.0).strong().color(Color32::GRAY);
                    ui.label(login_t);
                    ui.label("Username");
                    let mut username_emp = ui.add(egui::TextEdit::singleline(&mut self.username).hint_text("Enter some text..."));
                    ui.label("Password");
                    let mut password = ui.add(egui::TextEdit::singleline(&mut self.password).hint_text("Enter some text..."));
                    ui.horizontal(|ui|{
                      let mut name_emp = ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Name"));
                      let mut lastname_emp = ui.add(egui::TextEdit::singleline(&mut self.last_name).hint_text("Last Name"));

                    });
                    
                    ui.horizontal(|ui|{
                      ui.label("Day");
                      ui.label("   ");
                      ui.label("Month");
                      ui.label("   ");
                      ui.label("Year")

                    });
                    ui.horizontal(|ui|{
                     egui::ComboBox::from_id_salt("Day")
                     .width(20.0)
                     .height(20.0)
                     .selected_text(&self.birth_data.0.to_string()) // Show the current selected role
                     .show_ui(ui, |ui| {
                      for i in 1..=31{
                        ui.selectable_value(&mut self.birth_data.0, i, i.to_string());
                      } 
                       
                     });
                     egui::ComboBox::from_id_salt("Month")
                     .width(40.0)
                     .height(20.0)
                     .selected_text(&self.birth_data.1.to_string()) // Show the current selected role
                     .show_ui(ui, |ui| {
                      for i in 1..=12{
                        ui.selectable_value(&mut self.birth_data.1, i, i.to_string());
                      } 
                       
                     });
                     egui::ComboBox::from_id_salt("Year")
                     .width(50.0)
                     .height(20.0)
                     .selected_text(&self.birth_data.2.to_string()) // Show the current selected role
                     .show_ui(ui, |ui| {
                      for i in 0..200{
                        ui.selectable_value(&mut self.birth_data.2, 2024-i, (2024-i).to_string());
                      } 
                       
                     })

                     });

                     ui.add(egui::TextEdit::singleline(&mut self.department).hint_text("Department"));

             
                    }//Employer

                    ui.label("");
                    ui.label("");
  
                    if ui.button(RichText::new("REGISMA").size(20.0).color(Color32::from_rgba_premultiplied(57, 114, 196, 1))).clicked(){

                        let checkthis_regis = Self::insert(&self.clone().new_mem(),&self.username,&self.password,&self.name,&self.last_name,&Someapp::calculate_age(self.birth_data),&self.birth_data,&self.department,&self.login_string);
                        match checkthis_regis {
                            Ok(_) =>{
                              if self.login_true == false {
                            self.login_true = true;
                         }
        
                          if self.register_true == true { 
                            self.register_true = false;
                         }
                           
                            }
                            Err(e) => self.stat_string = e.to_string(),
                        };
                        
                    }
                    ui.label(self.stat_string.clone())
                    
                });
            }
        
            ui.add(egui::Separator::default());
            // 0 is empty, 1 is employees attandance page, 2 is employees profiles, 3 is employees profiles for each person
            //,4 is first page of attandances for location of dapartment ,5 is list of the attandances of employee, 6 is the time set for employer 
            
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Employees Attandance")).clicked(){
                  self.ui_page = 1;
                };
                if ui.add(egui::Button::new("Employees/Employers Profiles")).clicked(){
                  self.ui_page = 2;
                }

            });
            ui.add(egui::Separator::default());
            //window under seperator
            if self.login_c == true{
                  
                  self.timelocate(self.top.0,self.top.1,self.low.0,self.low.1); 
                  println!("{}",self.top.0);
                  
                  
                  if self.top.0 == 0.0 && self.top.1 == 0.0 && self.low.0 == 0.0 && self.low.1 == 0.0 {
                    
                    
                  }

                  if self.ui_page == 1{
                   
                   if self.top.0 == 0.0 || self.top.1 == 0.0|| self.low.0 == 0.0 || self.low.1 == 0.0 {
                     self.ui_page_attan = 1;
                     self.show(ctx,ui);
                     
                   }
                   
                   else{ 
                    if self.ui_page_attan !=3{
                    println!("some");
                    self.ui_page_attan = 2;
                    self.show(ctx,ui);
                  }else {
                      self.show(ctx,ui);
                  }
                  
                  }
                   
                   
                
                  
                }
        

                let employers_vec:Vec<Self> = Someapp::get_all_result(&self.clone().new_mem(),&String::from("Employer"),&self.department).expect("");
                let employees_vec:Vec<Self> = Someapp::get_all_result(&self.clone().new_mem(),&String::from("Employee"),&self.department).expect("");
                
                if self.ui_page == 2{
                  self.stat_keep.clear();
                  if self.login_c == true{
                        if self.login_string == "Employer"{
                          ui.label(RichText::new("----------Employer----------").size(20.5));
                        
                          for person in employers_vec.clone(){
                            if ui.add_sized([700.0, 30.0],egui::Button::new(RichText::new(format!("{} {}",person.name.clone(),person.last_name.clone())).size(15.0))).clicked(){
                              self.ui_page += 1;
                              self.stat_keep.push(person);
                            }
                          }
                          ui.label(RichText::new("----------Employee----------").size(20.5));
                          
      
                          for person in employees_vec.clone(){
                            
                            if ui.add_sized([700.0, 30.0],egui::Button::new(RichText::new(format!("{} {}",person.name.clone(),person.last_name.clone())).size(15.0))).clicked(){
                              self.ui_page += 1;
                              self.stat_keep.push(person);
                            }
                          }

                        }

                        if self.login_string == "Employee"{
                          ui.label(RichText::new("----------Employer----------").size(20.5));

                          
                          for person in employers_vec.clone(){
                            if ui.add_sized([700.0, 30.0],egui::Button::new(RichText::new(format!("{} {}",person.name.clone(),person.last_name.clone())).size(15.0))).clicked(){
                              self.ui_page += 1;
                              self.stat_keep.push(person);
                            }
                          }
                          ui.label(RichText::new("----------Employee----------").size(20.5));
                          
      
                          for person in employees_vec.clone(){
                            
                            if ui.add_sized([700.0, 30.0],egui::Button::new(RichText::new(format!("{} {}",person.name.clone(),person.last_name.clone())).size(15.0))).clicked(){
                              self.ui_page += 1;
                              self.stat_keep.push(person);
                              
                                
                            }
                          }
                        }
                      }
                }
                if self.ui_page == 3{
                for person in self.stat_keep.clone(){
                  ui.label(RichText::new(format!("Username: {}",person.username)).size(15.0));
                  ui.label(RichText::new(format!("Name: {}",person.name)).size(15.0));
                  ui.label(RichText::new(format!("Lastname: {}",person.last_name)).size(15.0));
                  ui.label(RichText::new(format!("Age: {}",person.age)).size(15.0));
                  ui.label(RichText::new(format!("Birthday : {}/{}/{}",person.birth_data.0,person.birth_data.1,person.birth_data.2)).size(15.0));
                  ui.label(RichText::new(format!("Department : {}",person.department)).size(15.0));
                  ui.label(RichText::new(format!("Role : {}",person.login_string)).size(15.0));
                  ui.label(RichText::new(format!("Fully Attandance : {}",person.attan_stat.0)).size(15.0));
                  ui.label(RichText::new(format!("Not Fully Attandance : {}",person.attan_stat.1)).size(15.0));
                  ui.label(RichText::new(format!("Absences : {}",person.attan_stat.2)).size(15.0));
                }
                  
                }
            }//check login true 
            

            
            if self.login_c == false{
              let login_text = RichText::new("You need to Login First").size(25.0).strong().color(Color32::RED).underline();
              ui.put(egui::Rect::from_min_size(Pos2::new(650.0, 350.0),Vec2::new(200.0, 300.0)),egui::Label::new(login_text),);
            }
            if self.login_c == true{
                 if self.login_string == "Employer"{

                 }
                 
            }

            
            
        });
    }
    
    

}
