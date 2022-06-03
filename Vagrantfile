Vagrant.configure("2") do |config|

  config.vm.box = "archlinux/archlinux"
  config.ssh.insert_key = false

  config.vm.provider :virtualbox do |v|
    v.memory = 4096
    v.cpus = 4
    v.linked_clone = true
  end

  config.vm.define "logs" do |app|
    app.vm.hostname = "logs.test"
    app.vm.network :private_network, ip: "192.168.56.10"

    app.vm.provider :virtualbox do |vm|
      vm.memory = 8192
      vm.cpus = 4
    end

    app.vm.provision :ansible do |ansible |
      ansible.config_file = "ansible/ansible.cfg"
      ansible.playbook = "ansible/provisioning/elk/main.yml"
      ansible.inventory_path = "ansible/provisioning/elk/inventory/hosts.ini"
      ansible.become = true
    end
  end
end