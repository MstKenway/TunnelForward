wget -O /usr/bin/TunnelForward http://103.136.187.104/TunnelForward
chmod +x /usr/bin/TunnelForward


cat >/etc/systemd/system/TunnelForward.service << EOF
[Unit]
Description=TunnelForward - Forwarding traffic
After=network.target nss-lookup.target network-online.target
Wants=network-online.target
  
[Service]
Type=simple
PIDFile=/run/TunnelForward.pid
ExecStart=/usr/bin/TunnelForward -f /etc/TunnelForward/config.json >/dev/null &2>1
PrivateTmp=true
  
[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable TunnelForward
