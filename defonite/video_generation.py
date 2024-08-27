# it will load a json data with an array of objects which will be configurations for the api call
import json

json_path = ".temp/scene_configs.json"
scene_configs = []
with open(json_path) as json_file:
    scene_configs = json.load(json_file)
    
import requests

def call_t2v_api(config):
    url = "http://127.0.0.1:7860/t2v/run"

    headers = {
        "Content-Type": "application/json"
    }

    response = requests.post(url, headers=headers,  params=config)

    if response.status_code == 200:
        print("API call successful")
        # Process the response as needed
        response_data = response.json()
        print(response_data)
    else:
        print("API call failed with status code:", response.status_code)
        print(response.text)

# Call the API
for config in scene_configs:
    print(config)
    call_t2v_api(config)
    break
