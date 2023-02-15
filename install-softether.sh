#**Installation of SoftEther v4.41-9782-beta for CentOS7&8
#**The Below line could change the mirror of centos7 Only
#** way to install =>
# bash <(curl -Ls https://raw.githubusercontent.com/howard-99/softether/main/install-softether.sh)

#bash <(curl -Ls https://raw.githubusercontent.com/howard-99/ir-mirror/main/centos7.sh)
sudo systemctl stop firewalld
sudo systemctl disable firewalld

sudo yum install -y epel-release 
sudo yum install -y vim wget unzip yum-utils htop nano certbot
sudo yum group install -y "Development Tools" 
sudo yum install yum-cron -y

sudo sed -i 's,^update_cmd =.*$,update_cmd = default,' /etc/yum/yum-cron.conf
sudo sed -i 's,^apply_updates =.*$,apply_updates = yes,' /etc/yum/yum-cron.conf

grep -qxF 'exclude = kernel*' /etc/yum/yum-cron.conf || echo -e "\n# Exclude Kernel Upgrades \nexclude = kernel*" | sudo tee -a /etc/yum/yum-cron.conf

sudo systemctl start yum-cron
sudo systemctl enable yum-cron

sudo wget -O /tmp/softether-vpnserver.tar.gz https://github.com/SoftEtherVPN/SoftEtherVPN_Stable/releases/download/v4.41-9782-beta/softether-vpnserver-v4.41-9782-beta-2022.11.17-linux-x64-64bit.tar.gz
sudo tar xfz /tmp/softether-vpnserver.tar.gz -C /usr/local
cd /usr/local/vpnserver
make

sudo useradd --system --no-create-home softether
sudo chown -R softether:softether /usr/local/vpnserver
sudo find /usr/local/vpnserver -type f -exec chmod 600 {} \;
sudo find /usr/local/vpnserver -type d -exec chmod 700 {} \;
sudo chmod +x /usr/local/vpnserver/vpncmd 
sudo chmod +x /usr/local/vpnserver/vpnserver

wget -O softether.service https://raw.githubusercontent.com/howard-99/softether/main/softether.service

sudo setcap CAP_NET_BIND_SERVICE=+eip /usr/local/vpnserver/vpnserver
sudo systemctl daemon-reload
sudo systemctl start softether.service
sudo systemctl enable softether.service
sudo rm -f /tmp/softether-vpnserver.tar.gz 

clear

echo "SoftEther v4.41-9782-beta successfully installad."

#https://blog.yasithab.com/centos/softether-vpn-on-centos-7/
#https://www.vpsbg.eu/docs/how-to-setup-a-softether-vpn-server-on-centos-7
