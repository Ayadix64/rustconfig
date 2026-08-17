use std::cmp::Ordering;
use std::str::Lines;
use std::{self, cmp, default, ptr};
use std::io::{self, Error, ErrorKind, Write, stdout};
use std::ptr::addr_of;
use std::{thread,time};
use std::fs;
use std::io::prelude::*;
use std::path::Path;



//#[derive(Debug, Clone, PartialEq)]
pub struct ConfigContext 
{
   pub configs: Vec<[String;2]>, 
   pub commits: Vec<(String , u32 /*space from commit*/ , u32 /*linenum*/)>,
   pub confile:  String
    
}


pub enum ConfigErr {
    Ok,
    FileOpenErr,
    FileReadingErr,
    FileWritingErr,
    SyntaxtErr,
    ConfigNotFound,
    CommitNotFound
}

impl ConfigErr {
    fn as_str(&self) -> &str
    {
        return match self {
            ConfigErr::Ok => "Status OK.",
            ConfigErr::FileOpenErr => "Can Not Open File",
            ConfigErr::FileReadingErr => "Can Not Read File",
            ConfigErr::FileWritingErr => "Can Not Write File",
            ConfigErr::SyntaxtErr => "Config syntax Erorr",
            ConfigErr::ConfigNotFound => "Config not found",
            ConfigErr::CommitNotFound => "Commit Not Found"
        }
    }
}

impl ConfigContext {

    pub fn new()->Self{
        Self{configs:Vec::new(),commits: Vec::new(),confile:String::new()}
    }


    pub fn OpenConfig(&mut self,file:&str) -> ConfigErr
    {
        
        let mut configfile = match fs::File::open(file) {
            Ok(file)=>file , 
            Err(erorr) => {println!("[Erorr] , can-not open file \"{file}\" , {erorr:?}."); return ConfigErr::FileOpenErr;}
        };

        let mut fldata : String = String::new();
        if let error = configfile.read_to_string(&mut fldata).err() && error.is_some(){
            println!("[Erorr] at reading {file} , {error:?}");
            return ConfigErr::FileReadingErr;
        };
        self.confile = String::from(file); 
        self.configs.clear();
        self.commits.clear();

        let mut syntaxterorrdetacted:bool = false;
        let mut linenum     :u32 = 0;

        for   l in fldata.lines(){
            let mut conf   = String::new();
            let mut value  = String::new();
            let mut commit = String::new();



            let mut add2conf    :bool = true;
            let mut add2value   :bool = false;
            

            let mut commitspaces:u32 = 0;
            let mut narrow      :u32 = 0;
            
            let mut discovererorr:bool = false;
            

            for i in 0..l.as_bytes().len()
            {
                if l.as_bytes()[i] == '#' as u8  && (narrow == 0 || narrow >= 2) {
                    for j in i+1..l.as_bytes().len()  //push evry thing in the comment, and beeak
                    {
                        commit.push(l.as_bytes()[j] as char);
                    }
                    break;
                }
                if l.as_bytes()[i] == ':' as u8 {
                    if !add2value {
                        add2conf=false;
                        add2value = true;
                        continue;
                    }else {eprintln!("[CONFIG ERORR] {file} line : {linenum}, {i}, syntax erorr, un-expected \" : \".");discovererorr=true;break;}
                }
                else if l.as_bytes()[i] == '"' as u8 {
                    narrow+=1;
                    continue;
                }

                 

                if l.as_bytes()[i] == ' ' as u8{
                    commitspaces+=1;
                }else {
                    commitspaces=0;
                    if add2conf && l.as_bytes()[i] != ':' as u8
                    {
                        conf.push(l.as_bytes()[i] as char);
                        commitspaces=0;

                    }
                    else if add2conf &&
                        conf.len()>0 &&
                            i>0 && l.as_bytes()[i-1] == ' ' as u8 && 
                            l.as_bytes()[i]!=':' as u8
                            { //aka; a space in the middel of config; erorr
                                eprintln!("[CONFIG ERORR] {file} line : {linenum}, {i}, syntax erorr, un-expected space.");
                                discovererorr=true;
                                break;
                    }

                }
                               
                if add2value && narrow==1  && l.as_bytes()[i] != ':' as u8
                {
                    value.push(l.as_bytes()[i] as char);
                }else if add2value && l.as_bytes()[i]!=' ' as u8{ //aka; a space in the middel of config; erorr
                    eprintln!("[CONFIG ERORR] {file} line : {linenum}, {i}, syntax erorr, expect \" found {} ", l.as_bytes()[i] as char);
                    discovererorr=true;
                    break;

                }
            }
            if narrow!=2 && narrow==1 {
                eprintln!("[CONFIG ERORR] {file} line : {linenum}, {}, syntax erorr, expect '\"' found  '{}' .",l.len() , l.as_bytes()[l.len()-1] as char);
                discovererorr=true;
            }
            if !discovererorr && conf.len() >0 {
                self.configs.push( [conf.clone(), value.clone()] );
            }
            if commit.len() > 0{
                self.commits.push((commit.clone() , commitspaces , linenum));
            }
            syntaxterorrdetacted=syntaxterorrdetacted|discovererorr;
           
            linenum+=1;
        }
        if syntaxterorrdetacted {return ConfigErr::SyntaxtErr;} 
        return ConfigErr::Ok;
    }



