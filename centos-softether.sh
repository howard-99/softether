yum -y update
yum -y install epel-release wget nano htop
yum -y groupinstall "Development Tools"
cd /usr/local/
wget https://github.com/SoftEtherVPN/SoftEtherVPN_Stable/releases/download/v4.41-9782-beta/softether-vpnserver-v4.41-9782-beta-2022.11.17-linux-x64-64bit.tar.gz

tar xvf softether-vpnserver-*
cd vpnserver && make

mkdir /etc/init.d/

cd /etc/init.d/
wget https://raw.githubusercontent.com/howard-99/softether/main/vpnserver

chmod 755 /etc/init.d/vpnserver
/etc/init.d/vpnserver start
chkconfig --add vpnserver 

#Firewalld
sudo yum install firewalld -y
sudo systemctl start firewalld
sudo systemctl enable firewalld

sudo firewall-cmd --zone=public --add-port=22/tcp
sudo firewall-cmd --zone=public --add-port=443/tcp
sudo firewall-cmd --zone=public --add-port=80/tcp
sudo firewall-cmd --zone=public --add-port=992/tcp
sudo firewall-cmd --zone=public --add-port=1194/tcp
sudo firewall-cmd --zone=public --add-port=5555/tcp
sudo firewall-cmd --zone=public --add-port=4500/tcp
sudo firewall-cmd --zone=public --add-port=1701/tcp
sudo firewall-cmd --zone=public --add-port=500/tcp
sudo firewall-cmd --zone=public --add-port=50740/tcp
sudo firewall-cmd --zone=public --add-port=8443/tcp
sudo firewall-cmd --zone=public --add-port=9699/tcp
sudo firewall-cmd --zone=public --add-port=2083/tcp
sudo firewall-cmd --zone=public --add-port=8080/tcp
sudo firewall-cmd --zone=public --add-port=443/udp
sudo firewall-cmd --zone=public --add-port=500/udp
sudo firewall-cmd --zone=public --add-port=4500/udp
sudo firewall-cmd --zone=public --add-port=1194/udp



yum -y update

clear
echo "SoftEther v4.41-9782-beta successfully installad."

cd /usr/local/vpnserver/
./vpncmd
