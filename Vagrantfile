Vagrant.configure("2") do |config|

  config.vm.box = "archlinux/archlinux"
  config.vm.synced_folder ".", "/vagrant", disabled: true

  config.ssh.insert_key = true
  config.ssh.forward_agent = true
  # config.ssh.private_key_path = "/home/jakob/.ssh/id_rsa"
  # config.ssh.username = "vagrant"
  # config.ssh.password = "vagrant"


  config.vm.provider :virtualbox do |v|
    v.memory = 2048
    v.cpus = 2
    v.linked_clone = true
  end

  config.vm.define "logs" do |app|
    app.vm.hostname = "logs.test"
    app.vm.network :private_network, ip: "192.168.56.10"

    app.vm.provider :virtualbox do |vm|
      vm.memory = 8192
      vm.cpus = 4
    end

    app.vm.provision :ansible do |ansible|
      # ansible.host_key_checking = false
      ansible.config_file = "ansible/ansible.cfg"
      ansible.playbook = "ansible/provisioning/elk/main.yml"
      ansible.inventory_path = "ansible/inventory_vagrant.py"
    end
  end

  config.vm.define "app" do |app|
    app.vm.hostname = "app1.test"
    app.vm.network :private_network, ip: "192.168.56.11"

    app.vm.provision :ansible do |ansible|
      # ansible.host_key_checking = false
      ansible.config_file = "ansible/ansible.cfg"
      ansible.playbook = "ansible/provisioning/app/main.yml"
      ansible.inventory_path = "ansible/inventory_vagrant.py"
    end
  end

end
