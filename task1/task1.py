import yaml

def load_family_data(filename):
    with open(filename, 'r', encoding='utf-8') as file:
        data = yaml.safe_load(file)
    return data['people'], data['families']

def build_family_tree(people, families):
    tree = {name: {'gender': info['gender'], 'spouse': None, 'children': [], 'parents': []} for name, info in people.items()}
    for family in families:
        husband, wife = family['parents']
        children = family.get('children', [])
        tree[husband]['spouse'] = wife
        tree[wife]['spouse'] = husband
        for child in children:
            tree[child]['parents'] = [husband, wife]
            tree[husband]['children'].append(child)
            tree[wife]['children'].append(child)
    return tree

def find_relatives(name, family_tree):
    if name not in family_tree:
        return "Имя не содержится в исходном списке родственников"
    
    relatives = {}
    person = family_tree[name]
    
    if person.get('spouse'):  
        relatives['Супруг(а)'] = person['spouse']
    
    parents = person.get('parents', []) 
    if parents:
        relatives['Отец и мать'] = parents
        
        siblings = []
        for parent in parents:
            if parent in family_tree:
                parent_children = family_tree[parent].get('children', [])  
                for child in parent_children:
                    if child != name and child not in siblings:
                        siblings.append(child)
        if siblings:
            relatives['Братья и сестры'] = siblings
        
        grandparents = []
        for parent in parents:
            if parent in family_tree:
                grandparents += family_tree[parent].get('parents', [])  
        if grandparents:
            relatives['Дедушка и бабушка'] = list(set(grandparents))
    
    if person.get('children'): 
        relatives['Дети'] = person['children']

    return relatives

def print_relatives(relatives, family_tree):
    for relation, names in relatives.items():
        print(f"{relation}: ", end='')
        if isinstance(names, list):
            for name in names:
                gender = 'м' if family_tree[name]['gender'] == 'm' else 'ж'  
                print(f"{name} ({gender})", end=', ')
        else:
            gender = 'м' if family_tree[names]['gender'] == 'm' else 'ж'
            print(f"{names} ({gender})", end=', ')
        print()

def main():
    people, families = load_family_data('input.yml')
    family_tree = build_family_tree(people, families)
    
    while True:
        name = input("Введите имя для поиска родственников (или введите 'выход' для завершения): ")
        if name.lower() == 'выход':
            break
        relatives = find_relatives(name, family_tree)
        if isinstance(relatives, str):
            print(relatives)
        else:
            print(f"{name} ({'м' if family_tree[name]['gender'] == 'm' else 'ж'}) имеет следующих родственников:")
            print_relatives(relatives, family_tree)

if __name__ == "__main__": 
    main()