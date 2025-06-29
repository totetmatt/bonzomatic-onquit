# Bonzomatic Onquit
Small companion tool for Bonzomatic that is aimed to be run after quiting bonzomatic to manage the shader file used.

## Install
* Download zip
* Unzip content to bonzomatic directory
* Change `config.json` to have `postExitCmd` config key set to `bonquit.bat` :
      
```json
{
 [...]   
"postExitCmd": "bonquit.bat"
 [...]
}
```

## User Guide

* **Do Nothing** : Status quo behavior. Nothing will be done as if there was no `postExitCmd` on the `config.json`. The shader file will stay and won't be touched.
* **Delete Shader** :  It will delete the shader file that was just used
* **Save Shader to File** : Will move the shader file to a location that you will specify
* **Move Shader to Directory** : Will move the shader to a specified directory. It will change file name to `timestamp_<name_of_origina_file.glsl>`
** By default, the directory will be `.\shaders`. It can be changed by updating the `bonquit.bat` and change the latest parameter of the invokation to the directory that you want 
*** `.\bonzomatic-onquit.exe %1 .\whatever_you_want`