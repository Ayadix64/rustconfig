# rustconfig
yep, 

i am curuntly learning rust by doing a small project , and i neded a config language, so why dosnt made one?

style

``` rust

configctx.OpenConfig("config");
for i in &configctx.configs {
    println!("{} : {}",i[0],i[1]);
}
println!("***********************************************");
for i in &configctx.commits {
    println!("{} , {}",i.0,i.2);
}
//you can do that, or

configctx.GetConfig("a_config_of_culture");
configctx.SetConfig("a_config_of_culture","true");

let comment = configctx.GetComment(2); //ge the comment frome line 2
comment.push_str(" , shipe!");
configctx.SetComment(comment.as_str(),5,2); //spaces from the last charctare in config
configctx.SaveConfig("config2");

```


the config file will looks somthing like this

``` config 
isAHuman: "may be" #hmmmm
isAAnimale: "will that is a good quation"
HaveYouEverherdofit: "Nope" 
# no tbh
isaSHIPE:"YES"
```

be aware that this code is writen to learn rust, but it is not that bad of code