    pub fn GetConfig(&mut self,conf:&str) -> Result<String,ConfigErr>
    {
        let mut ret :String = String::new();
        let mut found:bool = false;
        for i in 0..self.configs.len() {
            if conf.cmp(self.configs[i][0].as_str()) == Ordering::Equal
            {
                ret = self.configs[i][1].clone();
                found=true;
            }

        }
        if !found
        {
            eprintln!("[ERORR] never found the config {conf}");
            return Err(ConfigErr::ConfigNotFound);
        }
        Ok(ret)
    }
    pub fn SetConfig(&mut self,config:&str,val:&str)
    {
        if config.len() == 0 {
            eprintln!("[ERORR] seting a empty config name.");
            return;
        }
        let mut found:bool = false;
        for i in 0..self.configs.len() {
            if val.cmp(self.configs[i][0].as_str()) == Ordering::Equal
            {
                self.configs[i][1] = val.to_string(); 
                found=true;
            }

        }
        if !found
        {
            let mut con = String::from(config);
            let mut va = String::from(val);
            self.configs.push([con.clone(),va.clone()]);

        }
        return;
    }
    


    pub fn GetComment(&mut self,line:u32) -> Result<String,ConfigErr>
    {
        let mut ret =String::new();
        let mut found:bool = false;
        for i in 0..self.commits.len() {
            if self.commits[i].2 == line{
                ret = self.commits[i].0.clone(); 
                found=true;
            }
        }
        if !found
        {
            eprintln!("[ERORR] never found commint in the line {line}");
            return Err(ConfigErr::CommitNotFound);
        }
        return  Ok(ret);
    }
    pub fn SetComment(&mut self,commit:&str,line:u32 , spaces:u32)
    {
        let mut found:bool = false;
        let mut inserloc: usize = 0;
        for i in 0..self.commits.len() {
            if self.commits[i].2 == line{
                self.commits[i].0 = String::from(commit);
                self.commits[i].1 = spaces;
                found=true;
            }else if self.commits[i].2 > line {
                inserloc = i;
                break;
            }

        }
        if !found
        {
            let mut com :(String , u32 , u32) = (String::from(commit),spaces,line);
            self.commits.insert(inserloc,com);

        }
        return;
    }




    pub fn SaveConfig(&mut self,cfile:&str)->ConfigErr
    {
        let mut configfile = match fs::File::open(cfile) {
            Ok(file1)=>file1 ,
            Err(_) => match fs::File::create(cfile) {
                Ok(file2) =>file2,
                Err(erorr) => {println!("[ERORR] , can-not creat file \"{cfile}\" , {erorr:?}."); return ConfigErr::FileOpenErr;}
            }
            
        };

        
        let mut floutdata : String = String::new();

        let mut commit: usize = 0;
        

        for i in 0..self.configs.len(){
            
            floutdata.push_str(self.configs[i][0].as_str());
            floutdata.push_str(":\"");
            floutdata.push_str(self.configs[i][1].as_str());
            floutdata.push('"');
            if commit < self.commits.len() && self.commits[commit].2 == i as u32 // if they are commits  and the curent commit is
                                                                           // than it, and its number is the number of the
                                                                           // line, insert it
            {
                for _ in 0..self.commits[commit].1 {
                    floutdata.push(' ');
                }
                floutdata.push('#');
                floutdata.push_str(self.commits[commit].0.as_str());
                commit+=1;
            }

            floutdata.push('\n');
            
        }
        if self.commits.len() > 0  {
            for i in self.configs.len() as u32 ..self.commits[self.commits.len()-1].2+1  {
                if commit < self.commits.len() && self.commits[commit].2 == i as u32 {
                    for _ in 0..self.commits[commit].1 {
                        floutdata.push(' ');
                    }
                    floutdata.push('#');
                    floutdata.push_str(self.commits[commit].0.as_str());
                    commit+=1;
                }

                floutdata.push('\n');
            }
        } 

        return  match fs::write(cfile,floutdata.as_bytes()){
            Ok(_) =>ConfigErr::Ok,
            Err(err) => {eprintln!("[ERORR] writing file \"{cfile}\" , {err:?} .");ConfigErr::FileWritingErr}
        }
        
    }
    
}




/* MIT License                                                                       *
 *
 * Copyright (c) 2026 Ayadi                                                          *
 
 * Permission is hereby granted, free of charge, to any person obtaining a copy      *
 * of this software and associated documentation files (the "Software"), to deal     *
 * in the Software without restriction, including without limitation the rights      *
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell         *
 * copies of the Software, and to permit persons to whom the Software is             *
 * furnished to do so, subject to the following conditions:                          *

 * The above copyright notice and this permission notice shall be included in all    *
 * copies or substantial portions of the Software.                                   *

 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR        *
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,          *
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE       *
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER            *
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,     *
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE     *
 * SOFTWARE.                                                                         */
