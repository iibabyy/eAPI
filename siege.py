import requests
import json
import threading

# URL et données de la requête
REGISTER_URL = "http://localhost:8080/api/auth/register"
LOGIN_URL = "http://localhost:8080/api/auth/login"
HEADERS = {"Content-Type": "application/json"}

def data(i):
    data = {
        "name": "ibaby",
        "email": f"{i}@gmail.com",
        "password": "password",
        "passwordConfirm": "password"
    }
    return data

# Fonction pour envoyer une requête POST
def send_request(i):
    try:
        _data = data(i)
        response1 = requests.post(REGISTER_URL, data=json.dumps(_data),  headers=HEADERS)
        response = requests.post(LOGIN_URL, data=json.dumps(_data), headers=HEADERS)
        print(f"register: {response1.status_code} | login: {response.status_code}")
    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")

# Lancer 100 requêtes simultanément
if __name__ == "__main__":
    threads = []
    for i in range(100):
        thread = threading.Thread(target=send_request, args=(i, ))
        threads.append(thread)
        thread.start()
    
    # Attendre la fin de toutes les requêtes
    for thread in threads:
        thread.join()
