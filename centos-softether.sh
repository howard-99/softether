yum -y update
yum -y install epel-release wget nano htop
yum -y groupinstall "Development Tools"
cd /usr/local/
wget https://github.com/SoftEtherVPN/SoftEtherVPN_Stable/releases/download/v4.41-9782-beta/softether-vpnserver-v4.41-9782-beta-2022.11.17-linux-x64-64bit.tar.gz

tar xvf softether-vpnserver-*
cd vpnserver && make

cd /etc/init.d/
wget https://raw.githubusercontent.com/howard-99/softether/main/vpnserver

chmod 755 /etc/init.d/vpnserver
/etc/init.d/vpnserver start
chkconfig --add vpnserver 

yum -y update

clear
echo "SoftEther v4.41-9782-beta successfully installad."

cd /usr/local/vpnserver/
./vpncmd
