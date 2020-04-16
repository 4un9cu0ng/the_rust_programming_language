## Linux
### The installation of the ```rustup``` tool 
```curl https://sh.rustup.rs -sSf | sh```

### setup PATH
``` source $HOME/.cargo/env ```

Alternatively, you can add the following line to your ```~/.bash_profile```:
``` export PATH="$HOME/.cargo/bin:$PATH"```

## Updating and Uninstalling
### Update
``` rustup update ```

### Uninstall
``` rustup self uninstall```

## Troubleshooting
### Check version
``` rustc --version ```


