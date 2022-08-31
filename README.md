# Sebastian 
> **se·bas·tian** - *sɪˈbæstɪən*  
> A simple tool used to access UniMi services -- mainly
> [ariel](https://ariel.unimi.it/), but not only -- via CLI.

### Important: state
First of all, thank you for your interest. If you have a feature you have in
mind please create a new issue or, if you have some time on your hands, file a
PR. 

## Usage
The usage is pretty straighforward. The CLI is made out of root commands and
subcommands: 
``` bash
app
├── ariel       Ariel
│   ├── init         Initialize your configuration.
│   ├── scrape       Perform scraping on some ariel site.
│   └── search       Search info about a site. 
├── time        Timetable
│   ├── init         Initialize your configuration.
│   └── show         Show your timetable. 
└── unimia      Unimia
    ├── init         Initialize your configuration.
    └── show         Show your unimia status. 
```
