import yaml
from yaml.loader import SafeLoader
# Открываем файл
with open('test.yaml') as f:
    # читаем документ YAML
    data = yaml.load(f, Loader=SafeLoader)
    print(data)
#https://docs-python.ru/packages/modul-pyyaml-python/zagruzka-chtenie-dokumenta-yaml/
# {'UserName': 'Alicia', 'Password': 'pinga123*', 
# 'phone': '(495) 555-32-56', 'room': 10, 
# 'TablesList': ['EmployeeTable', 'SoftwaresList', 'HardwareList']}