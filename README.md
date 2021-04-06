# TunnelForward

Usage:
```
wget https://raw.githubusercontent.com/MstKenway/TunnelForward/master/install_go.sh
chmod +x install_go.sh
bash install_go.sh
```

---

··· 
Usage of TunnelForward:
  -a	Append the following config.json to the config.json file.
  -c	Server mode of the tunnel.(-tun need)
  -d int
    	Delete the local port in the config.json file. (default -1)
  -f string
    	Path for config.json file. (default "/etc/TunnelForward/config.json")
  -k string
    	Key of the tunnel.(-tun need)
  -l	List all configs in the config.json file.
  -r string
    	Single Relay config.json "LocalPort:RemoteAddr:RemotePort"
  -s	Client mode of the tunnel.(-tun need)
  -ttimeout int
    	TCP Timeout(s)
  -tun string
    	Single Tunnel config.json "LocalPort:RemoteAddr:RemotePort"
  -utimeout int
    	UDP Timeout(s) (default 60)
  -v	Print current version.
··· 
